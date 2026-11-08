# Asks for X_eTaL-libraries

What this repository needs from
[X_eTaL-libraries](https://github.com/softwarewrighter/X_eTaL-libraries)
that is general (plan G3b: mathematics, matrices, integrators,
notation, plotting; what a repository with no interest in manifolds
would want too), reached at this repository's own pin
(`XETAL_LIBRARIES_COMMIT`, cloned into `work/xetal-libraries/`;
`../X_eTaL-libraries` is read only from here: its own plan takes
these). What is particular to manifolds is not asked for: it is
developed here in `libs/` (plan, "The functions the demos need").
Machine learning that is not about manifolds goes to
[`xetal-ML-asks.md`](xetal-ML-asks.md).

Each entry: status, kind, the ask with a minimal example in X_eTaL's
naming style (the signature wanted), which demos and steps here need
it, and the workaround in use (the expression written out in the
program where it is used, named on the demo's page). Status words as
in `xetal-ML-asks.md`: **not yet taken**, **planned**, **in
progress**, **landed**, **declined**, **blocker**.

Written 2026-11-07 (step research-foundation) from reading the
pinned libraries at 3465af3 (`work/xetal-libraries/libs/*/src`, their
export lists and doc comments): Matrix, Geometry, Graphs, Random,
Statistics, Trig, Plot, Lists, Check.

What exists at the pin and is used as it stands (no ask): Matrix's
`i_dentity`, `d_iag`, `t_race`, `m_ul`, `s_olve`, `i_nverse`, `d_et`,
`o_rthonormal`, `r_ank`, `p_roject`, `r_emainder`, `c_oordinates`,
`e_igenvalues`, `e_igenvectors`, `s_vd`, `s_ingular`, `t_runcate`;
Graphs' `w_eighted`, `u_ndirected`, `s_hortest` (the tree's distances
for the experiment kit), `l_evels`, `c_omponents`; Random's
`u_niform!`, `n_ormal!`, `s_huffle!`, `d_eal!` (seeded by `--seed`);
Statistics' `q_uantile`, `s_ummary`, `z_scores`, `c_ovariance`,
`c_orrelation`; Trig's `a_tan2`, `a_sin`, `a_cos`, the hyperbolic
functions, `h_ypot`; Plot's `s_catter`, `l_ine!`, `l_ines!`, `c_hart!`,
`l_inspace`, `f_unction!`; Lists' `r_aze`, `w_indows`, `d_eltas`;
Check.

| # | Status | Kind | Ask | Needed by | Workaround |
| - | ------ | ---- | --- | --------- | ---------- |
| GL1 | not yet taken | feature | Pairwise distances between rows in any dimension (Geometry's `ge:d_istances` takes points of the plane only): `mx:d_istances P`, the straight-line distance between every pair of rows of a matrix, `Num a => a -> Float`, a symmetric matrix with a zero diagonal; `P mx:d_istancesTo Q` for every row of P to every row of Q. Example: `mx:d_istances 3 2 r_eshape 0.0 0.0 3.0 4.0 0.0 4.0` gives `0.0 5.0 4.0 / 5.0 0.0 3.0 / 4.0 3.0 0.0` | sphere-hello (`u:c_hords`), baseline-knn, representation-explorer (the distortion against the data's distances), every demo that ranks neighbors | written out: the squared differences summed one coordinate at a time by `p_ower` (`demos/sphere-hello/sphere-hello.xtl`, `u:c_hords`), or `s_qrt '+ r_/ ((f_loat p) - f_loat q) ^ 2.0` for one pair (`u:s_traight`, manifold-tour) |
| GL2 | not yet taken | feature | Small array words every program here rewrites: `a ls:p_airs b` (two vectors side by side as the rows of an n by 2 matrix), `n ls:e_ach v` (every item repeated n times, in order) and `n ls:t_imes v` (v whole, n times), `ls:p_olyLength M` (the length of the polyline through the rows of M, segment by segment). Lists, or a Grid library. Example: `1 2 ls:p_airs 3 4` gives `1 3 / 2 4`; `2 ls:e_ach 1 2` gives `1 1 2 2`; `2 ls:t_imes 1 2` gives `1 2 1 2` | manifold-tour (`u:p_airs`, `u:e_ach`, `u:t_imes`, `u:l_ength`); the experiment kit's data generators; every page that builds a grid of coordinates | written out in `demos/manifold-tour/manifold-tour.xtl` (four one-line functions, named on its page) |
| GL3 | not yet taken | feature | Matrix scaling to doubly stochastic, Sinkhorn's iteration (Sinkhorn 1964; Sinkhorn and Knopp 1967), with its count and its errors: `k mx:s_inkhorn M` (M positive; rows normalized then columns, k sweeps; `Num a => Int -> a -> Float`), `mx:s_inkhornErrors M` (the largest absolute row sum error and column sum error, as a pair), and the nearest orthogonal matrix, the polar factor, `mx:p_olar M` (U times V from the SVD: `(Num a) => a -> Float`; the SVD is at the pin, so this is one line there and a workaround here). Example: `3 mx:s_inkhorn 2 2 r_eshape 1.0 3.0 2.0 1.0` has row and column sums within 0.01 of 1 | mhc-explorer (steps mhc-matrices, mhc-depth): the doubly stochastic and the orthogonal mixing, and the constraint violation measured | Mixing (this repository's library, step mhc-matrices) holds the iteration as a private function, written out as the plan's G3 allows while the ask is open, and `mx:s_vd` gives the polar factor in one line; the page names the workaround |
| GL4 | not yet taken | feature | Integrators of dx/dt = v(x, t), one step each, taking the field as a function: `(f, h) it:e_uler (x, t)` (one Euler step of size h), `(f, h) it:r_k4 (x, t)` (one fourth-order Runge-Kutta step), both `(a -> Float -> Float) -> ...` returning the new `(x, t)`, so that `n '{ s -> (f, h) it:r_k4 s } p_ower (x0, 0.0)` is a trajectory; and `(f, h) it:p_ath (x0, n)` giving the n + 1 points as rows. An Integrators library (X_eTaL-ML's `Integrals` is a macro library for definite integrals of one variable, a different thing). Example: `({ x t -> n_eg 1 s_elect x } ... )` on the rotation `(-y, x)` for one turn stays within 1e-3 of the circle with rk4 at h = 0.1 and spirals out with Euler | dynamics-explorer (steps vector-fields and manifold-flows): every trajectory drawn, the error of each integrator | written out in the demo's program (two small functions, each a `p_ower` loop), named on its page |
| GL5 | not yet taken | feature | A plane vector field's divergence and curl by central differences on a grid: `(f, h) fd:d_ivergence P` and `(f, h) fd:c_url P` for the field f at the rows of P (points of the plane), step h: `(a -> Float) -> Float -> a -> Float`. Example: on the rotation `(-y, x)`, divergence 0 and curl 2 at every point, within 1e-9 for h = 1e-4 | dynamics-explorer (step vector-fields): a gradient field (no curl) told from a rotation (no divergence), measured | written out: four evaluations of f per point, in the demo's program |
| GL6 | not yet taken | feature | Plot: a vector field as arrows on a grid, `(lo, hi, n) pl:f_ield! f` (an SVG picture as `l_ine!` gives, arrows scaled to the longest), trajectories over it `pl:p_aths! (field, rows...)`, and the contours of a function of two variables `(lo, hi, n, levels) pl:c_ontours! g` (marching squares at the given levels). Example: `(-2.0 2.0, 2.0 2.0, 9) pl:f_ield! { p -> (n_eg 2 s_elect p) c_at 1 s_elect p }` draws the rotation | optimization-explorer (the contours of `(x^2 + 3 y^2) / 2` and the descent paths over them, step metric-descent); dynamics-explorer (step vector-fields) | the page draws the arrows and the contours from the numbers the program prints (the field sampled on the grid, the contour segments computed by a marching-squares function written out in the program): presentation from printed numbers, as manifold-tour's maps are; named on the page |
| GL7 | not yet taken | feature | The Cholesky factor and the solve by it for a symmetric positive-definite matrix, `mx:c_holesky S` and `b mx:s_olveSpd S`, for the natural-gradient step (the Fisher matrix is SPD; `mx:s_olve` by Gaussian elimination works and is twice the cost, so this is an ask of cost and of a guard, not a blocker) | optimization-explorer (step metric-training) | `mx:s_olve` at the pin |
| GL8 | not yet taken | feature | Random numbers from an explicit seed, pure: `r:u_niformFrom (seed, n)` and `r:n_ormalFrom (seed, n)`, n Floats that depend on the seed and n alone (not on the run's `--seed`, nor on what was drawn before), so a data set is a function of its seed wherever it is made; Park and Miller's minimal standard (16807 modulo 2^31 - 1) or any stated generator, with Box and Muller for the normal ones. Example: `r:u_niformFrom (42, 3)` gives the same three numbers in every run | the experiment kit (every data set of `libs/Kit`), every comparison that holds its data equal across demos | written out in `libs/Kit` (`kt:u_niform`, `kt:n_ormal`, Park-Miller), named in its docs; the kit switches to Random's when it lands |

## Notes on the sorting

By plan G3 and CLAUDE.md rule 3: a function went here when a
repository with no interest in manifolds would want it. The three
borderline needs the plan's settled questions name keep their sorting
(the plan, "Settled questions"): Sinkhorn is general (GL3: optimal
transport and matrix scaling want it), the step that stays on a
manifold is this repository's (Manifold), and the preconditioned
optimizer step is X_eTaL-ML's (xetal-ML-asks GM4).

What was expected to be an ask and is not: Matrix's SVD, eigenvalues
and best rank-k approximation (at the pin since 3465af3, which is why
the pin was taken at HEAD: the polar factor, the singular values of a
mixing matrix and the spectral measures of H2 are one call each);
Graphs' shortest paths (the tree's distances); Trig's hyperbolic
functions (the Poincare ball).
