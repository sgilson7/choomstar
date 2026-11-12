# Second-order notebook

Stage 6, kept by the agent as the work happens, not afterwards. `B3 Build in self-questioning`.

The code and tests are the work; this file is the record about the work. It is where a decision the agent made alone, an assumption that turned out wrong, or something only the person can do becomes visible to the person. With the plan approved in advance, it is the main place those decisions surface before the person uses the tool. [`docs/procedure.md`](https://github.com/sgilson7/builder-setting/blob/main/docs/procedure.md#the-second-order-notebook) says why.

One row per assumption, surprise or open item. **Kind** is *divergence* (what was assumed, what turned out to be true, what changed), *finding*, or *worklist* (the person's to do). Number rows by round (R1.1, R1.2, then R2.1 after the first triage) so a later round does not renumber an earlier one. **Rule** is the rule the row produced for later sessions, and where it now lives (`CLAUDE.md`, a test, `tests/breaks.json`), or blank. **Status** is *done*, *open*, or *person*; every *person* row is named in `HANDOFF.md`. `scripts/notebook_check.py` checks the form of each row and that the notebook is not empty once `PLAN.md` holds a decision, not what the rows say.

| # | Kind | Assumed or found | What is true | What changed | Rule | Status |
| --- | --- | --- | --- | --- | --- | --- |
| R1.1 | finding | References would be read through digests in refs/ | Chumontillado and Chumworld Arena had digests (~/Documents/Builder/refs); Long Time Comin and Vagrancy had none | Copied the two into ../refs/ and wrote digests for Long Time Comin and Vagrancy with tools/digest.py, filling What to copy (rule 20) | | done |
| R1.2 | divergence | The project starts from the blank template | The person asked for Chumontillado's physics, maps and skins directly, and identical play at the start | Started from the current template (newer tools: ask.py, STATE.md, decisions.py) and will bring in Chumontillado's crates, data, web and assets at c48d8dd, as Chumworld Arena copied CHUMWORLD at a named commit | CM-1 in the journal; parity and movement tests | done |
| R1.3 | finding | The repository name "Choomstar" | GitHub Pages serves under the lowercased name | Repository `choomstar`; BRIEF.md Autonomy says so | | done |
| R1.4 | divergence | The person named nine mission kinds and asked for two more | Melee-only and fixed-weapon could be rules rather than kinds | Kept as kinds of their own (eleven), and any mission can also fix a weapon; the person accepted every default on 2026-11-12 | | done |
| R1.5 | finding | The two new kinds come from research on Pikmin 2 and Melee | Salvage (Pikmin 2 Challenge Mode) and Gauntlet (Melee All-Star) | Journal Research page; CM-Q4 asks | | done |
| R1.6 | divergence | Medal thresholds and limits for 50 missions | No mission has been played | Every limit and medal mark in data/missions.json is a first guess, to be set by scripted runs and play | CM-5 | open |
| R1.7 | finding | Chumontillado's own HUD was in the screenshots | Its page CSP blocks injected styles | tools/shots.py hides the HUD by setting element styles from a script | | done |
| R1.8 | worklist | The mission list, mission names, tier names and briefs are generated text a reader will see | They are the person's to read (CLAUDE.md rule 12) | Listed on the journal's Missions page and in data/missions.json | | person |
| R2.1 | finding | Every CM question would wait for an answer | The person accepted every default on 2026-11-12 ("im good with defaults, now build") and asked that no turn stop again while the brief says keep going | Built on every default; the rule is in CLAUDE.md, Rules agreed for this project | CLAUDE.md, Rules agreed | done |
| R2.2 | divergence | Online code would be removed from the core too (CM-Q12 default) | online.rs, rollback.rs and versus.rs are woven through game.rs (168 references) and do no I/O | Removed from the page (web/online.js, web/net/, the relays in the CSP, online_fence.rs); the core modules stay, unreachable from the menu, which also keeps fold-back to Chumontillado a smaller diff | connect-src 'self' in web/index.html; boundary.rs | done |
| R2.3 | divergence | The glossary would be Choomstar's own | Chumontillado's glossary rules out "arena" for the Plaza, "walker" for nothing Choomstar needs, and "room", "menu", "level", "stage" in its strings | Chumontillado's glossary kept whole with Choomstar's terms at its foot; the Plaza's Not line lost "arena" (Arena is a mission kind here); Zombie's lost "walker" (a Long Time Comin foe kind) | GLOSSARY.md; copy.rs | done |
| R2.4 | divergence | Long Time Comin's patterns would be ported whole (CM-8) | Its held formations (form) and ground waves need its level geometry and a ring renderer | ring, aimed and sphere ported with their constants; a form hangs as a ring round its centre and sets off inward; a wave is a flat ring of 32 orbs along the floor | data/ltc.json _about; ltc.rs | open |
| R2.5 | divergence | Parkour missions carry no weapon ("none") | Chumontillado's movement changes with the melee weapon in hand (the navaja runs faster) | A parkour course carries the pistol only, the weapon that changes no movement; nothing lies on the map | mission.rs Run::start | done |
| R2.6 | divergence | Long Time Comin's foes would be seats or a new body type | Chumontillado's weapons hit seats and straw targets only | A foe stands as one to five straw-target hit bodies kept full; what they lose each tick is the foe's damage, so every weapon hits a foe as it hits a target (CM-1) | mission.rs sync_parts; a_shot_foe_loses_what_its_hit_body_took | done |
| R2.7 | divergence | Zombies would be new primitive figures (CM-Q8 default) | No time yet for a new skin; Chumontillado's bots already swing melee weapons | A zombie is a bot on side 2 with one melee weapon and a slower pace (shambler 45 %, runner 85 %, brute 60 % and 220 health), worn as the Friar, the Harlequin or the Black Knight; a figure of its own is still to come | mission.rs put_zombie | open |
| R2.8 | divergence | Combat "win 3 rounds" and the ladders | The draft said rounds and ladder without saying how | A round is won by felling every bot of it and lost by being felled; three lost ends it. A ladder is its bots one at a time, each a round | chart.rs goal_win_rounds, goal_ladder strings | done |
| R2.9 | divergence | Medals and restarts on R (CM-Q18) | Chumontillado's R reloads; Backspace stands you back at your spawn | In a mission Backspace starts it again at once and R still reloads | mission.rs, World::rules.no_reset | done |
| R2.10 | finding | The browser check would find no new failure | Its title test reads the rendered text, so CSS uppercase made "Choomstar" read "CHOOMSTAR" | The title is drawn in small capitals, which keeps its text | tests/browser/interactions.py | done |
| R2.11 | divergence | A course's gates could be spread over the floor a walk from the spawn reaches | On the Tobogán that floor is only the platform: the slide is surf ramps, so all eight gates stood at the top (the person: "tobogan level is messed up, the gates are all at the top") | Gates follow the map's own route, the weapon spots Chumontillado places along it, in the order a run meets them, evenly to the count asked; a gate counts within reach of its middle, whichever way is up | the_toboggan_course_runs_from_the_platform_down_the_slide_to_the_arena; breaks.json | done |
| R2.12 | divergence | A mission would start at once, and a won round go straight to the next | The person: the time between is too short; a count before a mission, and between rounds a pause that says who won ("really jarring when you win a round to instantly begin a new one") | Chumontillado's own round timing: a 3 s count before every mission, round and gauntlet stage (nobody moves, the clock waits, the count and go are heard), and a round's or stage's end said for 4 s ("You take the round", "The Hidalgo takes the round", with the rounds so far) before the next count | a_mission_opens_on_a_three_second_count…, a_round_won_is_said…, a_round_lost_names_the_bot…; breaks.json | done |
| R2.13 | finding | The results screen's first button (the one focused) would always be Try again | The person: "the default should not be retry mission if you get a gold, it should be back to the chart" | After a gold, Back to the chart comes first and is focused; after any other end, Try again | after_a_gold_the_first_button_is_the_chart_and_otherwise_try_again; breaks.json | done |
| R2.14 | divergence | Eleven mission kinds on Chumontillado's maps | The person asked for a twelfth, long jumps ("scored for how long you can go in the course, and how long of a jump you can hit at the end ... gold is only obtainable with a good jump off the end"), on new maps after Ape Race's Sector Ape | Kind longjump, four courses of their own (courses.rs: Launch Rail, Debris Field, Wreck Vines, Sector Ape) built from Chumontillado's boxes, surf troughs and vines in a space look (look.json `space`); score = metres along the course to its lip + metres past the lip where you land on the field; silver is the lip's distance, gold is past it. Vines hang straight down (Chumontillado's vines; Sector Ape's sideways vines would change the physics, CM-1). Lengths, limits and marks are first guesses, unplayed | courses.rs tests; a_long_jump_scores…, …earns_silver_at_most, falling_off…; breaks.json | open |

## Strings written by the agent

Each reader-visible string the agent wrote, with its key, for the person to read.

| Key | String |
| --- | --- |
| title | Choomstar |
| loading | Warming the reactor |
| tagline | A diver's mission run on Chumontillado's maps: fifty missions on a chart of eight tiers. Earn medals to open the next. |
| menu_text | Missions are the game: break targets, run courses, last out hordes, fight bots and Long Time Comin's foes. The practice room and a match against bots are here too. |
| missions | Missions |
| chart | Chart |
| mission_list | Mission list |
| briefing | Briefing |
| start_mission | Start |
| retry | Try again |
| next_mission | Next mission |
| to_chart | Back to the chart |
| mission_locked | This mission is locked. |
| locked_by | Opens with {list}. |
| goal_salvage | Bring {need} of {n} relics to the lift; stand on the open lift to leave. |
| goal_gauntlet | {n} fights on one health bar, {repairs} repair cells. |
| goal_win_rounds | Win {n} rounds; lose {n} and it is over. |
| goal_ladder | Beat {n} bots, one at a time; {lives} lives. |
| hud_restart | Backspace starts it again |
| mission.cleared | Every goal met. |
| mission.felled | You were felled. |
| secret_brief | Earn the medals it asks for to see it. |
| save.unreadable | The saved progress could not be read; a new run has begun and the earlier one is kept. |
| kind_*, tier_*, row_*, hud_*, goal_*, mark_*, medal_* | the rest of the Choomstar keys in data/strings.json after "save.no_step" |
