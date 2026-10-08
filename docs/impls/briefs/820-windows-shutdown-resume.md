# 820 — Windows restart or sign-out ends runnerd without stamping sessions for resume

Fix [#820](https://github.com/yicheng47/runner/issues/820). Jason requested this mission on 2026-10-08 after hitting the bug himself. Work only in `C:\Users\ROG\repos\yicheng47\runner\.worktrees\fix-820-windows-shutdown-resume`, on the existing branch `fix/820-windows-shutdown-resume`, created from `origin/main` at `8a7e40c`. The root checkout stays on main. Do not create another branch or worktree, and do not share a Cargo target directory.

## The bug, now confirmed

On Windows, runnerd hears about sign-out and shutdown only through console control events: `os_shutdown` in `crates/runner-daemon/src/daemon/server.rs` (about line 816) waits on `ctrl_close`, `ctrl_logoff` and `ctrl_shutdown`. When it fires, the loop breaks and the teardown (about line 228) calls `mark_running_for_resume_on_launch` so the next launch resumes the sessions.

`SetConsoleCtrlHandler`'s documentation says a console process that loads `user32.dll` or `gdi32.dll` is treated as a GUI application and never receives `CTRL_LOGOFF_EVENT` or `CTRL_SHUTDOWN_EVENT`. On Jason's PC on 2026-10-08, `tasklist /m` showed the running `runnerd.exe` loads both. So Windows ends runnerd at restart without the stamp, the next boot's stale-row cleanup (`cleanup_stale_running`, `repo/session.rs` about line 445) demotes the sessions, and nothing resumes. Jason has seen exactly this after a restart.

## Read first

- `AGENTS.md`, including Worktrees and Crew Missions.
- Issue #820.
- `crates/runner-daemon/src/daemon/server.rs`: the accept loop and its `signal` arm (about line 203), the teardown and its `SHUTDOWN_TIMEOUT` (8 s, `runner-core/src/daemon_process.rs` line 14), and `os_shutdown`.
- `crates/runner-daemon/src/repo/session.rs`: `mark_running_for_resume_on_launch` and `cleanup_stale_running`, and their tests.
- `crates/runner-daemon/src/daemon/boot.rs`: `stop_running_sessions_on_quit`, the app's Quit path that already does this right.
- `crates/runner-core/src/daemon_process.rs` about line 109: runnerd's Windows spawn flags (`CREATE_NO_WINDOW | CREATE_NEW_PROCESS_GROUP`, breakaway from the app's job).
- `docs/arch/windows.md` and whatever doc describes runnerd's lifecycle and quit choices (grep `docs/arch` for `runnerd`).

## Deliverables

1. **Hear the GUI-app shutdown messages.** On Windows, runnerd must react to `WM_QUERYENDSESSION` / `WM_ENDSESSION` (covering both restart/shutdown and sign-out) and run the same stamp-and-teardown path as the console events. The expected shape is a hidden top-level window with its own message loop on a dedicated thread, feeding the existing `os_shutdown` future, with the console events kept for the cases that still deliver them. A message-only window (`HWND_MESSAGE`) does **not** receive these broadcasts; use a real, never-shown top-level window. Only `Win32_UI_WindowsAndMessaging` is already a `windows-sys` feature in `crates/runner-daemon/Cargo.toml`; add others only if needed.
2. **The stamp must finish before Windows kills the process.** The process can be terminated as soon as the `WM_ENDSESSION` handler returns, and Windows gives a hung app only about 5 s. So the window procedure must not return from `WM_ENDSESSION` (with `wParam` true) until `mark_running_for_resume_on_launch` has committed. Killing the PTYs is secondary: the OS ends them anyway. Decide whether the full 8 s teardown can run inside that window or whether the stamp should commit first and the rest be best-effort, and say which in the handoff. Consider `ShutdownBlockReasonCreate` only if the stamp genuinely needs more time; do not block shutdown indefinitely.
3. **Do not lose the stamp to a race with dying children.** At shutdown Windows is also ending the agents' processes and their ConPTY hosts. If runnerd's session-exit handling marks a session stopped before the stamp runs, the stamp finds nothing to mark. Check the exit path and make sure a session that was running when the shutdown began is still stamped for resume. `SetProcessShutdownParameters` (a higher level is notified earlier) is one lever; evaluate it and say what you chose.
4. **Tests.** A Windows-only test that sends `WM_QUERYENDSESSION` and `WM_ENDSESSION` to the hidden window and proves the shutdown future resolves and the stamp runs before the handler returns, plus whatever covers deliverable 3 without a real reboot. Gate Windows-only test code and its imports with `#[cfg(windows)]`, and anything used only by `cfg(unix)` tests with `#[cfg(unix)]`; Windows CI has repeatedly broken on ungated imports. macOS and Linux behavior must not change.
5. **Docs.** Update the runnerd lifecycle doc in `docs/arch/` to state how runnerd hears Windows shutdown and sign-out, and why the console events alone are not enough.

Keep the change scoped to this. No app UI changes, no quit-dialog changes, no README change unless a README states the old behavior (`README.md` and `README.zh-CN.md` change together).

## Boundaries

Crews never run the dev app, restart Windows, sign out, or stop Jason's running runnerd or Runner. Do not run `runner-dev` or `runner` commands that start, stop or kill sessions or the daemon. Jason does the live restart test. Do not start extra agents, crews or subagents.

## Review, verification and authorization

The coder owns implementation and checks. The reviewer waits for an explicit Runner handoff, then reviews the full branch diff against #820 with must-fix findings first and file:line pointers. Focus on: the window really receives the broadcasts (top-level, not message-only, a message loop that actually pumps); the stamp committing before `WM_ENDSESSION` returns; no deadlock between the window thread and the tokio runtime or the teardown; the child-exit race; console events still handled; no change on Unix. Iterate until the reviewer posts `NO REMAINING MUST-FIX ISSUES`.

Run `cargo test --locked -p runner-daemon --profile ci` and `cargo test --locked -p runner-core --profile ci`, workspace Clippy with warnings denied (`cargo clippy --locked --workspace --all-targets --profile ci -- -D warnings`), `cargo fmt --all --check`, and `git diff --check`. This machine is Windows; macOS updater Clippy runs in CI. Record the exact commands and exit codes.

After a clean review, Jason authorizes squashing all work on this branch, this brief included, into one commit on top of current `origin/main` with a subject that names the fix (for example `fix(daemon): stamp sessions for resume on Windows restart and sign-out`), pushing `fix/820-windows-shutdown-resume`, and opening a PR against main whose body says `Fixes #820`. If main has moved, rebase; never merge main into the branch. Review or CI fixes after the push are amended into the same commit and pushed with `git push --force-with-lease`. Drive CI green on macOS and Windows. Do not merge, delete the branch or worktree, or cut a nightly or release. Final Runner handoff: PR URL, what changed, tests and exit codes, CI result, the reviewer's verdict, and Jason's live test: with two chats open and Runner left running, restart Windows (and separately sign out); on the next launch both chats resume, and `runnerd.log` shows the shutdown message and the stamp before the restart. Then stand by.
