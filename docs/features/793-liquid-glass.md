# Liquid glass appearance

Tracking issue: [#793](https://github.com/yicheng47/runner/issues/793). Priority: P2. Status: draft; design decisions open.

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

References: [material tokens](https://github.com/cristicretu/diri/blob/55cfa75c89695f3fcb2d52c759e3f961f80b536b/diri/crates/diri-ui/src/tokens.rs), [floating panels](https://github.com/cristicretu/diri/blob/55cfa75c89695f3fcb2d52c759e3f961f80b536b/diri/crates/diri-app/src/floating.rs), [AppKit panel behavior](https://github.com/cristicretu/diri/blob/55cfa75c89695f3fcb2d52c759e3f961f80b536b/diri/crates/diri-app/src/macos/floating_panel.rs), and the archived Runner backdrop spec at `docs/features/archive/557-window-backdrop.md`.

## Implementation phases

1. Design and compare representative chat, sidebar and floating-control frames; settle material densities, settings and platform scope.
2. Validate the blur/compositing approach in Runner's pinned GPUI version and measure frame time, memory and idle CPU against the solid appearance.
3. Implement the approved material treatment and settings, then validate native interactions and platform fallbacks.

## Verification

- Light and dark themes remain readable over bright, dark and busy wallpapers, including terminal output, CJK/IME composition and selection.
- Menus and popovers retain correct focus, keyboard navigation, anchoring and dismissal across resizing, multiple windows, fullscreen and display changes.
- Appearance changes apply consistently to existing and newly opened windows; solid appearance and Reduce Transparency behavior are verified.
- Measure rendering and memory impact during terminal streaming, resizing and idle use, recording any regressions before shipping.
- Run runner-app tests and workspace Clippy; record macOS visual validation and Windows compatibility evidence.
