# 733 — Move GPUI from the yanked gpui-ce 0.3.3 to gpui-pre

Tracking issue: [#733](https://github.com/yicheng47/runner/issues/733). Chore, P1, milestone 0.12. Baseline `main` at `63aa5a5` (2026-09-30). Target `gpui-pre =0.3.7`, the crates.io snapshot of `zed@1a28cff` published 2026-09-28. One mission, one PR, one commit.

## Why

Runner's `gpui` is `gpui-ce` 0.3.3, which was yanked from crates.io on 2026-08-28. It builds only because `Cargo.lock` pins it; a full `cargo update` fails today with `failed to select a version for the requirement gpui-ce = "^0.3"`. gpui-ce has since become a fork that diverges on purpose, and Zed's own crates.io `gpui` has been frozen at 0.2.2 since 2025-10-22. `gpui-pre` republishes upstream Zed's GPUI crates weekly and names the Zed commit each release was cut from; gpui-component and gpui-kit moved onto it in 0.7.0. The issue carries the full reasoning and the alternatives that were not chosen.

## What ships

Runner builds on `gpui-pre =0.3.7` on macOS and Windows with no intended user-visible change. Every `gpui-pre-*` crate sits on the same exact version, and no `gpui-ce` remains in the dependency graph. The docs name the new dependency and how to bump it. Upstream capabilities that arrive with it (`Window::request_attention`, AccessKit accessibility, the new IME hooks) become available but are not adopted here.

## Size

A source comparison of gpui-ce 0.3.3 against gpui-pre 0.3.7 on 2026-09-30, limited to what Runner uses, stands in for the spike the issue planned. It is not a compile, so its counts are a floor.

- 133 of the 134 GPUI names Runner imports still exist. `Corner` became `Anchor`.
- The `Element` trait only gained defaulted accessibility methods (`a11y_role`, `write_a11y_info`, `a11y_synthetic_children`), so the terminal element and the #726 text field compile against it unchanged.
- `EntityInputHandler` only gained defaulted IME methods (`paste`, `set_selected_text_range`, `text_length_utf16`, `text_input_configuration`, `text_input_editable_range`).
- `ShapedGlyph` and `Window::request_measured_layout` are unchanged, and `WrappedLine` changed only a private field.
- The test harness Runner relies on is all still there: `TestAppContext`, `VisualTestContext`, `add_window_view`, `simulate_resize`, `debug_bounds`, `debug_selector`, `NoopTextSystem`.

The breaks it found are all mechanical and listed in section 3. If the first compile reports far more than that, roughly a hundred errors outside the listed sites, stop and report before porting: the comparison missed something structural and the plan needs revisiting.

## Where it touches

### 1. Dependencies

In `crates/runner-app/Cargo.toml`, replace the four `gpui-ce` entries and add `gpui_platform`, which upstream split out of `gpui`. Keep the dependency names `gpui` and `gpui_platform` so the code reads like upstream. Target shape:

```toml
[target.'cfg(target_os = "macos")'.dependencies]
gpui = { package = "gpui-pre", version = "=0.3.7" }
gpui_platform = { package = "gpui-pre-platform", version = "=0.3.7", features = ["font-kit"] }

[target.'cfg(windows)'.dependencies]
# Keep gpui's defaults except windows-manifest; resync when its defaults change.
gpui = { package = "gpui-pre", version = "=0.3.7", default-features = false, features = ["font-kit", "wayland", "x11"] }
gpui_platform = { package = "gpui-pre-platform", version = "=0.3.7" }

[target.'cfg(windows)'.dev-dependencies]
gpui = { package = "gpui-pre", version = "=0.3.7", default-features = false, features = ["test-support"] }

[target.'cfg(target_os = "macos")'.dev-dependencies]
gpui = { package = "gpui-pre", version = "=0.3.7", features = ["test-support"] }
```

`gpui-pre`'s defaults are the same four as today's (`font-kit`, `wayland`, `x11`, `windows-manifest`), and `windows-manifest` still lives on `gpui`, so the Windows line keeps its shape. `gpui-pre-platform` has no default features, and on macOS it needs `font-kit`: without it GPUI falls back to a text system that lays text out but draws no glyphs. On Windows `font-kit` has no effect there. Check the feature names against the 0.3.7 manifests at the first compile.

### 2. Entry point

`crates/runner-app/src/main.rs:1242`: `Application::new().with_assets(Assets)` becomes `gpui_platform::application().with_assets(Assets)`. `Application::new()` no longer exists; `gpui_platform::application()` picks the platform and text backends for the host OS.

### 3. Mechanical API breaks

| Change in 0.3.7 | Sites on the baseline | Fix |
| --- | --- | --- |
| `Corner` renamed `Anchor` (`anchored().anchor(..)`, `Bounds::corner(..)`) | 7 lines in `ui/tooltip.rs`, `ui/button.rs`, `ui/menu.rs`, `surfaces/crews/popup.rs` | Rename, checking the variant names match |
| `Window::focus(&handle)` takes `cx`; `blur()` too | 22 `focus` calls in 14 files (`main.rs`, `surfaces/settings_page.rs`, `start_mission.rs`, `sidebar/project.rs`, `crews/{add_slot,create,slots,editor}.rs`, `roles/{create,edit,list}.rs`, `mission_workspace/{attach,feed,input}.rs`); 1 `blur` in `surfaces/sidebar/tests.rs` | Pass `cx` |
| `ScrollHandle::max_offset()` returns `Point<Pixels>`, not `Size<Pixels>` | 14 lines in `ui/{overlay,field,scrollbar,select}.rs`, `surfaces/{settings_page,start_chat}.rs`, `surfaces/mission_workspace/state.rs`, `surfaces/sidebar/tests.rs` | `.height` → `.y`, `.width` → `.x`; check the sign convention against the new `ScrollHandle` before assuming it is unchanged |
| `BoxShadow` gained `inset: bool` | 15 struct literals in 11 files under `ui/` and `surfaces/` | Add `inset: false` |
| `Menu` gained `disabled: bool` | 5 struct literals in `main.rs` `app_menus()` | Add `disabled: false` |

Paths are under `crates/runner-app/src/`. Anything the compiler reports beyond this table goes in the handoff with its fix.

### 4. Behavior that changes under the same code

These compile without edits but change what users see. Keep upstream's defaults unless the smoke test finds a problem, and name each in the PR body.

- **Inactive windows are throttled to about 30 fps.** `WindowOptions::inactive_frame_interval` defaults to `Some(33.333 ms)`, and Runner's `WindowOptions` in `main.rs` ends in `..Default::default()`, so it inherits the throttle. A terminal streaming agent output in a window that is not focused will animate at the lower rate. `None` turns the throttle off if the smoke test finds that worse.
- **Titlebar dragging.** `WindowOptions::app_owns_titlebar_drag` (macOS only) defaults to `false`, which keeps AppKit's native titlebar drag that Runner uses today with `appears_transparent: true` and `WindowControlArea::Drag`. Leave it `false`; upstream added it for windows that move themselves with `Window::start_window_move`.
- **Text rendering.** Windows and the app gained a `TextRenderingMode` that defaults to `PlatformDefault`. No change is expected, but glyph rendering is on the smoke list.
- **Focus.** Upstream changed how `Window` tracks focus, which is why `focus` and `blur` now take `cx`. Focus moves in modals and forms are on the smoke list.

### 5. Patches and forks

- **Zed's `[patch.crates-io]`** (`async-task`, `calloop`, `async-process`, `notify`, …) applies only inside Zed's workspace, and gpui-pre is built and tested against the crates.io versions. Add nothing to our root `Cargo.toml` unless the compile or the smoke test shows a need, and give each added entry a comment with its reason. Runner keeps its crates.io `notify`.
- **`zed-font-kit`** (macOS dev-dependency) already matches: `gpui-pre-macos` 0.3.7 depends on `zed-font-kit ^0.14.1-zed`.
- **`zed-reqwest`** (`runner-backend`, and `runner-app` on Windows for the updater) stays. gpui-pre replaced it with its own `gpui-pre-reqwest` fork, so Runner's copy no longer shares a build with GPUI's; switching Runner's HTTP client is a separate decision.

### 6. Build profiles

In the root `Cargo.toml`, rename `[profile.dev.package.gpui-ce]` and `[profile.ci.package.gpui-ce]` to `gpui-pre`, keeping their opt-levels. The platform crates (`gpui-pre-macos`, `gpui-pre-windows`, `gpui-pre-platform`) fall under the `"*"` rule at level 2 in dev and 1 in CI; give them their own entries only if a dev build is visibly slower than today's.

### 7. Docs

- `docs/arch/arch.md:98`, the stack table row: `gpui-pre` 0.3.7 (`zed@1a28cff`), a crates.io snapshot of upstream Zed's GPUI.
- `docs/arch/arch.md:911`, the API-break risk line: `gpui-pre` is pinned exactly and bumped deliberately; the terminal element and the IME integration remain the surfaces most exposed.
- Beside the stack row, the bump procedure: every `gpui-pre-*` pin moves together to one release, the release's crates.io description names its Zed commit, and that commit is the Zed source to read.
- `docs/tech/gpui-rendering.md:3` and `docs/tech/README.md:11,32`: point at `~/.cargo/registry/src/*/gpui-pre-0.3.7/src`, with the platform code in `gpui-pre-macos-0.3.7` and `gpui-pre-windows-0.3.7`. Check each claim the note makes about GPUI internals against the new source and correct any that moved.
- `docs/arch/windows.md:114`: the DXGI debug-probe sentence names gpui-ce; confirm the probe still exists in `gpui-pre-windows` and reword.
- `README.md:64,304` and `README.zh-CN.md:64,304`, in the same commit: Runner is built on Zed's GPUI through `gpui-pre`; the acknowledgements credit Zed and the gpui-pre publishers instead of gpui-ce.
- `docs/features/701-desktop-notifications.md:84,96`: the spec says gpui-ce 0.3.3 has no `request_attention`, so the dock bounce goes in `platform_ui`. gpui-pre has `Window::request_attention`; note that in the spec and leave the design choice to #701.

## Rules of the road

- Mission authorization follows AGENTS.md: branch `chore/733-gpui-pre` in `.worktrees/chore-733-gpui-pre` from `origin/main`; commits on it are authorized; everything, the brief included, is squashed into one commit before the push; the crew opens the PR against `main`, drives CI green on both platforms, and stops. No merge, no branch or worktree removal, no nightly or release.
- No behavior change beyond section 4. Do not adopt `request_attention`, accessibility or the new IME hooks, and do not change `inactive_frame_interval` or `app_owns_titlebar_drag` unless the smoke test asks for it.
- A site that needs more than a mechanical fix gets the smallest change that keeps today's behavior, and a line in the handoff.
- Do not launch the Runner app (`make run`); Jason smoke-tests.
- Gate imports and helpers that only `cfg(unix)` tests use behind `cfg(unix)`, or Windows Clippy goes red.

## Verification

- `make verify` on macOS, and CI green on macOS and Windows with the new `Cargo.lock` under `--locked`.
- `cargo update --dry-run` resolves without the yank error.
- `cargo tree -i gpui-ce` finds nothing, and `cargo tree -d` shows one copy of each `gpui-pre-*` crate.
- CI build times, cold and warm, against the last green `main` run, in the PR body.
- Smoke test by Jason on macOS, then on Windows on JASONPC: terminal rendering and scrolling, including agent output streaming into an unfocused window; IME, Chinese input in a chat and in a text field; window chrome, titlebar drag and double-click; the menu bar; tooltips and popovers (`Anchor`); focus moves in the role, crew and mission forms and modals; scrollbars and scroll-to-end (`max_offset`).

## Non-goals

Pulse, which is on a `gpui-ce` git rev and is a separate decision; adopting gpui-component or gpui-kit; accessibility; `request_attention`, which is #701's; any `gpui-pre` version other than 0.3.7; Linux; switching Runner's own HTTP client off `zed-reqwest`.
