//! Compact animation bar with playback controls.
//!
//! Note: This module is work-in-progress and not yet integrated.

#![allow(dead_code)]

use eframe::egui;
use std::time::{Duration, Instant};
use crate::theme;

/// Animation playback state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
}

/// Events that can be triggered by the animation bar.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AnimationEvent {
    None,
    Play,
    Pause,
    Stop,
    Rewind,
    StepBack,
    StepForward,
    GoToEnd,
    FrameChanged(f64),
    FpsChanged(u32),
}

/// Compact animation bar widget.
pub struct AnimationBar {
    /// Current frame number.
    frame: u32,
    /// Start frame.
    start_frame: u32,
    /// End frame (total frames).
    end_frame: u32,
    /// Frames per second.
    fps: u32,
    /// Current playback state.
    playback_state: PlaybackState,
    /// Whether to loop the animation.
    loop_enabled: bool,
    /// Time of last frame update.
    last_frame_time: Option<Instant>,
    /// Accumulated time since last frame.
    accumulated_time: Duration,
}

impl Default for AnimationBar {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationBar {
    /// Create a new animation bar.
    pub fn new() -> Self {
        Self {
            frame: 1,
            start_frame: 1,
            end_frame: 100,
            fps: 30,
            playback_state: PlaybackState::Stopped,
            loop_enabled: true,
            last_frame_time: None,
            accumulated_time: Duration::ZERO,
        }
    }

    /// Get the current frame.
    pub fn frame(&self) -> u32 {
        self.frame
    }

    /// Get the current frame as f64.
    pub fn frame_f64(&self) -> f64 {
        self.frame as f64
    }

    /// Set the current frame.
    pub fn set_frame(&mut self, frame: u32) {
        self.frame = frame.clamp(self.start_frame, self.end_frame);
    }

    /// Get the normalized time (0.0 to 1.0).
    pub fn normalized_time(&self) -> f64 {
        let range = (self.end_frame - self.start_frame) as f64;
        if range > 0.0 {
            (self.frame - self.start_frame) as f64 / range
        } else {
            0.0
        }
    }

    /// Is the animation playing?
    pub fn is_playing(&self) -> bool {
        self.playback_state == PlaybackState::Playing
    }

    /// Play the animation.
    pub fn play(&mut self) {
        self.playback_state = PlaybackState::Playing;
        self.last_frame_time = Some(Instant::now());
        self.accumulated_time = Duration::ZERO;
    }

    /// Pause the animation.
    pub fn pause(&mut self) {
        self.playback_state = PlaybackState::Paused;
    }

    /// Stop the animation and reset to start.
    pub fn stop(&mut self) {
        self.playback_state = PlaybackState::Stopped;
        self.frame = self.start_frame;
        self.last_frame_time = None;
    }

    /// Step forward one frame.
    pub fn step_forward(&mut self) {
        if self.frame < self.end_frame {
            self.frame += 1;
        } else if self.loop_enabled {
            self.frame = self.start_frame;
        }
    }

    /// Step backward one frame.
    pub fn step_backward(&mut self) {
        if self.frame > self.start_frame {
            self.frame -= 1;
        } else if self.loop_enabled {
            self.frame = self.end_frame;
        }
    }

    /// Go to first frame.
    pub fn rewind(&mut self) {
        self.frame = self.start_frame;
    }

    /// Go to last frame.
    pub fn go_to_end(&mut self) {
        self.frame = self.end_frame;
    }

    /// Update playback (call each frame).
    /// Returns true if the frame changed.
    pub fn update(&mut self) -> bool {
        if self.playback_state != PlaybackState::Playing {
            return false;
        }

        let now = Instant::now();
        let frame_duration = Duration::from_secs_f32(1.0 / self.fps as f32);

        if let Some(last_time) = self.last_frame_time {
            self.accumulated_time += now - last_time;
            self.last_frame_time = Some(now);

            if self.accumulated_time >= frame_duration {
                self.accumulated_time -= frame_duration;
                self.step_forward();

                // Stop at end if not looping
                if !self.loop_enabled && self.frame >= self.end_frame {
                    self.playback_state = PlaybackState::Stopped;
                }

                return true;
            }
        } else {
            self.last_frame_time = Some(now);
        }

        false
    }

    /// Show the animation bar.
    pub fn show(&mut self, ui: &mut egui::Ui) -> AnimationEvent {
        let mut event = AnimationEvent::None;

        // Clean background - seamless with panel
        let rect = ui.available_rect_before_wrap();
        ui.painter().rect_filled(rect, 0.0, theme::ANIMATION_BAR_BACKGROUND);

        // Subtle top border only
        ui.painter().line_segment(
            [
                egui::pos2(rect.min.x, rect.min.y),
                egui::pos2(rect.max.x, rect.min.y),
            ],
            egui::Stroke::new(1.0, theme::BORDER_COLOR),
        );

        ui.horizontal(|ui| {
            ui.add_space(theme::PADDING_SMALL);

            // Playback control buttons - flush with bar height, transparent background
            if self.icon_button(ui, "⏮", "Rewind") {
                self.rewind();
                event = AnimationEvent::Rewind;
            }

            if self.icon_button(ui, "⏪", "Step backward") {
                self.step_backward();
                event = AnimationEvent::StepBack;
            }

            let (play_icon, play_tooltip) = if self.is_playing() {
                ("⏸", "Pause")
            } else {
                ("▶", "Play")
            };
            if self.icon_button(ui, play_icon, play_tooltip) {
                if self.is_playing() {
                    self.pause();
                    event = AnimationEvent::Pause;
                } else {
                    self.play();
                    event = AnimationEvent::Play;
                }
            }

            if self.icon_button(ui, "⏩", "Step forward") {
                self.step_forward();
                event = AnimationEvent::StepForward;
            }

            if self.icon_button(ui, "⏭", "Go to end") {
                self.go_to_end();
                event = AnimationEvent::GoToEnd;
            }

            if self.icon_button(ui, "⏹", "Stop") {
                self.stop();
                event = AnimationEvent::Stop;
            }

            ui.add_space(theme::PADDING);

            // Frame counter
            ui.label(
                egui::RichText::new("Frame")
                    .color(theme::TEXT_SUBDUED)
                    .size(theme::FONT_SIZE_SMALL),
            );
            let mut frame = self.frame as i32;
            let frame_response = Self::styled_drag_value(ui, &mut frame, self.start_frame as i32..=self.end_frame as i32);
            if frame_response.changed() {
                self.frame = frame as u32;
                event = AnimationEvent::FrameChanged(self.frame as f64);
            }

            ui.label(
                egui::RichText::new(format!("/{}", self.end_frame))
                    .color(theme::TEXT_DISABLED)
                    .size(theme::FONT_SIZE_SMALL),
            );

            ui.add_space(theme::PADDING);

            // FPS control
            ui.label(
                egui::RichText::new("FPS")
                    .color(theme::TEXT_SUBDUED)
                    .size(theme::FONT_SIZE_SMALL),
            );
            let mut fps = self.fps as i32;
            let fps_response = Self::styled_drag_value(ui, &mut fps, 1..=120);
            if fps_response.changed() {
                self.fps = fps as u32;
                event = AnimationEvent::FpsChanged(self.fps);
            }

            ui.add_space(theme::PADDING);

            // Loop toggle
            Self::styled_checkbox(ui, &mut self.loop_enabled);
            ui.label(
                egui::RichText::new("Loop")
                    .color(if self.loop_enabled { theme::TEXT_DEFAULT } else { theme::TEXT_SUBDUED })
                    .size(theme::FONT_SIZE_SMALL),
            );
        });

        event
    }

    /// Styled DragValue that follows the style guide.
    /// Uses SLATE_800 for subtle elevation against SLATE_900 bar background.
    fn styled_drag_value(ui: &mut egui::Ui, value: &mut i32, range: std::ops::RangeInclusive<i32>) -> egui::Response {
        // Override visuals for this widget - use SLATE_800 for subtle elevation
        let old_visuals = ui.visuals().clone();

        // All states: no borders, sharp corners, appropriate fill
        ui.visuals_mut().widgets.inactive.bg_fill = theme::SLATE_800;
        ui.visuals_mut().widgets.inactive.weak_bg_fill = theme::SLATE_800;
        ui.visuals_mut().widgets.inactive.bg_stroke = egui::Stroke::NONE;
        ui.visuals_mut().widgets.inactive.fg_stroke = egui::Stroke::new(1.0, theme::TEXT_DEFAULT);
        ui.visuals_mut().widgets.inactive.rounding = egui::Rounding::ZERO;

        ui.visuals_mut().widgets.hovered.bg_fill = theme::SLATE_700;
        ui.visuals_mut().widgets.hovered.weak_bg_fill = theme::SLATE_700;
        ui.visuals_mut().widgets.hovered.bg_stroke = egui::Stroke::NONE;
        ui.visuals_mut().widgets.hovered.fg_stroke = egui::Stroke::new(1.0, theme::TEXT_STRONG);
        ui.visuals_mut().widgets.hovered.rounding = egui::Rounding::ZERO;

        ui.visuals_mut().widgets.active.bg_fill = theme::SLATE_700;
        ui.visuals_mut().widgets.active.weak_bg_fill = theme::SLATE_700;
        ui.visuals_mut().widgets.active.bg_stroke = egui::Stroke::NONE;
        ui.visuals_mut().widgets.active.fg_stroke = egui::Stroke::new(1.0, theme::TEXT_STRONG);
        ui.visuals_mut().widgets.active.rounding = egui::Rounding::ZERO;

        ui.visuals_mut().widgets.noninteractive.bg_fill = theme::SLATE_800;
        ui.visuals_mut().widgets.noninteractive.weak_bg_fill = theme::SLATE_800;
        ui.visuals_mut().widgets.noninteractive.bg_stroke = egui::Stroke::NONE;
        ui.visuals_mut().widgets.noninteractive.rounding = egui::Rounding::ZERO;

        let response = ui.add(
            egui::DragValue::new(value)
                .range(range)
                .speed(1.0),
        );

        // Restore visuals
        *ui.visuals_mut() = old_visuals;

        response
    }

    /// Styled checkbox that follows the style guide.
    fn styled_checkbox(ui: &mut egui::Ui, checked: &mut bool) -> egui::Response {
        // Override visuals for this widget
        let old_visuals = ui.visuals().clone();

        ui.visuals_mut().widgets.inactive.bg_fill = theme::SLATE_800;
        ui.visuals_mut().widgets.inactive.weak_bg_fill = theme::SLATE_800;
        ui.visuals_mut().widgets.inactive.bg_stroke = egui::Stroke::NONE;
        ui.visuals_mut().widgets.inactive.rounding = egui::Rounding::ZERO;

        ui.visuals_mut().widgets.hovered.bg_fill = theme::SLATE_700;
        ui.visuals_mut().widgets.hovered.weak_bg_fill = theme::SLATE_700;
        ui.visuals_mut().widgets.hovered.bg_stroke = egui::Stroke::NONE;
        ui.visuals_mut().widgets.hovered.rounding = egui::Rounding::ZERO;

        ui.visuals_mut().widgets.active.bg_fill = theme::SLATE_700;
        ui.visuals_mut().widgets.active.weak_bg_fill = theme::SLATE_700;
        ui.visuals_mut().widgets.active.bg_stroke = egui::Stroke::NONE;
        ui.visuals_mut().widgets.active.rounding = egui::Rounding::ZERO;

        ui.visuals_mut().widgets.noninteractive.bg_fill = theme::SLATE_800;
        ui.visuals_mut().widgets.noninteractive.weak_bg_fill = theme::SLATE_800;
        ui.visuals_mut().widgets.noninteractive.bg_stroke = egui::Stroke::NONE;
        ui.visuals_mut().widgets.noninteractive.rounding = egui::Rounding::ZERO;

        let response = ui.checkbox(checked, "");

        // Restore visuals
        *ui.visuals_mut() = old_visuals;

        response
    }

    /// Custom icon button with transparent background and hover effect.
    /// Returns true if clicked.
    fn icon_button(&self, ui: &mut egui::Ui, icon: &str, tooltip: &str) -> bool {
        let button_size = egui::vec2(theme::ANIMATION_BAR_HEIGHT, theme::ANIMATION_BAR_HEIGHT);
        let (rect, response) = ui.allocate_exact_size(button_size, egui::Sense::click());

        if ui.is_rect_visible(rect) {
            // Transparent background, lighter on hover
            let bg_color = if response.hovered() {
                theme::SLATE_800
            } else {
                egui::Color32::TRANSPARENT
            };

            ui.painter().rect_filled(rect, 0.0, bg_color);

            // Icon color - brighter on hover
            let text_color = if response.hovered() {
                theme::TEXT_STRONG
            } else {
                theme::TEXT_DEFAULT
            };

            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                icon,
                egui::FontId::proportional(theme::FONT_SIZE_BASE),
                text_color,
            );
        }

        response.on_hover_text(tooltip).clicked()
    }
}
