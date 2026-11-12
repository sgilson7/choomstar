# Second-order notebook

Stage 6, kept by the agent as the work happens, not afterwards. `B3 Build in self-questioning`.

The code and tests are the work; this file is the record about the work. It is where a decision the agent made alone, an assumption that turned out wrong, or something only the person can do becomes visible to the person. With the plan approved in advance, it is the main place those decisions surface before the person uses the tool. [`docs/procedure.md`](https://github.com/sgilson7/builder-setting/blob/main/docs/procedure.md#the-second-order-notebook) says why.

One row per assumption, surprise or open item. **Kind** is *divergence* (what was assumed, what turned out to be true, what changed), *finding*, or *worklist* (the person's to do). Number rows by round (R1.1, R1.2, then R2.1 after the first triage) so a later round does not renumber an earlier one. **Rule** is the rule the row produced for later sessions, and where it now lives (`CLAUDE.md`, a test, `tests/breaks.json`), or blank. **Status** is *done*, *open*, or *person*; every *person* row is named in `HANDOFF.md`. `scripts/notebook_check.py` checks the form of each row and that the notebook is not empty once `PLAN.md` holds a decision, not what the rows say.

| # | Kind | Assumed or found | What is true | What changed | Rule | Status |
| --- | --- | --- | --- | --- | --- | --- |
| R1.1 | finding | References would be read through digests in refs/ | Chumontillado and Chumworld Arena had digests (~/Documents/Builder/refs); Long Time Comin and Vagrancy had none | Copied the two into ../refs/ and wrote digests for Long Time Comin and Vagrancy with tools/digest.py, filling What to copy (rule 20) | | done |
| R1.2 | divergence | The project starts from the blank template | The person asked for Chumontillado's physics, maps and skins directly, and identical play at the start | Started from the current template (newer tools: ask.py, STATE.md, decisions.py) and will bring in Chumontillado's crates, data, web and assets at c48d8dd, as Chumworld Arena copied CHUMWORLD at a named commit | CM-1 in the journal; parity and movement tests | open |
| R1.3 | finding | The repository name "Choomstar" | GitHub Pages serves under the lowercased name | Repository `choomstar`; BRIEF.md Autonomy says so | | done |
| R1.4 | divergence | The person named nine mission kinds and asked for two more | Melee-only and fixed-weapon could be rules rather than kinds | Kept as kinds of their own (eleven), and any mission can also fix a weapon; CM-Q5 asks | | open |
| R1.5 | finding | The two new kinds come from research on Pikmin 2 and Melee | Salvage (Pikmin 2 Challenge Mode) and Gauntlet (Melee All-Star) | Journal Research page; CM-Q4 asks | | done |
| R1.6 | divergence | Medal thresholds and limits for 50 missions | No mission has been played | Every limit and medal mark in data/missions.json is a first guess, to be set by scripted runs and play | CM-5 | open |
| R1.7 | finding | Chumontillado's own HUD was in the screenshots | Its page CSP blocks injected styles | tools/shots.py hides the HUD by setting element styles from a script | | done |
| R1.8 | worklist | The mission list, mission names, tier names and briefs are generated text a reader will see | They are the person's to read (CLAUDE.md rule 12) | Listed on the journal's Missions page and in data/missions.json | | person |

## Strings written by the agent

Each reader-visible string the agent wrote, with its key, for the person to read.

| Key | String |
| --- | --- |
