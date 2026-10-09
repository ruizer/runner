# Liquid glass appearance

Tracking issue: [#793](https://github.com/yicheng47/runner/issues/793). Priority: P1, 0.14 (moved from 0.13 on 2026-10-07), and the release blocker of 0.14.0 since 2026-10-09, because it is a new design primitive. It headed 0.14 until file preview and code review ([#634](./634-file-preview-and-review.md)) took that place on 2026-10-09, and it still goes first so #634 is designed in it. Status: design signed off by Jason on 2026-10-09; implementation next.

Design: [Liquid glass and files](../../design/specs/793-634-glass-and-files.pen), shared with file preview and code review ([#634](./634-file-preview-and-review.md)) so its panel is designed in the new chrome. Selected direction: A · Frosted chrome. Primary frames: `DJM0c` (dark) and `P1ZnQS` (light); shared sidebar: `N4yXC`.

Layout: only the sidebar is chrome. Everything to its right, the chat header, the panes and the side panel, sits in one card inset 8 px from the window, and the columns inside it are split by plain 1 px dividers that run from the card's top to its bottom, so a header divider can never drift from the seam below it. The layout is universal: Glass and Solid share it on macOS and Windows alike, and Glass only changes the material of the sidebar and the frame around the card. On Windows the card sits below the existing 32 px title bar with the caption buttons, which takes the sidebar's color so it reads as part of the frame. Decided by Jason on 2026-10-09, replacing a rounded surface per section.

Defaults: Glass on macOS, Solid on Windows. Windows offers only Solid in 0.14, so its Appearance settings leave out the Window material row; Mica, Windows 11's own material, is the candidate for a later Windows option. Decided by Jason on 2026-10-09.

## Motivation

Give Runner a coherent liquid glass feel inspired by [Diri](https://github.com/cristicretu/diri): blurred backdrops, layered translucent chrome, subtle edge highlights and polished floating surfaces, while keeping terminal text easy to read.

Jason requested this after comparing Diri with Runner on 2026-10-03. This revisits the visual direction of [#557](https://github.com/yicheng47/runner/issues/557), which was closed as not planned, with a broader focus on the material treatment rather than a uniform window-opacity setting.

## Scope

- Design the treatment in Pencil before implementation, using a feature-scoped file under `design/specs/` and Runner's existing product components. Explore light and dark themes over varied desktop backgrounds.
- Start with macOS sidebar, window chrome, menus and popovers. Keep terminal surfaces dense enough for long coding sessions and preserve explicit TUI cell backgrounds, selections and IME legibility.
- Use distinct material densities for persistent chrome, work surfaces and floating controls, with restrained borders, highlights and motion.
- Provide a solid appearance option and account for the system's Reduce Transparency preference. Settle the default and any transparency controls during design.
- Preserve Windows functionality; evaluate an appropriate native material treatment separately rather than assuming macOS blur behavior transfers directly.

## Technical investigation

Diri uses GPUI's `WindowBackgroundAppearance::Blurred` with AppKit backdrop blur and translucent theme tokens. Its glass menus open in separate non-activating blurred panels because GPUI cannot blur behind an individual element. Review the positioning, focus, dismissal, accessibility and multi-window costs before choosing that approach for Runner. The requested appearance does not require adopting `NSGlassEffectView` or Diri's GPUI fork; verify what Runner's pinned GPUI version supports first.

On 2026-10-09 Jason chose to ship glass menus and popovers with this feature rather than after it, so they open in their own blurred panels and the costs above are handled in the implementation, not deferred.

References: [material tokens](https://github.com/cristicretu/diri/blob/55cfa75c89695f3fcb2d52c759e3f961f80b536b/diri/crates/diri-ui/src/tokens.rs), [floating panels](https://github.com/cristicretu/diri/blob/55cfa75c89695f3fcb2d52c759e3f961f80b536b/diri/crates/diri-app/src/floating.rs), [AppKit panel behavior](https://github.com/cristicretu/diri/blob/55cfa75c89695f3fcb2d52c759e3f961f80b536b/diri/crates/diri-app/src/macos/floating_panel.rs), and the archived Runner backdrop spec at `docs/features/archive/557-window-backdrop.md`.

## Implementation phases

1. Design and compare representative chat, sidebar and floating-control frames; settle material densities, settings and platform scope.
2. Validate the blur/compositing approach, including the floating panels for menus and popovers, in Runner's pinned GPUI version.
3. Implement the approved material treatment and settings, then validate native interactions and platform fallbacks.

## Verification

- Light and dark themes remain readable over bright, dark and busy wallpapers, including terminal output, CJK/IME composition and selection.
- Menus and popovers retain correct focus, keyboard navigation, anchoring and dismissal across resizing, multiple windows, fullscreen and display changes.
- Appearance changes apply consistently to existing and newly opened windows; solid appearance and Reduce Transparency behavior are verified.
- Frame rate, memory and CPU are not measured in this feature; the UI performance tests ([#831](https://github.com/yicheng47/runner/issues/831)) cover them after the glass change. Jason dropped them from this feature's QA on 2026-10-09.
- Run runner-app tests and workspace Clippy; record macOS visual validation and Windows compatibility evidence.
