# Asks for X_eTaL-fine-tuning

What this repository needs from
[X_eTaL-fine-tuning](https://github.com/softwarewrighter/X_eTaL-fine-tuning),
the nearest sibling: its pages' teaching shell and its checklist, to
be shared and not copied (the user, 2026-11-06; `docs/plan.md`, G2).
`../X_eTaL-fine-tuning` is read only from here: its own plan takes
these. This repository has no pin of it and wants none; the asks are
written so that what is shared reaches both repositories through
X_eTaL-ML ([`xetal-ML-asks.md`](xetal-ML-asks.md), GM1).

Status words as in `xetal-ML-asks.md`. Written 2026-11-06 from
reading X_eTaL-fine-tuning at d78aca4; the statuses read again
2026-11-07 (step research-foundation) at X_eTaL-fine-tuning 8c3f245
and X_eTaL-ML 2d23a96.

| # | Status | Kind | Ask | Needed by | Workaround |
| - | ------ | ---- | --- | --------- | ---------- |
| GF1 | **answered through GM1** (X_eTaL-ML 501ab15 took `shared/lesson` in beside `shared/microscope`, configured by the dependent; its `shared/lesson/README.md` names this ask and says the tests run by hand with X_eTaL-fine-tuning as the dependent). X_eTaL-fine-tuning itself (8c3f245) still holds its own copy and has not switched; that is its choice and not needed here. No longer a blocker | refactoring | `shared/lesson` split into mechanism and content, and the mechanism offered to X_eTaL-ML. Details below | every live page here | none that does not copy the crate |
| GF2 | not yet taken (nothing in X_eTaL-fine-tuning 8c3f245 or X_eTaL-ML 2d23a96 answers it); not a blocker: cited at a commit | docs | `docs/new-eyes.md`, the checklist a page is written and read against: its general part (the reader's assumed knowledge, plain explanations, a worked calculation, code in asides, the cold reading) kept where both repositories cite one copy, apart from what is particular to the fine-tuning lessons | every page here; step lab-review | cited here by its URL at a named commit; not copied |

## GF1: the lesson shell, mechanism apart from content

X_eTaL-fine-tuning wrote the teaching shell and is its first user;
this repository would be the second. What ties the crate to its
repository is listed in `xetal-ML-asks.md` (GM1, "Why a second
repository cannot use it as it is"). Asked of X_eTaL-fine-tuning:

1. **Separate content from mechanism inside the crate**, whichever
   repository ends up holding it. Content, which stays
   X_eTaL-fine-tuning's: the rows of `glossary.tsv` about weights,
   adapters and rank; `chrome::REPO` and `chrome::SITE`; the lessons
   index built from its `demos/`. Mechanism, which is general: the
   frames and asides, the glossary's machinery and marked prose
   (`eli5`), `explain`, `code`, `mode`, `heat`, `stepper`,
   `arithmetic`, `narrate`, `predict`, `lesson.css`, `builtins.tsv`.
   Glossary rows that any X_eTaL page needs (a list, a matrix, a
   stage, a shape) could become a shared base.
2. **Offer the mechanism to X_eTaL-ML**, as ask FM5 offered the heat
   map's additions: a crate beside `shared/microscope`, configured by
   the dependent as GM1 sets out. X_eTaL-fine-tuning then depends on
   it at its own X_eTaL-ML pin and keeps its content (its glossary
   rows, its `Site`), losing nothing its pages do now; its tests of
   the shell move with the shell.
3. **Keep the six frames and their check as they are for its own
   pages**; what this repository needs is that the set can be added
   to by a dependent, not that X_eTaL-fine-tuning's pages change.

The order that disturbs least: X_eTaL-fine-tuning makes the split in
place (its pages unchanged, its gate passing); X_eTaL-ML takes the
mechanism; X_eTaL-fine-tuning moves its pin and deletes its copy;
this repository moves its pin and wires its own content (step
shell-adoption). Each is one repository's change, taken and tested
alone.

What this repository will then own, and nothing more: its glossary
rows (from `docs/glossary.md`), its `Site`, the extra parts of its
page arc (the intuition, the mathematics, the ML run, the analysis,
the research extension), and presentation particular to geometry (a
sphere and a disk drawn from X_eTaL's coordinates), which is offered
back if a sibling wants it.
