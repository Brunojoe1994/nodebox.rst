# NodeBox Design System

This document defines the visual language and design principles for the NodeBox GUI. All UI development should follow these guidelines to ensure a consistent, professional, and usable interface.

**Reference Implementation:** `crates/nodebox-gui/src/theme.rs`

---

## Design Philosophy

NodeBox follows a **Figma-inspired design philosophy** with these core principles:

### 1. Angular & Geometric
- **Square edges preferred** over excessive rounding
- Corner radii: `4px` for all elements (panels, windows, widgets)
- No decorative curves or ornamental shapes
- Grid-aligned elements create visual harmony

### 2. Typography-Driven Hierarchy
- **Text is the primary visual element** — use size, weight, and color to create hierarchy
- Let typography do the work instead of boxes, badges, or icons
- Strong contrast between heading, body, and subdued text levels
- Consistent font sizes across the entire application

### 3. Space Over Lines
- **Use whitespace to delineate sections** instead of borders and dividers
- Reserve borders for structural elements (panels, inputs, selections)
- Consistent 8px-based spacing grid creates visual rhythm
- Generous padding makes content breathable

### 4. Subtle & Functional
- **No drop shadows** on panels or UI elements
- Minimal hover effects (1px expansion, subtle background change)
- State changes through color, not animation or decoration
- Every visual element must serve a purpose

### 5. High Contrast Dark Theme
- **Optimized for extended use** with comfortable contrast ratios
- White text on dark backgrounds for primary content
- Subtle gray variations create depth without visual noise

---

## Color System

### Gray Scale (21-stop, Dark Theme)

The gray scale provides smooth transitions for backgrounds, borders, and text.

| Token | RGB | Usage |
|-------|-----|-------|
| `GRAY_0` | `(0, 0, 0)` | Pure black (rarely used) |
| `GRAY_100` | `(13, 16, 17)` | Main panel backgrounds |
| `GRAY_150` | `(20, 24, 25)` | Bottom bars, recessed areas |
| `GRAY_200` | `(28, 33, 35)` | Tab bars, input backgrounds |
| `GRAY_250` | `(38, 43, 46)` | Borders, pane headers |
| `GRAY_300` | `(49, 56, 59)` | Inactive widget backgrounds |
| `GRAY_325` | `(55, 63, 66)` | Hover/active states |
| `GRAY_400` | `(76, 86, 90)` | Disabled text, secondary borders |
| `GRAY_550` | `(125, 140, 146)` | Subdued/muted text |
| `GRAY_700` | `(174, 194, 202)` | Header text |
| `GRAY_775` | `(202, 216, 222)` | Default body text |
| `GRAY_800` | `(211, 222, 227)` | Bright text variant |
| `GRAY_1000` | `(255, 255, 255)` | Strong/active text |

### Primary Accent: Blue

Blue is the primary accent color for selections, links, and interactive highlights.

| Token | RGB | Usage |
|-------|-----|-------|
| `BLUE_350` | `(24, 73, 187)` | Selection backgrounds |
| `BLUE_400` | `(51, 102, 255)` | Selection strokes |
| `BLUE_500` | `(68, 138, 255)` | Links, value text, primary accent |

### Status Colors

Use sparingly for semantic feedback only.

| Token | RGB | Usage |
|-------|-----|-------|
| `SUCCESS_GREEN` | `(0, 218, 126)` | Success states, valid input |
| `WARNING_ORANGE` | `(255, 122, 12)` | Warnings, caution states |
| `ERROR_RED` | `(171, 1, 22)` | Errors, destructive actions |

### Semantic Color Mapping

**Always use semantic tokens**, never raw gray values:

```rust
// Background colors
PANEL_BG          // Main panel background
TOP_BAR_BG        // Title/top bar
TAB_BAR_BG        // Tab bar background
BOTTOM_BAR_BG     // Footer/bottom bar
TEXT_EDIT_BG      // Input field background
HOVER_BG          // Hover state background
SELECTION_BG      // Selection highlight

// Text colors
TEXT_STRONG       // Primary/active text (white)
TEXT_DEFAULT      // Body text
TEXT_SUBDUED      // Secondary/muted text
TEXT_DISABLED     // Disabled/non-interactive

// Widget colors
WIDGET_INACTIVE_BG
WIDGET_HOVERED_BG
WIDGET_ACTIVE_BG
WIDGET_NONINTERACTIVE_BG
BORDER_COLOR
BORDER_SECONDARY
```

---

## Typography

### Font Sizes

| Token | Size | Usage |
|-------|------|-------|
| `FONT_SIZE_SMALL` | 11px | Labels, captions, metadata |
| `FONT_SIZE_BASE` | 12px | Body text, buttons, inputs |
| `FONT_SIZE_HEADING` | 16px | Section headings |

### Text Hierarchy

Use text color to establish hierarchy, not font weight:

1. **Strong** (`TEXT_STRONG` / white) — Active, focused, or primary content
2. **Default** (`TEXT_DEFAULT`) — Standard body text
3. **Subdued** (`TEXT_SUBDUED`) — Secondary information, hints
4. **Disabled** (`TEXT_DISABLED`) — Non-interactive elements

```rust
// Example: Parameter row
ui.label(RichText::new("Width").color(TEXT_SUBDUED));      // Label
ui.label(RichText::new("1920").color(BLUE_500));           // Value (interactive)
```

### Line Height

Use `LINE_HEIGHT_RATIO = 1.333` for comfortable reading in dense UIs.

---

## Spacing System

All spacing follows an **8px grid** for visual consistency.

### Spacing Tokens

| Token | Value | Usage |
|-------|-------|-------|
| `PADDING_SMALL` | 4px | Tight spaces, icon margins |
| `PADDING` | 8px | Standard padding, item spacing |
| `PADDING_LARGE` | 12px | Button padding, panel margins |
| `VIEW_PADDING` | 12px | Outer panel padding |
| `ITEM_SPACING` | 8px | Space between list items |
| `INDENT` | 16px | Hierarchical indentation |
| `ICON_TEXT_PADDING` | 4px | Icon-to-text gap |

### Layout Heights

| Token | Value | Usage |
|-------|-------|-------|
| `TOP_BAR_HEIGHT` | 28px | Address bar, toolbar |
| `TITLE_BAR_HEIGHT` | 24px | Pane headers |
| `LIST_ITEM_HEIGHT` | 24px | List items, tree nodes |
| `ROW_HEIGHT` | 24px | Parameter rows, table rows |
| `TABLE_HEADER_HEIGHT` | 32px | Table headers |

### Spacing Principles

1. **Use consistent spacing** — Same spacing between similar elements
2. **Increase spacing to separate groups** — Use `PADDING_LARGE` between sections
3. **Reduce spacing within groups** — Use `PADDING_SMALL` for related items
4. **Align to the grid** — All dimensions should be multiples of 4px

---

## Component Patterns

### Pane Headers

Consistent styling across all panes:

```rust
let header_rect = ui.available_rect_before_wrap();
let header_rect = header_rect.with_max_y(header_rect.min.y + PANE_HEADER_HEIGHT);

// Background
ui.painter().rect_filled(header_rect, 0.0, PANE_HEADER_BACKGROUND_COLOR);

// Title text
ui.label(
    RichText::new("PARAMETERS")
        .size(FONT_SIZE_SMALL)
        .color(PANE_HEADER_FOREGROUND_COLOR)
);
```

- Height: `24px` (`PANE_HEADER_HEIGHT`)
- Background: `PANE_HEADER_BACKGROUND_COLOR` (GRAY_250)
- Text: `PANE_HEADER_FOREGROUND_COLOR` (GRAY_700), 11px uppercase
- No border on top, optional 1px border on bottom

### Buttons

```rust
// Standard button
let response = ui.button(RichText::new("Export").size(FONT_SIZE_BASE));

// Icon button (no text)
let response = ui.add(egui::Button::new("⚙").frame(false));
```

- Padding: `12px` horizontal, `8px` vertical
- Corner radius: `4px`
- Hover: Background → `WIDGET_HOVERED_BG`, 1px expansion
- Active: Background → `WIDGET_ACTIVE_BG`

### Input Fields

```rust
let response = ui.add(
    egui::TextEdit::singleline(&mut value)
        .desired_width(100.0)
        .font(FontId::proportional(FONT_SIZE_BASE))
);
```

- Background: `TEXT_EDIT_BG` (GRAY_200)
- Text: `TEXT_DEFAULT`
- Focused border: 1px `BLUE_400`
- Corner radius: `4px`

### Lists & Selections

```rust
let is_selected = current_item == item_id;
let is_hovered = response.hovered();

let bg_color = if is_selected {
    SELECTION_BG
} else if is_hovered {
    HOVER_BG
} else {
    Color32::TRANSPARENT
};

ui.painter().rect_filled(item_rect, CORNER_RADIUS_SMALL, bg_color);
```

- Item height: `24px` (`LIST_ITEM_HEIGHT`)
- Selected: `SELECTION_BG` with 1px `BLUE_400` stroke
- Hovered: `HOVER_BG`
- Corner radius: `4px`
- Inner padding: `8px`

### Dialogs / Modals

```rust
egui::Window::new("Add Node")
    .fixed_size([500.0, 400.0])
    .frame(egui::Frame::window(&ctx.style())
        .fill(PANEL_BG)
        .stroke(Stroke::new(1.0, BORDER_COLOR))
        .rounding(CORNER_RADIUS))
```

- Fixed size (no resizing by default)
- Background: `PANEL_BG`
- Border: 1px `BORDER_COLOR`
- Corner radius: `4px`
- No drop shadow

### Separators

Use sparingly — prefer spacing over lines:

```rust
// Horizontal separator
ui.add(egui::Separator::default().horizontal());

// Or draw manually for control
let line_rect = Rect::from_min_max(
    pos2(rect.left() + PADDING, y),
    pos2(rect.right() - PADDING, y + 1.0)
);
ui.painter().rect_filled(line_rect, 0.0, BORDER_COLOR);
```

- Color: `BORDER_COLOR` (GRAY_250)
- Thickness: 1px
- Inset from edges by `PADDING`

---

## Interaction States

### State Progression

| State | Background | Text | Border |
|-------|------------|------|--------|
| Noninteractive | `GRAY_150` | `TEXT_SUBDUED` | None |
| Inactive | `GRAY_300` | `TEXT_DEFAULT` | None |
| Hovered | `GRAY_325` | `TEXT_STRONG` | None |
| Active/Pressed | `GRAY_325` | `TEXT_STRONG` | None |
| Selected | `BLUE_350` | `TEXT_STRONG` | 1px `BLUE_400` |
| Disabled | `GRAY_150` | `TEXT_DISABLED` | None |

### Hover Effects

- Background color change (subtle step up in gray scale)
- 1px expansion on widgets
- Cursor change to pointer for clickable elements
- No shadows, glows, or animations

### Focus States

- Input fields: 1px `BLUE_400` border
- Buttons: Same as hover state
- List items: Selected state

---

## Layout Guidelines

### Panel Structure

```
┌─────────────────────────────────────┐
│ PANE HEADER (24px)                  │  ← PANE_HEADER_BACKGROUND_COLOR
├─────────────────────────────────────┤
│                                     │
│  Content Area                       │  ← PANEL_BG
│  (VIEW_PADDING on all sides)        │
│                                     │
└─────────────────────────────────────┘
```

### Parameter Panel Layout

```
┌─────────────────────────────────────┐
│ PARAMETERS    separator   Node Name │  ← Header (24px)
├─────────────────────────────────────┤
│ Label          │ Value              │  ← Row (24px each)
│ Label          │ Value              │
│ Label          │ Value              │
│ ...                                 │
└─────────────────────────────────────┘
```

- Label width: `100px` fixed
- Value: fills remaining space
- Row height: `24px`
- Horizontal spacing: `8px`
- Vertical spacing: `0px` between rows (dense layout)

### Grid Alignment

The network view uses a 48px grid:

```
GRID_CELL_SIZE = 48px
NODE_MARGIN = 8px
NODE_WIDTH = 128px (48×3 - 8×2)
NODE_HEIGHT = 32px (48×1 - 8×2)
NODE_PADDING = 4px
NODE_ICON_SIZE = 24px
```

---

## Do's and Don'ts

### Do

- ✓ Use semantic color tokens from `theme.rs`
- ✓ Follow the 8px spacing grid
- ✓ Use typography hierarchy for emphasis
- ✓ Keep borders thin (1px) and subtle
- ✓ Provide hover feedback on interactive elements
- ✓ Test with actual content to verify spacing

### Don't

- ✗ Hardcode hex color values
- ✗ Add drop shadows to panels or dialogs
- ✗ Use borders when spacing would suffice
- ✗ Create custom fonts or sizes outside the system
- ✗ Add decorative elements that don't serve function
- ✗ Use animation for state changes (keep it snappy)

---

## Implementation Checklist

When creating new UI components:

1. [ ] Import tokens from `crate::theme`
2. [ ] Use semantic color constants, not raw values
3. [ ] Align dimensions to 4px/8px grid
4. [ ] Use standard heights (`ROW_HEIGHT`, `LIST_ITEM_HEIGHT`, etc.)
5. [ ] Implement hover states with `HOVER_BG`
6. [ ] Test with both short and long content
7. [ ] Verify text is readable (proper contrast)
8. [ ] Check alignment with adjacent components

---

## Quick Reference

```rust
use crate::theme::{
    // Backgrounds
    PANEL_BG, HOVER_BG, SELECTION_BG, TEXT_EDIT_BG,

    // Text
    TEXT_STRONG, TEXT_DEFAULT, TEXT_SUBDUED, TEXT_DISABLED,

    // Widgets
    WIDGET_INACTIVE_BG, WIDGET_HOVERED_BG, WIDGET_ACTIVE_BG,

    // Borders
    BORDER_COLOR, CORNER_RADIUS, CORNER_RADIUS_SMALL,

    // Spacing
    PADDING, PADDING_SMALL, PADDING_LARGE, ITEM_SPACING,

    // Typography
    FONT_SIZE_BASE, FONT_SIZE_SMALL, FONT_SIZE_HEADING,

    // Heights
    ROW_HEIGHT, LIST_ITEM_HEIGHT, PANE_HEADER_HEIGHT,

    // Accents
    BLUE_400, BLUE_500, SUCCESS_GREEN, WARNING_ORANGE, ERROR_RED,
};
```

---

## Evolution

This design system is living documentation. When adding new patterns:

1. First check if existing tokens cover your use case
2. If new tokens are needed, add them to `theme.rs` with semantic names
3. Update this document with the new pattern
4. Ensure consistency with existing components
