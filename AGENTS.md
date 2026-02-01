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

## Commit & Pull Request Guidelines
- Recent history favors short, sentence-style commit messages (e.g., “Use Ctrl key on Windows.”). Keep messages concise and specific.
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

### Spacing (8px Grid)
All spacing should be multiples of 8px:
- `PADDING` = 8px (standard)
- `PADDING_SMALL` = 4px
- `PADDING_LARGE` = 12px
- `VIEW_PADDING` = 12px (panel margins)
- `ITEM_SPACING` = 8px
- `INDENT` = 14px (hierarchy indent)

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
