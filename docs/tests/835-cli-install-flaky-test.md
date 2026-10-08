# #835 — Daemon lock release across fork

## Cause and scope

The original installer test depended on closing its daemon-lock file to release an `flock`. A concurrent fork inherits the same open file description, so the lock remains held until the child closes that descriptor or executes. The third install then sees contention, returns success after updating only the CLI, and leaves `runnerd` at `OLD-BUILD`. [Apple's `flock` reference](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/flock.2.html) documents both inheritance across fork and explicit unlock through either descriptor.

This is also a product lifetime problem: the daemon holds `runnerd.lock` while spawning sessions and headless processes, and shutdown waits only boundedly for blocking tasks. A fork still awaiting exec when the daemon drops its lock can make an app replacement's installer see the released lock as contended. Installer completion, the shutdown lock probe, and release of the startup lock also previously relied on file closure. The shared `LockFile` now explicitly unlocks on drop, including early error returns, before closing its file. No retry or delay was added to installation.

## Deterministic reproduction and proof

Validated on 2026-10-08, macOS Darwin 25.6.0, Rust 1.97.1. The existing `installing_while_daemon_is_locked_updates_only_the_cli` test now forks after acquiring the simulated daemon lock. The child retains inherited descriptors in an async-signal-safe `pause` loop throughout both subsequent installs. Returning from `fork` in the parent guarantees descriptor inheritance has happened; no timing assumption or load generator is needed. The test still checks that the CLI updates while the daemon remains unchanged under the lock, then that the daemon updates immediately after the parent drops its lock while the child remains alive. Windows retains the daemon and companion assertions without the Unix-only fork fixture. The fixture kills and reaps its child on drop, including assertion failure.

Before changing product code at base `257937ca`, with only the fork fixture added, 20/20 runs failed at the original final daemon-content assertion: actual `OLD-BUILD`, expected `NEW-BUILD` (Cargo exit 101 each time). After the fix, the same loop passed 200/200 consecutive runs (exit 0 each time). Logs: `/tmp/runner-835-before.log` and `/tmp/runner-835-after.log`. No fork fixture child remained afterward.

Each iteration used this command:

```sh
cargo test --locked -p runner-core --profile ci \
  cli_install::tests::installing_while_daemon_is_locked_updates_only_the_cli -- --exact
```

The loop counted each command's direct exit status; the reproduction ran 20 iterations and the fixed code ran 200:

```sh
passes=0
failures=0
for ((run=1; run<=attempts; run++)); do
  cargo test --locked -p runner-core --profile ci \
    cli_install::tests::installing_while_daemon_is_locked_updates_only_the_cli -- --exact >> "$log" 2>&1
  code=$?
  printf 'run=%d exit=%d\n' "$run" "$code" >> "$log"
  if ((code == 0)); then
    ((passes += 1))
  else
    ((failures += 1))
  fi
done
printf 'attempts=%d passes=%d failures=%d\n' "$attempts" "$passes" "$failures" >> "$log"
```

## Validation

The first full workspace test run exited 101 with the shell's default 256-file-descriptor limit: four app failures, 66 daemon unit-test failures, and one CLI hook-adapter timeout. App and daemon failures include explicit `Too many open files` errors. Raising the limit to 4096 in the validation shell cleared the app and daemon failures; only the hook-adapter timeout remained (exit 101). Logs: `/tmp/runner-835-workspace-tests.log` and `/tmp/runner-835-workspace-tests-4096.log`.

The hook integration test passed on its own, including all 100 reports (exit 0; `/tmp/runner-835-hook-isolated.log`). It also passed in a full-workspace rerun limited to four test threads instead of this machine's default 18. That suggests contention around its three-second response deadline, independent of the deterministic #835 test, which passed all 200 proof runs without limiting threads or descriptors. The full-workspace rerun passed with exit 0.

| Final check | Exit | Log |
| --- | --- | --- |
| `ulimit -n 4096; RUST_TEST_THREADS=4 cargo test --locked --workspace --no-fail-fast --profile ci` | 0 | `/tmp/runner-835-workspace-tests-4096-threads4.log` |
| `cargo clippy --locked --workspace --all-targets --profile ci -- -D warnings` | 0 | `/tmp/runner-835-clippy.log` |
| `cargo clippy --locked --workspace --all-targets --profile ci --features updater -- -D warnings` | 0 | `/tmp/runner-835-clippy-updater.log` |
| `cargo fmt --all --check` | 0 | `/tmp/runner-835-fmt-check.log` |
| `git diff --check` | 0 | `/tmp/runner-835-diff-check.log` |

Windows validation awaits PR CI after working-tree review. No live smoke tests, development app, installed Runner files, or global agent configuration were touched. All reproduction, diagnostic, and validation processes finished; the fork fixture was verified absent with `ps`.
