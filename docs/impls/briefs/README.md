# Mission briefs

A brief is the goal a crew mission starts from: where to work, what to read, what to build, how to verify, and what the crew is authorized to do. It lands on `main` with the mission's pull request, folded into one of its commits. Shipped briefs are pruned in the next release's docs sweep; the ones below stay as references for writing the next one, and every other brief is in git history (the 2026-09-30 prune kept these seven of 59).

Start from the brief closest to the mission's shape:

| Brief | Shape | What to take from it |
|---|---|---|
| [752](./752-usage-pill-runtimes.md) | A small bug fix | The whole skeleton at its shortest: worktree, the bug, Read first with line hints, numbered deliverables, boundaries, and the current authorization section |
| [735](./735-start-chat-modal.md) | A UI feature from a design | Design PNGs in `target/design-ref/`, the spec winning over the brief, a reviewer checklist, and the smoke list for the PR body |
| [575](./575-live-cwd.md) | Spec first, then code | A spec the reviewer passes with `SPEC OK` before any code, and probes in throwaway shells |
| [644](./644-antigravity-runtime.md) | A new agent runtime | Phases taken from the spec's site list, and reviewer checks that keep the other runtimes' argv unchanged |
| [747](./747-antigravity-followups.md) | Work that probes a live agent CLI | Explicit probe boundaries: what may run, where, and what must stay untouched |
| [644 continuation](./644-antigravity-runtime-continuation.md) | Resuming a mission on an existing PR | The starting state after a rebase, and updating the open PR instead of opening another |
| [582](./582-split-test-files.md) | A move with no behavior change | The split pattern, the cut table, and verification that proves only code moved |

The authorization sections in 575, 644 and 582 predate the commit rule in [`AGENTS.md`](../../../AGENTS.md) (Crew Missions); copy that section from 752 or 735 instead. Every brief still carries the standing lines: work only in the named worktree, crews never run the dev app, gate imports used only by `cfg(unix)` tests with `#[cfg(unix)]`, and stop at an open PR with CI green on macOS and Windows.
