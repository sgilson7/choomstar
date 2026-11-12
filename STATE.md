# State

The project's long-term memory, at most two pages. Rewrite it at each milestone (a stage boundary, a finished feature, a deploy) and before a handoff. After a compaction or in a new session, read this file and ask `tools/ask.py` for cards; do not re-read files it already summarises. `scripts/context_check.py` checks its size, its commit and the cards it cites.

Commit: 39bfd3b

## Where the build is

Stage 0 done, 2026-11-11: the opening entries are written and published.

- Folder layout: `~/Documents/Choomstar/` holds `builder/` (reference, read through cards), `choomstar/` (this project, from `builder/template`), `choomstar-journal/` (own git repo, never pushed), `refs/` (digests: chumontillado, chumworld-arena, long-time-comin, vagrancy).
- Journal published (private, db for the chips): https://claude.ai/artifact/Wu3LrWtBfTBwbEi92RSZB6 — republish with the Artifact tool from `choomstar-journal/index.html` after each entry. Pages: reading (opening, 12 rules CM-1..CM-12, chips CM-Q1..Q18), brief, research, missions (3 animations), screens (mockups), plan (M0-M7), live now. `scripts/journal_check.py` passes.
- Journal tools: `tools/entries.py` (one-off, already run; do not rerun), `tools/build.py` (redraws GEN markers from data/missions.json; rerun after the data changes), `tools/shots.py`, `data/ltc.json`.
- `data/missions.json`: 50 missions from `tools/draft/missions_draft.py` (tier = requirement count; requirement {mission, medal bronze|silver|gold}; 11 kinds, one per tier max; mission 50 secret, row 7).
- `SECOND-ORDER.md` rows R1.1 to R1.8 (R1.8 person: read the generated mission names).
- Measured: template check passes in 3 engines; Chumontillado c48d8dd 366 tests pass, 2 ignored.

## Decisions in force

- Base: copy Chumontillado (c48d8dd) crates/data/web/assets into this template; remove online code (default of a CM question). Movement and weapons unchanged.
- Two new mission kinds (research: Pikmin 2 Challenge Mode, Melee All-Star): Salvage and Gauntlet.
- Chart default: star chart (finale at centre, tier rings, one spoke per kind, one route per mission from a same-kind requirement first). Alternatives mocked: lanes, Vagrancy bands. A scrolling mission list with briefing beside it.
- Medals: bronze (any clear), silver, gold, shown by letter and ring count.

## What is next

All CM questions are built on their defaults until the person answers (BRIEF Autonomy). Next: PLAN.md (M0 to M7 as in the journal's Plan page), then M0: bring Chumontillado's crates/, data/, web/, assets/, tools/mesh, scripts/check_clips.py, join_skins.py, tests/clip_defect.py and its check.sh/build.sh differences into this template; remove online code (online.rs, rollback, versus, web/net, online.js, tests/browser/online.py); get `./scripts/check.sh` passing; commit. Then M1 missions core, M2 chart/list UI, M3-M6 content, M7 publish (`gh repo create choomstar --public`, `./scripts/publish.sh`).

## Cards the next session needs

card:docs.context.how-to-read-builder, card:entries.kinds.p2, card:docs.procedure.p2, card:docs.journal.the-opening-entry
