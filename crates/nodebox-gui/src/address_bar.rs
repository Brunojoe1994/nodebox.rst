//! Address bar with breadcrumb navigation.

#![allow(dead_code)]

use eframe::egui::{self, Sense};
use crate::theme;

/// The address bar showing current network path.
pub struct AddressBar {
    /// Path segments (e.g., ["root", "network1"]).
    segments: Vec<String>,
    /// Status message displayed on the right.
    message: String,
    /// Hovered segment index (for highlighting).
    hovered_segment: Option<usize>,
}

impl Default for AddressBar {
    fn default() -> Self {
        Self::new()
    }
}

impl AddressBar {
    /// Create a new address bar.
    pub fn new() -> Self {
        Self {
            segments: vec!["root".to_string()],
            message: String::new(),
            hovered_segment: None,
        }
    }

    /// Set the current path from a path string (e.g., "/root/network1").
    pub fn set_path(&mut self, path: &str) {
        self.segments = path
            .trim_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect();
        if self.segments.is_empty() {
            self.segments.push("root".to_string());
        }
    }

    /// Set the status message.
    pub fn set_message(&mut self, message: impl Into<String>) {
        self.message = message.into();
    }

    /// Clear the status message.
    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    /// Get the current path as a string.
    pub fn path(&self) -> String {
        format!("/{}", self.segments.join("/"))
    }

    /// Show the address bar. Returns the clicked path if a segment was clicked.
    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<String> {
        let mut clicked_path = None;
        self.hovered_segment = None;

        // Clean background - uses panel bg for seamless integration
        let rect = ui.available_rect_before_wrap();
        ui.painter().rect_filled(rect, 0.0, theme::PANEL_BG);

        // Subtle bottom border only
        ui.painter().line_segment(
            [
                egui::pos2(rect.min.x, rect.max.y - 1.0),
                egui::pos2(rect.max.x, rect.max.y - 1.0),
            ],
            egui::Stroke::new(1.0, theme::BORDER_COLOR),
        );

        ui.horizontal(|ui| {
            ui.add_space(theme::PADDING);

            // Draw path segments with separators - smaller, more subtle
            for (i, segment) in self.segments.iter().enumerate() {
                // Separator (except before first segment)
                if i > 0 {
                    ui.label(
                        egui::RichText::new("/")
                            .color(theme::TEXT_DISABLED)
                            .size(11.0),
                    );
                }

                // Segment as clickable text - subtle styling
                let is_last = i == self.segments.len() - 1;
                let text_color = if is_last {
                    theme::TEXT_DEFAULT
                } else {
                    theme::TEXT_SUBDUED
                };

                let response = ui.add(
                    egui::Label::new(
                        egui::RichText::new(segment)
                            .color(text_color)
                            .size(11.0),
                    )
                    .sense(Sense::click()),
                );

                // Subtle hover effect
                if response.hovered() {
                    self.hovered_segment = Some(i);
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }

                // Handle click - navigate to this segment's path
                if response.clicked() {
                    let path = format!(
                        "/{}",
                        self.segments[..=i].join("/")
                    );
                    clicked_path = Some(path);
                }
            }

            // Right-aligned status message
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(theme::PADDING);
                if !self.message.is_empty() {
                    ui.label(
                        egui::RichText::new(&self.message)
                            .color(theme::TEXT_DISABLED)
                            .size(10.0),
                    );
                }
            });
        });

        clicked_path
    }
}
