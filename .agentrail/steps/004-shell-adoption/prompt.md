Step shell-adoption (docs/plan.md, Saga 1 row 4; decision G2;
principle 6; CLAUDE.md rules 3c and 9; docs/xetal-ML-asks.md GM1;
docs/xetal-fine-tuning-asks.md GF1).

Bring the shared teaching shell into this repository and publish the
first live pages. This step exists so that the shell is shared and
not copied.

First, find out where the asks stand: read X_eTaL-ML and
X_eTaL-fine-tuning at their HEADs (their plans, CHANGES.md, any file
answering this repository's asks, and the shared crate itself if it
exists).

If GM1 has NOT landed (there is no shared lesson crate in X_eTaL-ML
that a dependent can configure, and no agreed alternative recorded in
the asks files): do not copy X_eTaL-fine-tuning's shared/lesson, in
whole or in part, and do not build pages on microscope alone as a
substitute. Bring the asks files up to date with what you found, mark
the step blocked (`agentrail abort` with the reason), commit, and
tell the user what is missing and in which repository. Stop there.

If it has landed:
1. Move the pin: `just ml-pin` to the commit that carries it, alone,
   `just gate --full`, the pin file in its own commit. If X_eTaL-ML's
   answer differs from what GM1 asked (names, settings), follow what
   landed and correct the asks file and CLAUDE.md to match.
2. Wire the shell to this repository: the site (name, URL, commit,
   links), the pins the footer shows, the root and the glossary
   settings in .cargo/config.toml, MICROSCOPE_LIBS naming this
   repository's libs/ and the pinned X_eTaL-libraries, the stylesheet.
3. shared/glossary.tsv: this repository's rows, made from
   docs/glossary.md (one source: say which file is edited and how the
   other is made from it, with a check in the gate that they agree).
4. The page arc of principle 6 as frames: the shell's six kinds and
   this repository's added parts (the intuition, the mathematics, the
   experiment, the ML run, the analysis, the research extension), and
   the browser check requiring them of every page; a demo's README
   checked for the same parts.
5. The comparison table of a page: choices against measures, each
   cell from the X_eTaL run and marked measured or analytical, with
   the shell's sentence for a change (comparisons only, no verdict).
   Presentation only (rule 3a). If the shell has a part that does
   this, use it; if it is general and missing, it is an addition to
   the shared crate, asked for (a new GM row), with a thin version
   here meanwhile only if that is not a copy of anything.
6. The first live pages: sphere-hello (the two distances for a pair
   the reader picks, the neighbors under each, the plain mean and the
   mean on the sphere; flat pictures drawn from X_eTaL's coordinates;
   a turning view through the shared view of ask GM3 if it has
   landed at the pin, the flat views alone if not; plan G13) and baseline-knn (the three data sets, the accuracy
   and distortion table). Each in the full arc, every term a glossary
   entry, read cold by a reviewer given only its text and revised.
7. The catalog page; `just pages`, `just serve-pages` on port 3285,
   `just publish`, `just check-live`; the browser baselines.
8. The asks files: GM1 and GF1 marked landed with the commit; what
   was learned in wiring recorded.

README (Demos with the live links, Build, Status), CLAUDE.md (the
rule 3c sentence that pages wait, removed), CHANGES.md, the plan.
