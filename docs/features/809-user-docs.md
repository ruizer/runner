# 809 — User documentation

> Tracking issue: [#809](https://github.com/yicheng47/runner/issues/809)
> Priority: P2, no milestone yet. Platforms: macOS and Windows.
> Status: draft structure, waiting for Jason's comments.
> Related: runnersh.dev renders these pages at `/docs/`; that work lives in the website repository. [#565](https://github.com/yicheng47/runner/issues/565) (i18n) decides when a Chinese version follows.

## Motivation

Runner has no documentation for people who use it. `docs/` is written for people building Runner, and the README carries the whole user-facing story: a feature tour, the `runner` CLI summary, the supported-agents table and the example crews. A README can introduce Runner but cannot teach it: there is no path from a fresh install to a first mission, no explanation of how roles, crews, missions and sessions relate, and no reference beyond what fits on one page.

## Where the docs live

The user guide lives in this repository, in `docs/guide/`, next to the code it describes. A pull request that changes user-visible behavior updates the guide in the same diff, and anyone can correct a page because the repository is public.

runnersh.dev reads `docs/guide/` from the latest published `v*` release tag when it builds, so the website describes the version people download. Its existing rebuild on each published release picks up new docs automatically. A correction made on `main` reaches the website with the next release; it is visible on GitHub at once.

## Structure

```
docs/guide/
  README.md                     table of contents; the website's navigation
  getting-started/
    install.md
    first-chat.md
    first-mission.md
  concepts/
    roles.md
    crews.md
    missions.md
    sessions.md
    projects.md
  guides/
    build-a-crew.md
    drive-runner-from-agents.md
    chats-and-windows.md
    mcp-and-skills.md
    appearance.md
    troubleshooting.md
  reference/
    cli.md                      generated
    supported-agents.md
    keyboard-shortcuts.md
```

### Getting started

| Page | Covers | Sources |
| --- | --- | --- |
| `install.md` | Downloading Runner for macOS or Windows, installing the agent CLIs it drives, and the `runner` command on `PATH`. | README Download, Supported agents and Drive Runner from your agents; **Settings → Agents** and **Settings → General → Command line** |
| `first-chat.md` | Starting a chat with a role, splitting panes, the terminal drawer, and what happens on quit and relaunch. | README Chats and Also in the box; new writing |
| `first-mission.md` | Starting the seeded pair-coding crew on a goal, reading the feed, answering a question, and stopping and archiving the mission. | README Missions and Example crew; `examples/pair-coding/`; new writing |

### Concepts

| Page | Covers | Sources |
| --- | --- | --- |
| `roles.md` | A role as a reusable agent configuration: runtime, command, model, effort, system prompt, working directory. | README Crews; `crates/runner-core/src/protocol/model.rs` (`Role`); the role page |
| `crews.md` | Slots, handles, the one lead, per-slot overrides, team conventions. | README Crews; `model.rs` (`Crew`, `Slot`); the crew page |
| `missions.md` | The goal, the feed and event log, signals, questions for the person, mission permissions (Bypass), and how a mission survives a quit. | README Missions and Mission controls; `docs/arch/arch.md` |
| `sessions.md` | A session as one agent process in a real terminal; stop, resume and restart; resume after relaunch. | README Also in the box; `docs/arch/arch.md` |
| `projects.md` | Binding a working directory once, and how chats and missions inherit it. | README Projects |

### Guides

| Page | Covers | Sources |
| --- | --- | --- |
| `build-a-crew.md` | Choosing roles and slots, writing system prompts and team conventions, mixing agents, and the crews in `examples/`. | README Example crew and More crews; `examples/` |
| `drive-runner-from-agents.md` | The `runner` skill each agent gets, starting and following a mission from a shell or another agent, messaging the lead, and exit codes. | README Drive Runner from your agents; `runner help agents` |
| `chats-and-windows.md` | Tabs, split panes, dragging panes, folders, status markers, several windows and the hand-off overlay. | README Chats and Multi-window |
| `mcp-and-skills.md` | Viewing and switching MCP servers and skills per agent, and what Runner changes in each agent's config files. | README MCP servers and skills |
| `appearance.md` | Themes, the app and terminal palettes per mode, and zoom. | README Light and dark, and A palette per mode |
| `troubleshooting.md` | An agent CLI not detected, the `runner` command missing, a sandbox blocking the CLI (exit 5), a session that did not resume. | README Supported agents; `runner help agents`; open issues |

### Reference

| Page | Covers | Sources |
| --- | --- | --- |
| `cli.md` | Every `runner` command and option. | Generated from `runner help` and each command's `--help` |
| `supported-agents.md` | The capability table, per-agent setup notes, and Windows requirements. | README Supported agents |
| `keyboard-shortcuts.md` | Every default shortcut, with the macOS and Windows keys side by side. | `crates/runner-app/src/keymap.rs` |

## Conventions

- **Plain Markdown.** GitHub-flavored Markdown with no MDX and no raw HTML, so a page reads the same on GitHub and on the website.
- **Title and summary.** The first `#` heading is the page title and the first paragraph is its summary, used for the page description and link previews. No front matter.
- **Navigation.** `docs/guide/README.md` is a nested list of links in reading order; the website builds its sidebar from that list. A page missing from it is not published.
- **Links.** Pages link to each other with relative `.md` paths, which work on GitHub; the website rewrites them to its routes.
- **Callouts.** GitHub alerts (`> [!NOTE]`, `> [!TIP]`, `> [!WARNING]`) render on GitHub and map to the website's callouts.
- **Platforms.** Shortcuts give the macOS keys first, then Windows (`⌘D` / `Ctrl+D`). A behavior that differs on Windows says so where it applies.
- **Vocabulary.** Role, crew, slot, lead, mission, session, chat and project are used exactly as the app uses them; `concepts/` defines them once.
- **Images.** None in the first version; see Open questions.
- **English only** until #565 settles the Chinese version.

## Generated CLI reference

`cli.md` is written by a script that runs the built `runner` binary's `help` and every subcommand's `--help` and assembles the output into one page. A `make` target regenerates it, and CI fails when the committed page differs from the binary's output, so the reference cannot drift.

## Keeping the guide current

`AGENTS.md` gains a rule: a pull request that changes user-visible behavior, such as a command, a setting, a shortcut or what a screen does, updates `docs/guide/` in the same pull request. A feature's spec names the pages it touches.

## Out of scope

- The website's docs pages: layout, search and rendering live in the website repository.
- A Chinese translation; it follows #565.
- Trimming the README into a shorter introduction that links to the guide; that comes after the guide ships.
- Contributor documentation; `docs/arch/`, `docs/features/` and the rest stay as they are.

## Open questions

1. **Images.** The website draws every view of Runner as an HTML replica, never a screenshot. Screenshots in the guide go stale with each UI change. Recommendation: text-only for the first version; revisit once the guide exists.
2. **Folder name.** `docs/guide/` keeps the guide under `docs/` beside the contributor material, with its own README as the entry point. A top-level `guide/` would separate the two more clearly.
3. **Corrections between releases.** A fix on `main` waits for the next release to reach the website. Recommendation: accept it; releases are frequent.

## Implementation phases

1. The structure: `docs/guide/README.md`, one stub per page with its title and summary, the conventions, and the `AGENTS.md` rule.
2. Getting started and Concepts, drawn mostly from the README.
3. Guides.
4. Reference, with the generated `cli.md`, its `make` target and the CI check.
5. The README links to runnersh.dev/docs once the website renders the guide.

## Verification

- Every page renders on GitHub and every relative link resolves.
- Every command, setting path and shortcut is checked against the current release on macOS and Windows.
- `cli.md` matches the binary's help output, and the CI check fails on a deliberately stale copy.
- Someone following only the getting-started pages goes from a fresh install to a finished first chat and a first mission.
