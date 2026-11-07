Step metric-training (docs/plan.md, Saga 1 row 9: milestone M2,
second half; the research's experiment A, "Learn the metric, not just
the weights", only its first rung; docs/papers.md, the rows on the
natural gradient, read at source first).

Extend demos/optimization-explorer from a bowl to a model: does a
metric-aware step help training, and is it worth what it costs?

Deliver:
1. One small model small enough that its Fisher matrix can be formed
   and solved exactly (logistic regression or a tiny network on the
   flat data of step experiment-kit, badly scaled on purpose in one
   preset so that the metric has something to fix).
2. Three ways to train it, on the same data, split, seeds, start and
   number of steps: plain gradient descent; a diagonal preconditioner;
   a natural-gradient step (the Fisher matrix as the metric, solved
   with the Matrix library, damped, the damping stated). The Fisher
   matrix checked in a test against its definition on a tiny case.
3. Measures: loss by step, held-out accuracy, steps to a target loss,
   the size of each update, and the cost of a step counted (numbers
   stored, operations for the solve), so that "fewer steps" is set
   against "dearer steps". The same comparison with the inputs well
   scaled, where the metric should buy little: show it.
4. The Fisher metric as a geometry: the same small change of the
   weights measured in the flat metric and in the Fisher metric; why
   the natural step does not depend on how the weights are scaled,
   shown by rescaling an input and rerunning (plain descent's path
   changes, the natural one's predictions do not, within tolerance;
   run this check undamped and say that damping gives the property
   up).
5. On the page: the three loss curves on one scale, the table, the
   narrator's sentence from measured values, and the limits said: a
   model this small hides the real cost (the matrix grows with the
   square of the number of weights), and Adam's diagonal is a cousin
   of the preconditioner, not the natural gradient.
6. docs/experiments.md: the optimization results, including the case
   where the metric did not pay. A learned metric (the research's
   G_phi) is NOT built here: record what this step's results suggest
   for it under Saga 2.

Libraries (CLAUDE.md rule 3): the Fisher matrix of the model and
the natural-gradient step are X_eTaL-ML's by G3 (the ask step
research-foundation wrote): use them if landed at the pin, otherwise
written out in the program, kept short, with Matrix's solve.

Tests, the cold reading, baselines; README, the demo's README,
CHANGES.md, the plan; publish and check-live.
