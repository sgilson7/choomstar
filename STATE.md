# State

The project's long-term memory, at most two pages. Rewrite it at each milestone (a stage boundary, a finished feature, a deploy) and before a handoff. After a compaction or in a new session, read this file and ask `tools/ask.py` for cards; do not re-read files it already summarises. `scripts/context_check.py` checks its size, its commit and the cards it cites.

Commit: 39bfd3b

## Where the build is

Stage 0 done (journal published: https://claude.ai/artifact/Wu3LrWtBfTBwbEi92RSZB6). The person accepted every CM default on 2026-11-12 ("im good with defaults, now build") and said never to stop a turn again (CLAUDE.md, Rules agreed).

M0 in progress, 2026-11-12:
- Copied from Chumontillado c48d8dd: crates/, web/, data/ (missions.json kept), assets/, tools/mesh, tools/art, scripts/check_clips.py, join_skins.py, break_tests.py, tests/clip_defect.py, tests/browser/check.py + interactions.py, tests/breaks.json, GLOSSARY.md (+ Choomstar terms at its foot; Plaza's Not line lost "arena"), LICENSES.md, Cargo.lock.
- Online removed from the page: web/online.js, web/net/, online_fence.rs gone; CSP connect-src 'self'; app.js uses web/save/store.js. Core online.rs/rollback.rs/versus.rs stay (no I/O; unreachable from the menu; kept for fold-back).
- Saves kit copied (save.rs still the kit's example Save; playtest.rs; old_saves.rs, storage_fence.rs; no_storage.rs deleted). Needs: Choomstar's Save {version, prefs, missions}, tests/saves/v1.json, act "save.load" (page sends it; core has "prefs.load").
- world.rs hooks (physics untouched): Player.team/skill/run_pct, Dummy.tall/kind, World.rules {hunt, stay_down}, World.felled. nav.rs: Nav::reachable(map, from).
- data/ltc.json: LTC's 10 foes, 30 patterns, colours, balance.

## What is next

1. ltc.rs (orbs: ring/aimed/sphere; form and wave approximated, R2.4) and foes as clusters of hit dummies (kind>0, health reset each tick, damage summed).
2. mission.rs: specs from data/missions.json (include_str), unlock, medals, Run (targets, gates, horde zombies = melee bots with run_pct, combat/ladder, team, arena waves, bosses incl. Black Knight as a big bot, salvage relics + lift, gauntlet stages + repair cells); World::strike for orb hits.
3. Game: Screen::Missions (chart + list + briefing), Results; menu; HUD lines; R restarts; Save.
4. Page: chart SVG from ui().chart; scrollable list; styles chrome.
5. check.sh (add clips + rotted steps), PLAN.md, SECOND-ORDER rows R2.x, journal entries, publish (gh repo create choomstar --public; ./scripts/publish.sh).

## Cards the next session needs

card:docs.context.how-to-read-builder, card:entries.kinds.p2, card:docs.procedure.p2, card:docs.journal.the-opening-entry
