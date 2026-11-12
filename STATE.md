# State

The project's long-term memory, at most two pages. Rewrite it at each milestone (a stage boundary, a finished feature, a deploy) and before a handoff. After a compaction or in a new session, read this file and ask `tools/ask.py` for cards; do not re-read files it already summarises. `scripts/context_check.py` checks its size, its commit and the cards it cites.

Commit: d118ed0

## Where the build is

MVP live, 2026-11-12: https://sgilson7.github.io/choomstar/ from 4517593 (repo github.com/sgilson7/choomstar, public; gh-pages via ./scripts/publish.sh; repo-local git identity sgilson7 noreply). check.sh passes in 3 engines; live browser check passes. Journal round 1 published (Build, round 1 page; Live now at 4517593): https://claude.ai/artifact/Wu3LrWtBfTBwbEi92RSZB6. The person accepted every CM default; CLAUDE.md "Rules agreed": never stop a turn while the brief says keep going.

Code (card:mine for details):
- Chumontillado c48d8dd ported whole; page online removed (R2.2); saves kit (save.rs Save {version 1, prefs, missions}; tests/saves/v1.json).
- world.rs hooks: Player.team/skill/run_pct, Dummy.tall/kind, Rules {hunt, stay_down, no_reset}, felled, strike(), respawn pub. nav.rs reachable().
- ltc.rs: LTC orbs (ring/aimed/sphere; form+wave simplified R2.4), Foe with gaits, phases, summons, moth recoil. data/ltc.json.
- mission.rs: pack (include_str data/missions.json), open/medal_for/record/route_from/chart_at, Run with Parts (Targets, Course, Horde, Duel, Team, Arena, Boss, Salvage; gauntlet = list of parts), foes as dummy clusters (R2.6), zombies = melee bots (R2.7).
- chart.rs: view model (Chart, Entry, MedalMark), mission/results panels, mission HUD lines. game.rs: Screen::Missions/Results, acts menu.missions, mission.pick/start/retry/next, results.chart, save.load; Backspace restarts (R2.9).
- web/app.js draws the chart SVG and the list; style.css chrome theme.

## What is next

1. CM-5: a core test that clears every mission by an oracle (teleport to gates/relics, fell foes) so each goal is shown reachable.
2. Zombie figure of its own (R2.7); the diver on title/briefing/results (CM-Q6, Style Lab stills); chrome sky (CM-Q11).
3. Chart polish: spoke labels overlap outer nodes.
4. PLAN.md rows; HANDOFF.md full; tokens row.
After each change: check.sh, commit, publish.sh, live check, journal entry + republish.

## Cards the next session needs

card:docs.context.how-to-read-builder, card:entries.kinds.p2, card:docs.procedure.p2, card:docs.journal.the-opening-entry
