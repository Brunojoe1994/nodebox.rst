# Repository Guidelines

## Project Structure & Module Organization
- `src/main/java` holds the core Java application (`nodebox.*` packages).
- `src/main/python` contains bundled Python node libraries and helpers.
- `src/main/resources` stores runtime assets and `version.properties`.
- `src/test/java` contains JUnit tests; `src/test/python` and `src/test/clojure` hold language fixtures.
- `libraries/` and `examples/` ship built-in node libraries and example projects.
- `res/`, `artwork/`, and `platform/` contain assets and platform-specific launchers.
- `build/` and `dist/` are generated outputs; avoid manual edits.
- `build.xml` (Ant) and `pom.xml` (Maven deps) define the build and test pipeline.

## Build, Test, and Development Commands
- `ant run` builds and launches NodeBox.
- `ant test` compiles and runs JUnit tests; XML reports land in `reports/`.
- `ant generate-test-reports` renders HTML reports from `reports/TEST-*.xml`.
- `ant dist-mac` / `ant dist-win` create packaged apps in `dist/`.
- `ant clean` removes build artifacts.

Prereqs: Java JDK and Apache Ant are required; Maven is used for dependency resolution (see `README.md`).

## Coding Style & Naming Conventions
- Java: 4-space indentation, braces on the same line, and standard Java naming (classes `UpperCamelCase`, methods `lowerCamelCase`, constants `UPPER_SNAKE_CASE`).
- Python: follow existing API naming (many public helpers are `lowerCamelCase`), keep function signatures consistent with current modules.
- Keep edits localized and match the surrounding file’s formatting and ordering.

## Testing Guidelines
- JUnit is the primary test framework; tests are discovered by `**/*Test.class` in `src/test/java`.
- Name new Java tests `SomethingTest.java` and keep them close to the package they cover.
- Run `ant test` before shipping changes that affect core behavior or UI flows.

## Branching Strategy
- **Use `rewrite-in-rust` as the main branch.** All new development and PRs should target this branch.
- **NEVER commit or merge directly into `master`.** The `master` branch exists for legacy reasons only and should not be modified.
- Create feature branches from `rewrite-in-rust` and merge back into it.

## Commit & Pull Request Guidelines
- Recent history favors short, sentence-style commit messages (e.g., "Use Ctrl key on Windows."). Keep messages concise and specific.
- PRs should describe the user-visible change, list test commands run, and include screenshots or recordings for UI updates.
- Link relevant issues or tickets when applicable.

## Notes for Contributors
- Versioning lives in `src/main/resources/version.properties`; update it when preparing a release build.
- **NEVER modify the Java code** (`src/main/java`). The Java codebase is legacy and read-only; use it only as a reference. All new development happens in the Rust crates under `crates/`.

## UI Design System (Rust GUI)

The NodeBox GUI uses a design token system inspired by Rerun. All design constants are centralized in `crates/nodebox-gui/src/theme.rs`.

### Color Tokens
Always use semantic color tokens from `theme.rs` instead of hardcoded hex values:

**Backgrounds:**
- `PANEL_BG` - Main panel background (dark)
- `TOP_BAR_BG` - Title bar background
- `TAB_BAR_BG` - Tab bar background
- `HOVER_BG` - Hover state background
- `SELECTION_BG` - Selection highlight (blue)

**Text:**
- `TEXT_STRONG` - Active/primary text (brightest)
- `TEXT_DEFAULT` - Regular body text
- `TEXT_SUBDUED` - Secondary/muted text
- `TEXT_DISABLED` - Disabled text

**Widgets:**
- `WIDGET_INACTIVE_BG` - Default widget background
- `WIDGET_HOVERED_BG` - Hovered widget background
- `WIDGET_ACTIVE_BG` - Active/pressed widget background
- `BORDER_COLOR` - Standard border color

**Accents:**
- `BLUE_400` / `BLUE_500` - Primary accent (selection, links)
- `SUCCESS_GREEN`, `WARNING_ORANGE`, `ERROR_RED` - Status colors

### Spacing (4px Grid)
All spacing should be multiples of 4px:
- `PADDING_SMALL` = 4px
- `PADDING` = 8px (standard)
- `PADDING_LARGE` = 12px
- `VIEW_PADDING` = 12px (panel margins)
- `ITEM_SPACING` = 8px
- `INDENT` = 12px (hierarchy indent)

### Typography
- `FONT_SIZE_BASE` = 12px (body text)
- `FONT_SIZE_SMALL` = 11px
- `FONT_SIZE_HEADING` = 16px

### Layout Heights
- `TOP_BAR_HEIGHT` = 28px
- `TITLE_BAR_HEIGHT` = 24px
- `LIST_ITEM_HEIGHT` = 24px
- `ROW_HEIGHT` = 22px

### Corner Radii
- `CORNER_RADIUS` = 6px (panels, windows)
- `CORNER_RADIUS_SMALL` = 4px (buttons, widgets)

### Best Practices
1. **Use tokens, not literals** - Import from `crate::theme` and use named constants
2. **Follow the 8px grid** - All margins and padding should align to the grid
3. **Maintain text hierarchy** - Use `TEXT_STRONG` for emphasis, `TEXT_SUBDUED` for secondary info
4. **Hover feedback** - Add 2px expansion + background color change on hover
5. **Selection states** - Use `SELECTION_BG` with `BLUE_400` stroke (2px)
6. **Call `theme::configure_style()`** - This is done in app startup; don't override global styles

## API Design & Backwards Compatibility

### Property Names
When renaming properties in the API, keep internal storage names for backwards compatibility:
- Internal property: `canvasWidth`, `canvasHeight` (for file format compatibility)
- External API: `width()`, `height()` (cleaner public interface)

```rust
// In NodeLibrary
pub fn width(&self) -> f64 {
    self.properties
        .get("canvasWidth")  // Internal name for backwards compat
        .and_then(|s| s.parse().ok())
        .unwrap_or(1000.0)
}
```

### Centered Coordinate System
The canvas uses a centered coordinate system where:
- Geometry is positioned relative to the origin (0, 0)
- Canvas extends from `-width/2` to `+width/2` and `-height/2` to `+height/2`
- This matches standard graphics conventions and simplifies transforms

**For SVG export:**
```rust
// Use centered viewBox
let half_w = width / 2.0;
let half_h = height / 2.0;
format!(r#"viewBox="{} {} {} {}""#, -half_w, -half_h, width, height)
```

**For PNG export with tiny_skia:**
```rust
// Center the transform
let transform = Transform::from_translate(width as f32 / 2.0, height as f32 / 2.0);
```

## Screen-space Rendering

For UI elements that should remain constant size regardless of zoom (handles, borders, guides):
- Apply zoom transform to world coordinates first
- Use fixed pixel values for stroke width and sizes after transformation

```rust
// Canvas border with constant 1px stroke
let screen_top_left = self.pan_zoom.world_to_screen(top_left, center);
let screen_bottom_right = self.pan_zoom.world_to_screen(bottom_right, center);
painter.rect_stroke(canvas_rect, 0.0, Stroke::new(1.0, border_color));
```

## Rust Dead Code Warnings

### Module-level suppression
For WIP modules or test utilities where many items may be unused:
```rust
#![allow(dead_code)]
```

### Item-level suppression
For individual items that are intentionally kept for future use or API completeness:
```rust
#[allow(dead_code)]
pub fn some_future_method(&self) { ... }
```

### Test-only methods
Methods marked `#[cfg(test)]` still generate warnings if unused within tests:
```rust
#[cfg(test)]
#[allow(dead_code)]
pub fn new_for_testing() -> Self { ... }
```

## egui Migration Notes

### Deprecated methods
- `ui.allocate_ui_at_rect(rect, |ui| { ... })` is deprecated
- Use `ui.allocate_new_ui(egui::UiBuilder::new().max_rect(rect), |ui| { ... })` instead

## Build Commands

### Excluding problematic crates
The `nodebox-python` crate has pyo3 dependencies that may cause build issues. Exclude it when not needed:
```bash
cargo build --workspace --exclude nodebox-python
cargo test --workspace --exclude nodebox-python
```

### Running specific crates
```bash
cargo run -p nodebox-gui          # Run the GUI
cargo run -p nodebox-cli          # Run the CLI
cargo test -p nodebox-core        # Test specific crate
```
