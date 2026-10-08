# #845 — CI nextest validation

## Scope and timing method

Both CI jobs use the pinned prebuilt cargo-nextest 0.9.148 with the `ci` Cargo profile, no retries, and the same Cargo timing artifact. Check names and all existing formatting, Clippy, updater, installer, resource, and lockfile checks are retained. `make test` still uses Cargo's test harness.

The issue's baseline is macOS run 37769556810 (Test 8m32s, compile 1m47s, test run 6m44s) and Windows run 37765871789 (Test 13m13s, compile 3m18s, test run 9m52s). Compare nextest's reported execution duration separately from the Test step and whole job. The cache only saves on main. `Cargo.toml` is unchanged, so these changes preserve its cache key; each CI run's actual cache restore determines whether it is cold or warm. The mission reserves the two warm end-to-end target measurements per platform for main after merge; this PR records observed times and cache warmth without claiming that gate is met.

## Shared-state audit

| State | What it guards in tests | nextest verdict |
| --- | --- | --- |
| `runner-app` `THEME_LOCK` | Active theme and thread-local captured GPUI fills | Process-local; keep the lock for plain Cargo, no group needed |
| Codex `CONFIG_LOCK` | Trust config read/modify/write; every test supplies a unique `TempDir` config; mocked spawns are a no-op | No shared path across tests; keep the lock, no group needed |
| Copilot `CONFIG_LOCK` | Trust config read/modify/write; explicit temporary configs and temporary override homes; mocked spawns without an override are a no-op | No shared path across tests; keep the lock, no group needed |
| Antigravity `SETTINGS_LOCK` | Settings read/modify/write in each test's temporary home; spawn tests use thread-local `with_conversation_home` | No shared path across tests; keep the lock, no group needed |
| `DELIVERY_LOG`, `STUB_FETCHES`, claimed rollouts, config/conversation fixture overrides | In-process logger, fetch capture, claim set, and thread-local fixtures | Isolated by process |
| Sockets, pipes, HTTP listeners, startup lockfiles | Paths under unique temporary roots, ULID pipe names, or OS-assigned port 0 | No fixed shared listener or lock path |
| Environment | Test fixtures pass overrides to child `Command`s or thread-local config maps; production environment cleanup occurs in child executables | Process-local |
| Daemon golden files and failure artifacts | Distinct golden name per test; normal CI only reads goldens, failure artifacts are distinct per name | No concurrent writer to the same file |
| CLI goldens | Read one golden file; the manifest/update test is the sole whole-file writer, other cases return during updates; an in-process lock preserves plain Cargo concurrency | Independent nextest case processes; one update owner |
| `runner-cli` `daemon_process`, `golden`, `hook_process` | Isolated daemon/CLI processes and shell fixtures, with timing-sensitive IPC | One shared `real-daemon` group capped at two tests, fitting the small hosted runners; the daemon fixture also holds an in-process lock for plain Cargo |

No workspace Rust documentation contains a fenced code example. The plain Cargo run verifies every doctest target reports zero tests.

## Slow tests

The CLI table exposes 69 independent command tests (each preserves text and JSON variants), plus inside-mission, offline, help, and manifest/update tests. The manifest compares every context/argv heading in order, detecting missing, extra, and reordered cases. A regression test exercises missing and reordered manifests. Each command still gets a new isolated fixture.

`recording_split_points` is split into 19 per-recording tests with the identical event-boundary offsets and 200 seeded random offsets. An inventory test requires every `.ndjson` fixture to have a test. Splitting preserves the Cargo cache key and avoids an optimization change that would force every PR run to compile cold.

Before changing the database path, an isolated harness linked against this worktree's CI dependencies measured the exact connection initialization: `SqliteConnectionManager::connect()` returned `NotADatabase` in 860 µs, while `Pool::builder().max_size(8).build(manager)` failed after 30.005193 s. r2d2 retries the failing initial connection until its default initialization deadline. Startup now probes the same manager synchronously before building the pool; the default 30 s checkout timeout is unchanged. Unit coverage asserts a SQLite error within five seconds and a valid pool's 30 s timeout. The real-daemon regression requires both client startup paths to report failure within ten seconds.

## Golden update

Update the entire current-platform section and shared section in deterministic table order, preserving the other platform's status section:

```sh
UPDATE_RUNNER_GOLDENS=1 cargo nextest run --locked -p runner-cli --test golden --cargo-profile ci built_cli_goldens_manifest_and_update
```

Running the whole golden binary with `UPDATE_RUNNER_GOLDENS=1` also works; only the manifest/update test writes. Running the update twice and comparing the file verifies determinism.

## Local commands and results

No development app or real agent is launched; the integration tests own their isolated daemons and shell fixtures. The baseline plain Cargo run failed `persistent_hook_reporter_delivers_to_real_isolated_daemon` with the existing three-second hook response timeout. Nextest's first full run passed every real-daemon test but exposed a GPUI redraw test racing a real worker event after advancing only the mock clock; that test now flushes the worker and waits for the revision and render with a real deadline, retaining every assertion. The next full nextest run passed all tests in 58.442 s. The first post-change plain Cargo run reproduced the baseline hook timeout, motivating the fixture's in-process lock.

Jason also requested investigation of [Windows job 113292821886](https://github.com/yicheng47/runner/actions/runs/37771766043/job/113292821886) on main at `e5158b5e`. Its only failing target was `runner-app --bin Runner`: the same redraw test failed at line 146 with terminal-wake revisions `1 == 1`. Clippy and the installer check passed, and macOS passed in the same run. This corroborates the locally reproduced timing assumption addressed by this branch; Windows verification of the fix belongs to this PR's CI.

The baseline inventory has 2,054 tests across 23 binaries, including six ignored tests. The new inventory has 2,147 tests across the same binaries and identical ignored sets. No non-ignored test is filtered out. Only `built_cli_goldens` and `recording_split_points` disappear, replaced by their split tests. The net increase of 93 comprises 73 CLI tests, 19 recording tests, and one database regression. A subprocess rerun in a Unix daemon unit test prints an extra passing summary; raw Cargo log totals include that nested test and should be reduced by one when comparing to nextest.

Local CI-profile numbers on 2026-10-08: the old golden binary took 110.85 s, the old snapshot binary took 66.13 s, and the old daemon-process binary took 64.93 s. In the clean nextest run, the slowest split recording took 23.476 s, and the invalid-database regression completed both startup paths in 1.460 s. These are local execution comparisons, not hosted job-time claims.

The latest plain Cargo run passed all 2,141 non-ignored tests and left the same six ignored (raw logs show 2,142 passes because of the Unix subprocess rerun). Its daemon-process, golden, and snapshot binaries took 56.11 s, 38.48 s, and 23.54 s respectively. Workspace Clippy and Clippy with updater passed with warnings denied.

| Command | Exit code |
| --- | --- |
| `curl -LsSf https://get.nexte.st/0.9.148/mac -o /tmp/845-nextest.tar.gz` and `tar zxf /tmp/845-nextest.tar.gz -C ~/.cargo/bin` | 0 |
| `cargo nextest --version` (0.9.148), `cargo nextest run --help` (forwards `--timings`) | 0 |
| Baseline `cargo test --locked --workspace --no-fail-fast --profile ci --timings` | 101; existing hook timeout |
| First `cargo nextest run --locked --workspace --no-fail-fast --cargo-profile ci --timings` | 100; redraw timing failure |
| Next `cargo nextest run --locked --workspace --no-fail-fast --cargo-profile ci --timings` | 0; 2,141 passed, six skipped, 58.442 s |
| Final `cargo nextest run --locked --workspace --no-fail-fast --cargo-profile ci` | 0; 2,141 passed, six skipped, 56.013 s |
| First `cargo test --locked --workspace --no-fail-fast --profile ci` | 101; existing hook timeout before the fixture lock |
| Latest `cargo test --locked --workspace --no-fail-fast --profile ci` | 0; 2,141 passed, six ignored |
| `cargo clippy --locked --workspace --all-targets --profile ci -- -D warnings` | 0 |
| `cargo clippy --locked -p runner-app --features updater --all-targets --profile ci -- -D warnings` | 0 |
| `cargo fmt --all --check` | 0 |
| `git diff --check` | 0 |
| `cargo nextest list --locked --workspace --cargo-profile ci --message-format json` and baseline binary `--list --format terse` inventory comparison | 0; all 23 binaries and unchanged tests retained |
| `cargo nextest show-config test-groups --locked --workspace --cargo-profile ci` | 0; all three integration binaries assigned to the two-thread group |
| `UPDATE_RUNNER_GOLDENS=1 cargo nextest run --locked -p runner-cli --test golden --cargo-profile ci built_cli_goldens_manifest_and_update` (twice) | Both 0; 39.038 s and 39.355 s; both byte-identical to the committed golden |
| `cmp crates/runner-cli/tests/goldens/cli.txt /tmp/845-original-golden.txt` (after each update) | Both 0 |

All 100 non-ignored tests in the real-daemon group passed in each of the three full nextest runs (the first full run had only the unrelated redraw failure; the next two were entirely green). The final database unit regression took 0.037 s, and the two real startup failure paths together took 1.565 s. Hosted three-consecutive-run verification remains pending PR CI.

## Hosted CI evidence

After local validation, Jason explicitly changed the sequencing: commit, push, and open a regular PR so hosted CI can run alongside the review. Review remains required before this is ready to merge. Baseline totals come from job logs, excluding the nested Unix subprocess summary.

| Platform and baseline run | Test step | Job | Passed / failed / ignored |
| --- | --- | --- | --- |
| macOS 37769556810 | 8m32s | 10m58s | 2,048 / 0 / 6 |
| Windows 37765871789 | 13m13s | 16m39s | 1,981 / 2 / 4 |

The Windows issue baseline is an earlier #844 revision with 1,106 daemon unit tests; the main run Jason linked has 1,108. Thus Windows should gain 95 tests relative to the issue's baseline (93 from this change and two from the newer base), with formerly failing tests also becoming passes. On the current base this predicts 2,078 Windows passes and four ignored. macOS should gain exactly 93, yielding 2,141 passes and six ignored. Hosted totals and consecutive real-daemon results will be checked against those expectations.
