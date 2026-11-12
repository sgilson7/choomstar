# Brief

Stage 1. The person's own words come first; the rest can be written by the agent from them and read by the person. `C1 Frame the problem and set the rules`, `B1 Establish context`.

## The description, word for word

Paste the description of the tool here unchanged: what you want, in your own words, as long or short as you like. A paraphrase is already an interpretation. Leave out names of people and employers if the repository will be public. Later requests in plain prose are numbered in `TRIAGE.md`.

> 

## The tool

What it does, who uses it, and on what device.

## Done means

The conditions the person will check before deploying. Make each one checkable.

- 

## Decisions that stay with the person

Content, data, anything pedagogical or specific to the person's field, any third-party asset or likeness, and anything outside `BUILDER-SPEC.md`.

- 

## Accessibility

How the tool carries information besides colour (shape, pattern, brightness, text), and the input devices it must work with.

## Style

`default`, or a path to a style pack, or a description of the look. With `default` or nothing, the art pipeline uses `styles/default/`.

## References

`none`, or earlier projects this one borrows from, one per line, with where their files are and what each lends, in the form `- **Ape Race** (../ape-race): the main menu and the online lobby`. When a project has a digest (`tools/digest.py`), name the digest instead: `- **Ape Race** (../refs/ape-race.md): the main menu and the online lobby`; `scripts/context_check.py` checks every digest named here.

The agent opens each one before planning and says in the opening entry what it took from it; `scripts/journal_check.py` fails while a name here is missing from that entry. A project named here is read, not copied wholesale: code comes in only under its licence, and art only with its owner's say.

## Words

Terms the person already uses in their field, with what they mean. They go into `GLOSSARY.md` before any string uses them.

## Autonomy

One paragraph: what the agent may do alone, what it must ask about, and what it may never do. State whether the plan is approved in advance and whether the agent may publish.

Two lines the agent reads literally:

- Deploys: `on my word` (the agent deploys when the person says so in the session; the default), `when the checks pass` (the agent publishes with `./scripts/publish.sh` after every passing check), or `never`.
- Subagents: `none` (the default: one agent does the work, because each subagent rereads the project and runs its own checks, which has cost 180,000 to 670,000 tokens apiece), or what they may do and how many at once.

A paragraph that has worked for one-day builds: *The plan is approved in advance. Where this brief is silent, the agent decides, records the decision in `SECOND-ORDER.md`, and continues. The agent may not loosen a Builder requirement or a boundary test, and may not add a server, an account, analytics or any off-origin request. The agent may create a public repository named ____ and publish it with `./scripts/publish.sh` once `./scripts/check.sh` passes.*

## Journal

`default`

Where the design journal lives. `default` is a folder beside this project, `../<this folder>-journal/`, kept as its own repository and never pushed; a relative path puts it elsewhere. Write `none`, with the reason, to build without a journal. With any other value the agent starts the journal and writes its opening entry before the plan, and `scripts/journal_check.py` fails while it is missing.

## Allowed core dependencies

`serde`, `serde_json` (reference default). Add others only here, with the reason.
