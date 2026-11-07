Step mhc-depth (docs/plan.md, Saga 1 row 11: milestone M3, second
half; hypothesis H2; the research's experiment B, "Compare constraint
spaces for residual mixing").

Extend demos/mhc-explorer from matrices to networks: does holding the
mixing to a constraint set help a deep stack carry a signal and
train?

Deliver:
1. A stack of small residual blocks over n streams (n and the width
   small; depth the control: for example 4, 16, 64), with four kinds
   of mixing at equal trained numbers where that can be arranged, and
   the difference stated where it cannot: a plain residual (the
   identity), free mixing, doubly stochastic mixing (Sinkhorn of free
   numbers, a stated number of iterations), orthogonal mixing (a
   stated parameterization). The block's definition from the papers;
   the mixing here does not depend on the input unless the paper's
   toy case can be afforded: say which.
2. Propagation at the start of training, by depth: the size of the
   signal going forward and of the gradient coming back, layer by
   layer, for each kind, over several seeds; constraint violation
   (row and column sum errors; distance from orthogonality) by layer.
3. Training: one small task from step experiment-kit, the same data,
   split, seeds and number of steps for each kind and each depth:
   loss by step, held-out accuracy, whether the run stayed finite,
   the cost of a step counted (Sinkhorn's iterations are not free).
   Gradients through the mixing checked against central differences.
4. The H2 table in docs/experiments.md and on the page: kind of
   mixing by depth: signal and gradient size, trained or failed,
   accuracy, cost. Set against what docs/research-questions.md wrote
   beforehand. Say where a constraint did not help (shallow stacks
   may show nothing), what each costs, and that stability of the
   mixing does not make a whole network stable (the research's own
   caution). Averages that shrink differences may also wash out
   information: measure it (the spread across streams by depth).
5. The page: the stack drawn with a strip per layer colored by signal
   size, the four kinds side by side on one scale; a depth control
   whose effect is explained from measured values.

Libraries (CLAUDE.md rule 3): the stack of blocks, the size of the
signal and of the gradient by layer, and the gradient through each
kind of mixing join libs/Mixing here, every gradient checked against
central differences; training steps are Optim's, the network's
pieces NN's.

Tests, the cold reading, baselines; README, the demo's README,
CHANGES.md, the plan (Saga 3's candidates updated from what was
learned); publish and check-live.
