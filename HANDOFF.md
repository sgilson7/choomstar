# Handoff

Stage 9, written by the agent. `C5 Consolidate in your own words`. A fresh session starts from this file, not from memory.

Commit: dc3b8c2 (live)

## What this is

Choomstar: a single-player mission run in the browser on Chumontillado's core (c48d8dd): fifty missions of eleven kinds on a star chart of eight tiers, with Long Time Comin's foes and orb patterns. Live at https://sgilson7.github.io/choomstar/ (repo github.com/sgilson7/choomstar). The journal is at https://claude.ai/artifact/Wu3LrWtBfTBwbEi92RSZB6.

## Rules that hold it up

- Movement and weapons are Chumontillado's: its 337 world/weapon tests pass unchanged (CM-1); missions change the world only through `World::rules`, `Player.team/skill/run_pct`, `Dummy.kind/tall`, `World::strike/hit` (break rows "CM sides", "CM targets stay down", "CM a zombie's pace").
- Missions are data (`data/missions.json`, `data/ltc.json`); `mission::open` is Vagrancy's rule (break row CM-3).
- Every mission clears for an oracle player (`every_mission_can_be_cleared_inside_its_time`, CM-5).
- Storage only in `web/save/store.js` (`storage_fence.rs`); every deployed save loads (`old_saves.rs`, `tests/saves/v1.json`).
- Binary files live in `assets/` (the page scan reads `web/` as text).

## Shape of the code

| Where | What |
| --- | --- |
| `crates/core/src/mission.rs` | the pack, unlocks, medals, a mission's run (Parts) |
| `crates/core/src/ltc.rs` | Long Time Comin's orbs and foes |
| `crates/core/src/chart.rs` | the chart, list, briefing, results and HUD as the page draws them |
| `crates/core/src/game.rs` | screens Missions and Results, the mission acts, the save |
| `web/app.js`, `web/style.css` | the chart SVG, the list, the chrome look |

## Commands

`./scripts/check.sh` (all checks, three browsers: prints BUILDER CHECK PASSED); `./scripts/publish.sh` (check, then gh-pages); `BUILDER_ORIGIN=https://sgilson7.github.io/choomstar .venv/bin/python tests/browser/check.py` (live check).

## What will bite within the hour

Two journal live entries on the same date: the newest must come first on its page, or journal_check reads the older as newest. A JPEG under `web/` fails the boundary test. Strings are checked against every glossary Not line, Chumontillado's included.

## The journal

`../choomstar-journal/index.html`, published at https://claude.ai/artifact/Wu3LrWtBfTBwbEi92RSZB6; newest entry 2026-11-12-live-3 (dc3b8c2).

## Open for the person

Every `SECOND-ORDER.md` row with status *person*, by number, with what the person has to decide or supply. `scripts/notebook_check.py` fails while one is missing.

- R1.8: read the fifty mission names, their briefs and the tier names (data/missions.json, data/strings.json `tier_*`); they are the agent's text.

## Mistakes so far

By system, from `SECOND-ORDER.md`, with the rule each one produced.

## What passed

`./scripts/check.sh` passed at dc3b8c2 (format, clippy, core and boundary tests, break rows, wasm build and audit, art, lifelike clips, notebook, journal, browser check in Chromium, Firefox and WebKit). The live browser check passed in all three.

Tokens (`tools/tokens.py`): | 2026-11-12 08:49 | be6be7a2 | 269 calls | context at first plan 198k | peak 414k | mean 222k | 1 compaction | re-read 59.1M | 26% above 200k |

## The next action

One action.
