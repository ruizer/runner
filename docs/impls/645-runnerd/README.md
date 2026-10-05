# runnerd — program record

Implementation record for [feature 645](../../features/645-session-host.md) ([#645](https://github.com/yicheng47/runner/issues/645)). The spec says *what*; this directory says *how, in what order, and what has landed*. This file is the condensed state and the decisions that bind. [plan.md](plan.md) has the phases, the four phase 1 missions in detail and the #709 evaluation; [impl_log.md](impl_log.md) is the dated log. Briefs go in [`../briefs/`](../briefs/) as `645-m{n}-{slug}.md`.

## Status (2026-10-05)

Planning. The spec and this plan landed on `main` on 2026-10-05, after Jason's review. No mission has started. Next, on `main`: the channel infrastructure and the downgrade guard. Then the umbrella branch is cut and 1a starts.

The phases: (1) the local daemon, four missions, which gates 0.13.0; (2) updates leave agents running; (3) remote machines over ssh; (4) the Windows PC.

## Decisions that bind

- **The state owner runs without the UI.** `runnerd` runs `AppCore`, and the app, the CLI and any later client are its clients (spec decision 1).
- **The live terminal path is unchanged.** The agent's bytes reach the app untouched, the mirror parses them with the same `alacritty_terminal` code the daemon uses, and everything except input and output stays local to the app. Snapshots happen only on reconnect.
- **The upstream alacritty event loop is not the engine**, because a daemon must forward raw bytes and it does not expose them (plan, "#709 does not go first").
- **The boundary is a crate boundary.** A new `protocol` module in `runner-core` holds everything that crosses the socket (1a). The app loses its normal dependency on the backend (1c), and `runner-backend` is renamed `runner-daemon` (1d).
- **The four phase 1 missions run in order.** 1a and 1b run with no other `runner-app` mission in flight.
- **No downgrade, fix forward.** Jason's Mac takes every `nightly-runnerd` cut from the first one, with his data directory backed up first. Problems are fixed on the umbrella. Every build he could go back to refuses to start while `runnerd` runs. Checks A to E (plan, "The checks before landing") gate the landing on `main`, not the install.
- **Phase 1 lives on `feat/645-runnerd` and ships on `nightly-runnerd`.** Missions merge into the umbrella; the umbrella follows `main` by rebase, adds no schema change, and lands on `main` once, after the daily-driving gate. Channel infrastructure lands on `main` first.

## Open

- Whether 1a lands on `main` (proposed) or on the umbrella, and how long the daily-driving gate runs (proposed: one week).
- Whether to close #795 as covered by the spec. #709 was closed on 2026-10-05.
- The issue body's Shape section and title predate `runnerd`. The spec file is still named `645-session-host.md`.
