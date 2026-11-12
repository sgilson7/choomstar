# Brief

Stage 1. The person's own words come first; the rest is written by the agent from them and read by the person. `C1 Frame the problem and set the rules`, `B1 Establish context`.

## The description, word for word

The opening (role, references, permissions), then the description, as given on 2026-11-11:

> Builder is already in this folder as builder/ (do not clone it). Use Builder to constrain and direct your work as [ROLE: the game's co-designer and developer | the tool's developer]. The goal is a page on GitHub Pages that anyone can open in a browser, so put running well in the browser first.
>
> Earlier projects built with Builder, to read for what works: [Chumontillado, Vagrancy, long time coming, chumworld arena]. List them under References in BRIEF.md. When one has a digest in refs/, name the digest.
>
> First, read Builder through its cards (python3 <project>/tools/ask.py --start, then ask.py "question"; open a Builder file only where a card points) and write the design journal's opening entry: how you will use Builder for this game, the rules you will hold to and what holds each, and your questions as answer chips.
>
> PERMISSIONS: You may create a public repository named Choomstar and deploy it to GitHub Pages when the checks pass." "Do not use subagents for now."  DESCRIPTION OF GAME: I'd like to make a game that is a single player version of chumontillado, that is mainly an arcade mode with a tiered progression system like vagrancy, or like super smash bros melee one mode with the 50 different missions, or pikmin 2's mission mode. use chumontillado's physics, existing maps, and skins directly. if you need to create new assets like maps character models guns weapons etc, use the method described within chumontillado.  The different mission types I am thinking of are the following: horde mode, where an increasing number of zombies that hurt you in melee range begin to spawn and you either have to survive or defeat a certain number of them. another is parkour, reach the end in a certain amount of time and get a medal for different times. combat, where you fight an increasing and ever more deadly set of other enemy cpu players, a team deathmatch set of missions, melee only missions, fixed weapon missions,  hit the target mode, where you have targets on a map and a fixed weapon. and ultrakill levels, where you are in small combat arenas against new foes like enemies from long time coming, you can use some of those enemies outright to start, even with their ball firing patterns and everything. also there should be boss missions, where you fight against a boss type enemy, look at long time coming and chumworld arena as examples. think of two additional mission types yourself from a literature review over pikming 2 and super smash bros melees arcade modes. the mvp should have 50 different missions, and focus on making the map screen more legibile but highly stylized, like vagrancy but cleaner, use the game phantom blade zero as a good example. you should also have a way to scroll through the missions in a menu as well. start by building out the design log, mocking up everything i've asked for here, and making animations to describe the flow of the missions and how they logically progress from one to the other, and any relevant design decisions that need to be surfaced to me. look at the snapclass journal for good animations embedded, and the chumontillado journal as another good example of mockups, and the builderrefinedartstyle repo for anything you need regarding generating new lifelike blender characters and for embedding videos in journals. now please begin, once the design journal is made, surface it to me, then continue working, dont stop until the mvp is deployed to a github and a public github pages, you have my permission to make deploy. i want this to be made in an open ended way and to be extensible, add new mission packs, characters, guns, etc and then slowly fold new things from this game into the original chumontillado that supports online play, as choomstar is strictly single player. feel free to use the diver from long time coming as a representation / model for the main character, and the game should have an overall space age / chrome theme to it, since its named choomstar, but you can re-use any and all assets from chumontillado while making this to speed up, and the physics and gameplay should be identical to chumontillado at the start before content is added, ok now begin good luck

## The tool

Choomstar: a single-player first-person movement shooter for a desktop browser (keyboard and mouse), built on Chumontillado's core at commit c48d8dd with its movement, weapons, maps and skins unchanged. Play is a mission mode: 50 missions of nine kinds laid out as a tiered chart (rows opened by medals on earlier missions), each mission a short challenge with bronze, silver and gold. The look is chrome and space age; the player is Long Time Comin's diver.

## Done means

For the MVP:

- Movement, guns and melee feel the same as Chumontillado at c48d8dd: its movement and weapon tests pass unchanged in this repository.
- 50 missions, listed in `data/missions.json`, across the nine kinds the journal names (horde, parkour, combat, team deathmatch, melee only, fixed weapon, targets, arena, boss, plus the two the research entry proposes), each finishable with at least a bronze medal by the agent's playtest harness or a scripted run.
- A mission chart (tiers as bands, missions as nodes, routes between them) and a scrolling mission list, both drawn from the core's `ui()`; a locked mission says what opens it.
- Medals and best times survive a reload (the saves kit).
- Long Time Comin's enemies (at least five kinds, with their orb patterns) and one boss with phases.
- Published on GitHub Pages at `sgilson7.github.io/choomstar/`, with the browser check passing against the live address.

## Decisions that stay with the person

- How the movement and every weapon feel (unchanged from Chumontillado until a T row changes them).
- The mission list, its order, medal times and difficulty: the agent's numbers are starting points, set by playtest.
- The two proposed mission kinds (journal CS questions).
- What folds back into Chumontillado, and when.
- The diver's likeness and any new character, gun or map look.

## Accessibility

Medals are told apart by letter and shape (B, S, G and a ring count) as well as colour; a locked node carries a lock glyph and its requirement as text. The mission list can be read and chosen by keyboard alone. HUD timers and counts are text. Settings keep Chumontillado's field of view and look speed.

## Style

Chrome and space age: dark field, brushed-metal panels, one hot accent per tier, thin rules and condensed type, after the menus of Phantom Blade Zero (ink-black, red-gold accents, calligraphic restraint) recoloured toward chrome. In the 3D scenes, Chumontillado's figures and maps as they are, with chrome-tinted sky and HUD.

## References

- **Chumontillado** (../refs/chumontillado.md): the whole game core — movement, weapons, maps, skins (painted and lifelike), bots, HUD; the asset method (primitive figures in `skins.rs`, lifelike figures baked by Style Lab).
- **Vagrancy** (../refs/vagrancy.md): the road — a tree of fights where a row is its count of requirements, requirement kinds as a closed list, the chart drawn as bands with one route into each node.
- **Long Time Comin'** (../refs/long-time-comin.md): enemies and their orb patterns as data, boss phases, the diver mesh for the main character.
- **Chumworld Arena** (../refs/chumworld-arena.md): a ladder of opponents and the single-player campaign map; behaviour-tree pilots from data; boss and watch-mode lessons.

Model journals named by the person: Chumontillado's (`~/Documents/ChumworldShooter/chumontillado-journal`, mockups) and SnapClass's (`~/Documents/BuilderTrials/snapclass-1/snapclass-guide-journal`, animations with Play and step buttons and a text list under each). Style Lab (`~/Documents/BuilderRefinedArtstyle/style-lab`) for lifelike Blender characters and videos in journals.

## Words

Mission, mission kind, medal, tier, chart, mission list, horde, parkour, target, arena, boss, pack: `GLOSSARY.md`.

## Autonomy

The person asked the agent to build the journal, show it, then continue without stopping until the MVP is deployed; the plan is therefore approved in advance, and each question in the journal is built on its default until the person answers (an answer that differs becomes a TRIAGE row). Where this brief is silent, the agent decides, records the decision in `SECOND-ORDER.md`, and continues. The agent may not loosen a Builder requirement or a boundary test, and may not add a server, an account, analytics or any off-origin request. Online play from Chumontillado is removed: Choomstar is single player.

- Deploys: `when the checks pass`. The agent may create a public repository named `choomstar` under sgilson7 (the person wrote "Choomstar"; GitHub Pages addresses are lowercase, so the repository is `choomstar`) and publish it to GitHub Pages with `./scripts/publish.sh` once `./scripts/check.sh` passes.
- Subagents: `none` ("Do not use subagents for now").

## Journal

`default`

## Allowed core dependencies

`serde`, `serde_json` (reference default; Chumontillado's core uses the same two).
