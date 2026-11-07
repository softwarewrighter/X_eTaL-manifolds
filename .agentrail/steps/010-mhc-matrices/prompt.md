Step mhc-matrices (docs/plan.md, Saga 1 row 10: the research's
milestone M3, first half; hypothesis H2; docs/research.txt part 1
section 4 and part 2 section 4; docs/papers.md, the rows on
hyper-connections and mHC, which must say "read at source" before
this step builds on them; glossary: polytope).

Build demos/mhc-explorer, live: track "constraints", hypothesis H2,
comparing free and doubly stochastic mixing of residual streams. This
step is the matrices; step mhc-depth is the network.

The question: when a network carries several streams of numbers side
by side and mixes them at every layer, what may the mixing be allowed
to do?

The program:
1. From one residual stream to several: `x <- x + F(x)`, then n
   streams and an n x n mixing matrix, as the papers define it (take
   the definitions from the papers, not from the research text, and
   cite them).
2. Doubly stochastic matrices of size 3: entries not negative, every
   row and every column summing to 1; nine entries, four free (count
   them on the page). The path from the identity to uniform mixing by
   one control, every matrix on it doubly stochastic (row and column
   sums printed).
3. Sinkhorn's iteration from a free positive matrix: rows, then
   columns, repeated; the row and column sum errors by iteration; how
   many iterations for a stated tolerance; one iteration worked in
   full.
4. What the constraint does to a signal: such a matrix keeps the mean
   of the streams and shrinks their differences; singular values
   (Matrix's) of matrices along the path; a vector mixed 1, 10, 100
   times, its mean and its spread at each; set beside a free matrix
   whose powers blow up or die away, and an orthogonal one that keeps
   length and may have negative entries.
5. The set itself: the Birkhoff polytope is a polytope, flat faces
   and corners (the permutation matrices), not a smooth manifold
   everywhere; a 2 x 2 case is a segment and can be drawn whole. The
   page says why the paper's word is "manifold-constrained" and what
   this repository calls the set (CLAUDE.md rule 6).

The page, in the arc of principle 6: streams drawn as parallel
channels, mixing matrices as heat maps, the errors and singular
values as charts, each from the run. Actual mHC learns matrices that
depend on the input; this page's single control does not: say so.

Libraries (CLAUDE.md rule 3): libs/Mixing is begun here (the alias
`hc:` unless a sibling's library uses it): the residual block over n
streams with a mixing matrix, the kinds of mixing (free, doubly
stochastic, orthogonal) as the papers define them, and the
constraint's violation (row and column sum errors, distance from
orthogonality); typed, documented, tested, with a small demo.
Sinkhorn's iteration itself is matrix scaling, X_eTaL-libraries' by
G3 (the ask step research-foundation wrote): use it if landed at the
pin, otherwise written out in Mixing's demo program and named as the
workaround, never silently made a Mixing function.

Tests: sums and non-negativity along the path, Sinkhorn's errors
decrease to tolerance, the mean is kept exactly within tolerance,
singular values at most 1, every symbol and control in the glossary,
the browser check, baselines. A cold reading.

README, the demo's README, CHANGES.md, the plan; publish and
check-live.
