//! Vello-based viewer widget for GPU-accelerated vector rendering.
//!
//! This module provides a viewer widget that uses Vello for GPU rendering
//! and displays the result in egui.
//!
//! Due to wgpu version differences between egui-wgpu and vello, we use a
//! texture-copy approach: Vello renders to its own GPU texture, copies to
//! a CPU buffer, then uploads to egui as a texture.

use eframe::egui::{self, ColorImage, TextureHandle, TextureOptions};
use nodebox_core::geometry::{Color, Path};
use vello::kurbo::Affine;
use vello::peniko::Color as PenikoColor;
use vello::wgpu;
use vello::{AaConfig, RenderParams, Renderer, RendererOptions, Scene};

use crate::vello_convert::convert_paths;

/// Error type for Vello viewer operations.
#[derive(Debug)]
pub enum VelloViewerError {
    /// Failed to create wgpu adapter.
    AdapterCreation,
    /// Failed to create wgpu device.
    DeviceCreation(String),
    /// Failed to create Vello renderer.
    RendererCreation(String),
    /// Failed to render.
    RenderFailed(String),
    /// Failed to copy texture to buffer.
    BufferCopyFailed(String),
}

impl std::fmt::Display for VelloViewerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VelloViewerError::AdapterCreation => write!(f, "Failed to create wgpu adapter"),
            VelloViewerError::DeviceCreation(e) => write!(f, "Failed to create device: {}", e),
            VelloViewerError::RendererCreation(e) => write!(f, "Failed to create renderer: {}", e),
            VelloViewerError::RenderFailed(e) => write!(f, "Render failed: {}", e),
            VelloViewerError::BufferCopyFailed(e) => write!(f, "Buffer copy failed: {}", e),
        }
    }
}

impl std::error::Error for VelloViewerError {}

/// GPU context for Vello rendering.
///
/// This maintains its own wgpu instance separate from egui's to avoid
/// version conflicts.
struct VelloGpuContext {
    device: wgpu::Device,
    queue: wgpu::Queue,
    renderer: Renderer,
}

impl VelloGpuContext {
    /// Create a new GPU context for Vello.
    fn new() -> Result<Self, VelloViewerError> {
        // Create wgpu instance
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // Request adapter
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        }))
        .map_err(|_| VelloViewerError::AdapterCreation)?;

        // Request device
        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("vello_device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
                ..Default::default()
            },
        ))
        .map_err(|e| VelloViewerError::DeviceCreation(e.to_string()))?;

        // Create Vello renderer
        let renderer = Renderer::new(
            &device,
            RendererOptions {
                ..Default::default()
            },
        )
        .map_err(|e| VelloViewerError::RendererCreation(format!("{:?}", e)))?;

        Ok(VelloGpuContext {
            device,
            queue,
            renderer,
        })
    }
}

/// Vello-based viewer for GPU-accelerated vector rendering.
pub struct VelloViewer {
    /// GPU context (lazily initialized).
    gpu_ctx: Option<VelloGpuContext>,
    /// Cached egui texture for display.
    texture: Option<TextureHandle>,
    /// Current render dimensions.
    width: u32,
    height: u32,
    /// Background color.
    background_color: Color,
    /// Whether initialization failed.
    init_failed: bool,
    /// Error message if initialization failed.
    init_error: Option<String>,
}

impl Default for VelloViewer {
    fn default() -> Self {
        Self::new()
    }
}

impl VelloViewer {
    /// Create a new Vello viewer.
    pub fn new() -> Self {
        VelloViewer {
            gpu_ctx: None,
            texture: None,
            width: 0,
            height: 0,
            background_color: Color::WHITE,
            init_failed: false,
            init_error: None,
        }
    }

    /// Set the background color.
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    /// Initialize the GPU context if not already done.
    fn ensure_initialized(&mut self) -> bool {
        if self.init_failed {
            return false;
        }

        if self.gpu_ctx.is_some() {
            return true;
        }

        match VelloGpuContext::new() {
            Ok(ctx) => {
                self.gpu_ctx = Some(ctx);
                true
            }
            Err(e) => {
                log::error!("Failed to initialize Vello GPU context: {}", e);
                self.init_failed = true;
                self.init_error = Some(e.to_string());
                false
            }
        }
    }

    /// Render paths to a scene.
    fn build_scene(&self, paths: &[Path], transform: Affine) -> Scene {
        let mut scene = Scene::new();
        let vello_paths = convert_paths(paths);

        for vp in &vello_paths {
            // Draw fill
            if let Some(fill_color) = vp.style.fill {
                scene.fill(
                    vello::peniko::Fill::NonZero,
                    transform,
                    &vello::peniko::Brush::Solid(fill_color),
                    None,
                    &vp.bezpath,
                );
            }

            // Draw stroke
            if let Some(stroke_color) = vp.style.stroke {
                let stroke = vello::kurbo::Stroke::new(vp.style.stroke_width);
                scene.stroke(
                    &stroke,
                    transform,
                    &vello::peniko::Brush::Solid(stroke_color),
                    None,
                    &vp.bezpath,
                );
            }

            // Default stroke if no fill or stroke
            if vp.style.fill.is_none() && vp.style.stroke.is_none() {
                let stroke = vello::kurbo::Stroke::new(1.0);
                scene.stroke(
                    &stroke,
                    transform,
                    &vello::peniko::Brush::Solid(PenikoColor::BLACK),
                    None,
                    &vp.bezpath,
                );
            }
        }

        scene
    }

    /// Render the scene and copy to CPU buffer.
    fn render_to_image(
        &mut self,
        paths: &[Path],
        transform: Affine,
        width: u32,
        height: u32,
    ) -> Result<ColorImage, VelloViewerError> {
        // Build scene first (before taking mutable borrow of gpu_ctx)
        let scene = self.build_scene(paths, transform);
        let bg = crate::vello_convert::color_to_peniko(&self.background_color);

        let ctx = self.gpu_ctx.as_mut().ok_or(VelloViewerError::AdapterCreation)?;

        // Create render target texture
        let texture = ctx.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("vello_render_target"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::STORAGE_BINDING
                | wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Render
        let params = RenderParams {
            base_color: bg,
            width,
            height,
            antialiasing_method: AaConfig::Msaa16,
        };

        ctx.renderer
            .render_to_texture(&ctx.device, &ctx.queue, &scene, &view, &params)
            .map_err(|e| VelloViewerError::RenderFailed(format!("{:?}", e)))?;

        // Create buffer for reading back
        let bytes_per_row = (width * 4 + 255) & !255; // Align to 256 bytes
        let buffer_size = (bytes_per_row * height) as u64;

        let staging_buffer = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("vello_staging_buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        // Copy texture to buffer
        let mut encoder = ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("vello_copy_encoder"),
            });

        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &staging_buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(bytes_per_row),
                    rows_per_image: Some(height),
                },
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );

        ctx.queue.submit(std::iter::once(encoder.finish()));

        // Map buffer and read data
        let buffer_slice = staging_buffer.slice(..);
        let (sender, receiver) = std::sync::mpsc::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            let _ = sender.send(result);
        });

        ctx.device.poll(wgpu::PollType::wait_indefinitely()).ok();

        receiver
            .recv()
            .map_err(|_| VelloViewerError::BufferCopyFailed("Channel closed".to_string()))?
            .map_err(|e| VelloViewerError::BufferCopyFailed(e.to_string()))?;

        // Read pixel data
        let data = buffer_slice.get_mapped_range();
        let mut pixels = Vec::with_capacity((width * height) as usize);

        for y in 0..height {
            let row_start = (y * bytes_per_row) as usize;
            for x in 0..width {
                let offset = row_start + (x * 4) as usize;
                pixels.push(egui::Color32::from_rgba_unmultiplied(
                    data[offset],
                    data[offset + 1],
                    data[offset + 2],
                    data[offset + 3],
                ));
            }
        }

        drop(data);
        staging_buffer.unmap();

        Ok(ColorImage {
            size: [width as usize, height as usize],
            pixels,
        })
    }

    /// Render and display in egui.
    ///
    /// Returns true if GPU rendering was used, false if fallback is needed.
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        paths: &[Path],
        transform: Affine,
        rect: egui::Rect,
    ) -> bool {
        let width = rect.width().max(1.0) as u32;
        let height = rect.height().max(1.0) as u32;

        // Check if we need to re-render
        let needs_render = self.texture.is_none()
            || self.width != width
            || self.height != height;

        if needs_render {
            if !self.ensure_initialized() {
                // Show error message
                if let Some(ref err) = self.init_error {
                    ui.painter().rect_filled(
                        rect,
                        0.0,
                        egui::Color32::from_rgb(40, 40, 40),
                    );
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        format!("GPU rendering unavailable: {}", err),
                        egui::FontId::default(),
                        egui::Color32::RED,
                    );
                }
                return false;
            }

            self.width = width;
            self.height = height;

            match self.render_to_image(paths, transform, width, height) {
                Ok(image) => {
                    self.texture = Some(ui.ctx().load_texture(
                        "vello_output",
                        image,
                        TextureOptions::LINEAR,
                    ));
                }
                Err(e) => {
                    log::error!("Vello render failed: {}", e);
                    return false;
                }
            }
        }

        // Display the texture
        if let Some(ref texture) = self.texture {
            ui.painter().image(
                texture.id(),
                rect,
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                egui::Color32::WHITE,
            );
            true
        } else {
            false
        }
    }

    /// Force a re-render on the next frame.
    pub fn invalidate(&mut self) {
        self.texture = None;
    }

    /// Check if GPU rendering is available.
    pub fn is_available(&self) -> bool {
        !self.init_failed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vello_viewer_new() {
        let viewer = VelloViewer::new();
        assert!(viewer.gpu_ctx.is_none());
        assert!(!viewer.init_failed);
    }

    #[test]
    fn test_vello_viewer_set_background() {
        let mut viewer = VelloViewer::new();
        viewer.set_background_color(Color::rgb(0.5, 0.5, 0.5));
        assert_eq!(viewer.background_color.r, 0.5);
    }
}
