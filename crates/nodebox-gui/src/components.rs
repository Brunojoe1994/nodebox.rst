//! Shared UI components for consistent styling across panes.
//!
//! This module provides reusable building blocks for the NodeBox UI,
//! ensuring consistent appearance and behavior.

#![allow(dead_code)]

use eframe::egui::{self, Rect};
use crate::theme;

/// Draw a pane header with title and return position info for additional content.
///
/// This function draws:
/// - Header background
/// - UPPERCASE title text on the left
/// - Vertical separator line after the title
///
/// Returns the x position after the separator where additional content can be placed.
pub fn draw_pane_header_with_title(
    ui: &mut egui::Ui,
    title: &str,
) -> (Rect, f32) {
    let header_height = theme::PANE_HEADER_HEIGHT;
    let (header_rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), header_height),
        egui::Sense::hover(),
    );

    // Pixel-align the rect for crisp borders
    let aligned_rect = Rect::from_min_max(
        egui::pos2(header_rect.min.x.floor(), header_rect.min.y.floor()),
        egui::pos2(header_rect.max.x.ceil(), header_rect.max.y.floor()),
    );

    // Header background (fills the entire rect including border areas)
    ui.painter().rect_filled(
        aligned_rect,
        0.0,
        theme::PANE_HEADER_BACKGROUND_COLOR,
    );

    // Top border (1px light line) - draw at top edge
    ui.painter().line_segment(
        [
            egui::pos2(aligned_rect.left(), aligned_rect.top() + 0.5),
            egui::pos2(aligned_rect.right(), aligned_rect.top() + 0.5),
        ],
        egui::Stroke::new(1.0, theme::SLATE_700),
    );

    // Title on left (UPPERCASE)
    let title_font = egui::FontId::proportional(10.0);
    let title_galley = ui.painter().layout_no_wrap(
        title.to_uppercase(),
        title_font,
        theme::PANE_HEADER_FOREGROUND_COLOR,
    );
    let title_x = header_rect.left() + theme::PADDING;
    ui.painter().galley(
        egui::pos2(title_x, header_rect.center().y - title_galley.size().y / 2.0),
        title_galley.clone(),
        theme::PANE_HEADER_FOREGROUND_COLOR,
    );

    // Vertical separator line (1px) - 8px after title
    let sep_x = title_x + title_galley.size().x + theme::PADDING;
    ui.painter().line_segment(
        [
            egui::pos2(sep_x, header_rect.top() + theme::PADDING_SMALL),
            egui::pos2(sep_x, header_rect.bottom() - theme::PADDING_SMALL),
        ],
        egui::Stroke::new(1.0, theme::TEXT_DISABLED),
    );

    // Bottom border (1px dark line) - draw at bottom edge
    ui.painter().line_segment(
        [
            egui::pos2(aligned_rect.left(), aligned_rect.bottom() - 0.5),
            egui::pos2(aligned_rect.right(), aligned_rect.bottom() - 0.5),
        ],
        egui::Stroke::new(1.0, theme::SLATE_950),
    );

    // Return header rect and x position after separator (8px margin)
    let content_start_x = sep_x + theme::PADDING;
    (header_rect, content_start_x)
}

/// Draw a simple text button in a header.
///
/// Returns true if clicked.
pub fn header_text_button(
    ui: &mut egui::Ui,
    header_rect: Rect,
    x: f32,
    text: &str,
    width: f32,
) -> (bool, f32) {
    let button_font = egui::FontId::proportional(10.0);
    let button_rect = Rect::from_min_size(
        egui::pos2(x, header_rect.top()),
        egui::vec2(width, header_rect.height()),
    );

    let response = ui.interact(
        button_rect,
        ui.id().with(text),
        egui::Sense::click(),
    );

    let color = if response.hovered() {
        theme::TEXT_STRONG
    } else {
        theme::PANE_HEADER_FOREGROUND_COLOR
    };

    ui.painter().text(
        button_rect.left_center(),
        egui::Align2::LEFT_CENTER,
        text,
        button_font,
        color,
    );

    (response.clicked(), x + width + theme::PADDING)
}

/// Draw a toggle button in a header (subtle, Figma-like styling).
///
/// Returns true if clicked.
pub fn header_toggle_button(
    ui: &mut egui::Ui,
    header_rect: Rect,
    x: f32,
    text: &str,
    active: bool,
    tooltip: &str,
) -> (bool, f32) {
    let font = egui::FontId::proportional(10.0);

    // Calculate text width for proper sizing
    let galley = ui.painter().layout_no_wrap(
        text.to_string(),
        font.clone(),
        theme::TEXT_DEFAULT,
    );
    let text_width = galley.size().x;

    let button_rect = Rect::from_min_size(
        egui::pos2(x, header_rect.top()),
        egui::vec2(text_width, header_rect.height()),
    );

    let response = ui.interact(
        button_rect,
        ui.id().with(text),
        egui::Sense::click(),
    );

    let color = if active {
        theme::TEXT_DEFAULT
    } else {
        theme::TEXT_DISABLED
    };

    ui.painter().text(
        egui::pos2(x, header_rect.center().y),
        egui::Align2::LEFT_CENTER,
        text,
        font,
        color,
    );

    if !tooltip.is_empty() {
        response.clone().on_hover_text(tooltip);
    }

    (response.clicked(), x + text_width + theme::PADDING)
}

/// Draw a tab button in a header.
///
/// Returns true if clicked.
pub fn header_tab_button(
    ui: &mut egui::Ui,
    header_rect: Rect,
    x: f32,
    text: &str,
    active: bool,
) -> (bool, f32) {
    let font = egui::FontId::proportional(11.0);

    // Calculate text width
    let galley = ui.painter().layout_no_wrap(
        text.to_string(),
        font.clone(),
        theme::TEXT_STRONG,
    );
    let text_width = galley.size().x;

    let button_rect = Rect::from_min_size(
        egui::pos2(x, header_rect.top()),
        egui::vec2(text_width, header_rect.height()),
    );

    let response = ui.interact(
        button_rect,
        ui.id().with(format!("tab_{}", text)),
        egui::Sense::click(),
    );

    let color = if active {
        theme::TEXT_STRONG
    } else {
        theme::TEXT_SUBDUED
    };

    ui.painter().text(
        egui::pos2(x, header_rect.center().y),
        egui::Align2::LEFT_CENTER,
        text,
        font,
        color,
    );

    (response.clicked(), x + text_width + theme::PADDING_LARGE)
}

/// Draw a vertical separator line in a header.
pub fn header_separator(ui: &mut egui::Ui, header_rect: Rect, x: f32) -> f32 {
    ui.painter().line_segment(
        [
            egui::pos2(x, header_rect.top() + theme::PADDING_SMALL),
            egui::pos2(x, header_rect.bottom() - theme::PADDING_SMALL),
        ],
        egui::Stroke::new(1.0, theme::TEXT_DISABLED),
    );
    x + theme::PADDING
}
