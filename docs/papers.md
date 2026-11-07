# Papers

The catalog (plan G12): every paper `docs/research.txt` names or
leans on, and the standard sources the first four explorers need.
No paper is tracked in this repository (the user, 2026-11-06): each
row keeps the link and the notes made from reading it. Nothing is
built on a paper known only through the research text; a row says how
the paper was read: **at source** (the full text, from the publisher
or arXiv), **abstract** (the abstract and whatever front matter was
open), or **not found**.

Read 2026-11-07 (step research-foundation), each on the web that day
by an agent given the research's description and told to report
where it was wrong; nothing here is from memory. Where the research's
account differs from the paper, the correction is in the row and
collected at the end. Today's date matters: several of these are from
2026 and may still change (a camera-ready differing from an arXiv
version is noted where seen).

Columns: the paper; the claim `docs/research.txt` makes of it and
whether it held when read; the setting and assumptions; the notes;
what this repository plans to build from it.

## Hyper-connections and mHC (track: constraints)

### Hyper-Connections

- Defa Zhu, Hongzhi Huang, Zihao Huang, Yutao Zeng, Yunyao Mao, Banggu
  Wu, Qiyang Min, Xun Zhou (ByteDance Seed). arXiv 2409.19606 (v1 29
  Sep 2024, v3 18 Mar 2025); ICLR 2025.
  <https://arxiv.org/abs/2409.19606>. Read at source (the main text
  and appendices through K).
- The research: several residual streams with learned mixing, which
  mHC constrains. **Holds.** The hidden vector is copied n times (the
  "hyper hidden matrix"); an (n + 1) by (n + 1) learned matrix per
  layer holds the write-back weights, the read weights and the n by n
  stream mixing; mHC's own abstract describes itself as a constrained
  HC.
- Setting: no constraint on the mixing at all (static, SHC, or
  input-dependent through a small tanh term, DHC), initialized so the
  network equals Pre-Norm. OLMo 1B and 7B and an MoE, 500B tokens
  each; ViT and DiT for vision.
- Notes: the motivation is the Pre-Norm/Post-Norm seesaw between
  gradient vanishing and representation collapse; collapse is measured
  as the cosine similarity of adjacent layers' inputs (gradient
  vanishing is not measured). OLMo-1B loss 2.811 to 2.781 (DHC, n =
  4); n = 1 is worse than the baseline; 7B loss 2.581 to 2.559; memory
  +26 to 28 percent for dense DHC at n = 4. No limitations section.
- Here: the "free" mixing of H2 (mhc-explorer), the control against
  mHC; its n = 1 result is why the experiment keeps n = 4.

### mHC: Manifold-Constrained Hyper-Connections

- Zhenda Xie, Yixuan Wei, Huanqi Cao, Chenggang Zhao, Chengqi Deng,
  Jiashi Li, Damai Dai, Huazuo Gao, Jiang Chang, Kuai Yu, Liang Zhao,
  Shangyan Zhou, Zhean Xu, Zhengyan Zhang, Wangding Zeng, Shengding
  Hu, Yuqing Wang, Jingyang Yuan, Lean Wang, Wenfeng Liang
  (DeepSeek-AI; the ICML listing differs in one name). arXiv
  2512.24880 (v1 31 Dec 2025, v2 5 Jan 2026); ICML 2026 (poster).
  <https://arxiv.org/abs/2512.24880>. Read at source.
- The research: the mixing matrices constrained to be doubly
  stochastic (the Birkhoff polytope) by a Sinkhorn projection;
  stability of signal and gradient through depth. **Holds, with one
  nuance:** the Sinkhorn-Knopp projection is stopped at 20 iterations,
  so the constraint is approximate; the paper's own measured composite
  gain is about 1.6, not 1.
- Setting: n = 4 streams; x_{l+1} = H_res x_l + H_post^T F(H_pre x_l);
  only H_res (n by n) is constrained (nonnegative, rows and columns
  summing to 1); H_pre = sigmoid in (0, 1)^n and H_post = 2 sigmoid in
  (0, 2)^n are not stochastic; each is a static bias plus a dynamic
  projection of the normalized stream, gated by a scalar starting at
  0.01. MoE models of 3B, 9B, 27B on 39B, 105B, 262B tokens.
- Notes: unrolling HC gives a product of unconstrained H_res; the
  "Amax gain" (largest absolute row sum forward, column sum backward)
  of the composite reaches about 3000 in the 27B HC model against about
  1.6 for mHC; HC shows a loss surge near step 12k, mHC does not.
  Sinkhorn as used: exp of the logits, then alternate row and column
  normalization, 20 sweeps. Stated theory: a doubly stochastic matrix
  has spectral norm at most 1; products of doubly stochastic matrices
  are doubly stochastic; the polytope is the convex hull of the
  permutations. 27B: BBH 43.8 / 48.9 / 51.0 (baseline / HC / mHC),
  MMLU 59.0 / 63.0 / 63.4, loss 0.021 below the baseline; mHC beats HC
  on every benchmark but MATH; 6.7 percent time overhead at n = 4.
  No limitations section.
- Here: H2's doubly stochastic mixing, the gain measure, the Sinkhorn
  sweep count as a control and the row and column sum errors measured
  (mhc-explorer, steps mhc-matrices and mhc-depth). The glossary calls
  the set a polytope, as the paper's own footnote allows.

### KromHC: Manifold-Constrained Hyper-Connections with Kronecker-Product Residual Matrices

- Wuyang Zhou, Yuxuan Gu, Giorgos Iacovides, Danilo Mandic (Imperial
  College London). arXiv 2601.21579 (v1 29 Jan 2026, v2 1 Jun 2026);
  ICML 2026 (poster); code <https://github.com/wz1119/KromHC>.
  <https://arxiv.org/abs/2601.21579>. Read at source.
- The research: Kronecker products of smaller doubly stochastic
  matrices cut parameters while keeping the constraint exact.
  **Holds.**
- Setting: the n by C stream reshaped to a tensor with n = product of
  i_k; H_res = U^K kron ... kron U^1, each factor an i_k by i_k doubly
  stochastic matrix written as a softmax-weighted convex combination
  of its i_k! permutations (Birkhoff-von Neumann), so exact; the
  Kronecker product of doubly stochastic matrices is doubly stochastic
  (their Theorem 4.2). Parameters O(n^2 C) against mHC's O(n^3 C).
  Models of 60M and 186M parameters on FineWeb-Edu, single runs.
- Notes: their critique of mHC: 20 sweeps are inexact, the column-sum
  error of a 24-layer product about 0.05 against zero for KromHC. At
  12 blocks: loss 2.971 (residual), 2.964 (mHC), 2.972 (mHC-lite),
  2.966 (KromHC) with added parameters 1844K, 2433K, 959K: "on par"
  with half the parameters. Limitation stated: n with a large prime
  factor.
- Here: the exact alternative to Sinkhorn for H2 (a convex combination
  of permutations, which Mixing can write for n = 3 or 4 directly) and
  the measure of Sinkhorn's inexactness; constraint-lab (Saga 3) for
  the Kronecker form itself.

### The Birkhoff Geometry of Manifold-Constrained Hyper-Connections: Two Channels, Vertex Viscosity, and Sinkhorn as a Retraction

- Xiaoyu Li (UNSW), Zhizhou Sha (UT Austin), Chiwun Yang (City
  University of Hong Kong). arXiv 2610.06653 (5 Oct 2026); no venue.
  <https://arxiv.org/abs/2610.06653>. Read at source (the HTML ends in
  Appendix F).
- The research: "mean/difference channels, contraction, and Sinkhorn
  geometry; information retention". **Partly.** The channels, the
  contraction and Sinkhorn as a retraction are the paper; "information
  retention" is not a quantity in it (no entropy or mutual
  information): the notion is a fading-memory horizon, tau = 1 / (1 -
  sigma_2) layers, for the difference channel, the mean channel being
  kept exactly.
- Setting: theory only, no model trained (n = 4, a laptop). The
  stream split into its mean across streams and the differences
  (x = 1 xbar^T + x_perp).
- Notes: Theorem 3.1, on the mean channel mHC is an ordinary residual
  network with gain exactly 1 for every doubly stochastic mixer.
  Theorem 3.2, the second singular value of a mixer is at most 1 - n
  times its smallest entry, so composites contract the differences
  toward the uniform average. Proposition 3.4, among nonnegative
  mixers only permutations avoid the collapse. Lemma 4.1 to Theorem
  4.3: Sinkhorn of exp of the logits is a global chart; the logit
  gradient is the Fisher-Rao gradient; the straight-through update is
  a Fisher-Rao (entropic mirror) flow and its step is Sinkhorn as a
  retraction. Vertex viscosity: logit gradient flow approaches or
  escapes a permutation at rate 1/t, mirror flow exponentially.
  Proposition 6.1: Sinkhorn's local convergence factor is sigma_2^2,
  about 1.5 sweeps per unit of horizon (error 1e-3 at horizon 5 needs
  13 sweeps). Section 7: isometric mean-preserving mixers keep the
  differences at the price of negative entries, "an empirical question
  if worth its cost". Limitations: no training; constants not sharp.
  Carries an AI-use statement.
- Here: H2's two-channel measure (the mean kept, the differences
  shrunk) and the sweeps-against-horizon table (mhc-explorer, step
  mhc-matrices); what counts against H2 (the collapse of the
  differences) is taken from its Proposition 3.4.

### xHC: Expanded Hyper-Connections

- Xiangdong Zhang, Xiaohan Qin, Sunan Zou, Tuo Dai, Xiaoming Shi,
  Huaijin Wu, Yebin Yang, Zhuo Xia, Shaofeng Zhang, Lin Yao, Yuliang
  Liu, Yu Cheng, Junchi Yan (SJTU and others; the NeurIPS listing
  differs). arXiv 2607.14530 (16 Jul 2026); NeurIPS 2026 (poster; the
  camera-ready adds a 69B result the arXiv version lacks).
  <https://arxiv.org/abs/2607.14530>. Read at source.
- The research: expanded residual streams and sparse updates.
  **Holds.**
- Setting: N = 16 streams, k = 4 written per sublayer (2 fixed, 2
  chosen by a router), every stream read; the residual map keeps mHC's
  20 Sinkhorn sweeps; the write-back enriched by three causal
  depthwise convolutions, orthogonalized (not on attention, which was
  unstable). MoE models of 18B and 28B total parameters.
- Notes: cost O(k^3 C) against O(N^3 C). 18B: 40.6 / 44.8 / 48.8
  (vanilla / mHC / xHC) at +4.1 percent FLOPs. 10B ablations (val
  loss): vanilla 2.029, mHC N = 4 2.004, N = 16 1.998, xHC 1.983.
  Their mHC reimplementation costs about 15 percent wall-clock, not
  DeepSeek's 6.7.
- Here: constraint-lab (Saga 3), stream count against cost; not in
  Saga 1.

### Manifold-Constrained Hyper-Connections for Parameter-Efficient Finetuning

- Valentijn Oldenburg, Floris de Kam, Bente Zuijdam, Lieve Eberson,
  Nicky van Zutphen, Stef de Wildt, Ivo Verhoeven, Cees Snoek
  (University of Amsterdam). arXiv 2607.18130 (v1 20 Jul 2026, v2 7
  Oct 2026); NeurIPS 2026 AXIOM workshop (a workshop paper, not the
  main track); code <https://github.com/valentijn7/mHC_PEFT>.
  <https://arxiv.org/abs/2607.18130>. Read at source.
- The research: residual routing for fine-tuning with LoRA; mHC alone
  does not consistently beat LoRA. **Holds:** "As a standalone PEFT
  method, mHC does not consistently outperform LoRA."
- Setting: frozen OLMo-2 1B and 7B, every sublayer wrapped; Tulu-3 SFT
  mixture, 20k steps; LoRA, VeRA, (IA)^3 and prompt tuning as
  baselines; no full fine-tuning baseline.
- Notes: 1B test loss: frozen 2.00, static Sinkhorn mHC with 772
  parameters 1.79, LoRA r = 8 1.28, mHC n = 16 plus LoRA 1.21. The
  key finding: an identity mixing beat a learned one at every stream
  count (n = 16: 1.27 against 1.32). At 7B the combination gains 0.001
  over LoRA alone. The last layer's Sinkhorn mixing collapses to the
  uniform average. Limitations stated: one model family, one mixture,
  few seeds.
- Here: geometric-weight-updates (Saga 4, with X_eTaL-fine-tuning);
  its identity-beats-learned finding is a null result H2's page cites
  as a caution.

## Geometry of representations and dynamics (tracks: representation, dynamics, adaptive)

### Neural Feature Geometry Evolves as Discrete Ricci Flow

- Moritz Hehl, Max von Renesse, Melanie Weber. arXiv 2509.22362 (26
  Sep 2025); ICML 2026 (poster); code
  <https://github.com/Weber-GeoML/Ricci-Flow_Feature-Geometry>.
  <https://arxiv.org/abs/2509.22362>. Read at source (the main text
  and part of the appendix).
- The research: feature geometry through the layers resembles discrete
  Ricci flow; graph-geometry changes relate to class separability.
  **Holds.** The test is a per-vertex correlation between the change
  of neighborhood distances across a layer and the neighborhood's
  Ollivier-Ricci curvature, negative meaning Ricci-flow-like; negative
  on every real data set (MNIST 1 vs 7: -0.58, 88.7 percent of
  vertices; CIFAR cars vs planes: -0.76). Class separability
  corresponds to community structure (modularity up, normalized cut
  down).
- Setting: binary classification by feedforward ReLU networks; the
  data manifold approximated by k-NN graphs on each layer's features;
  Ollivier-Ricci (a linear-time approximation) and Forman curvature;
  the authors say there is no canonical discrete Ricci flow on graphs.
  Over 20,000 networks, 50 seeds, widths 15 to 50, depths 7 to 15.
- Notes: the local coefficient is near 0 at initialization and
  negative after training (a global coefficient from earlier work is
  negative already at initialization, which they attribute to degree
  correlation). An early-stopping heuristic from the coefficient's
  reversal is motivated, not benchmarked. Theorems: a random Gaussian
  layer preserves the k-NN graph with high probability; ReLU can
  rewire it. Limitations: small data, feedforward only.
- Here: Saga 4 (topology-and-dimension): the neighborhood graph of a
  small network's hidden representations tracked as it learns. Not in
  Saga 1.

### Geometric Neural Ordinary Differential Equations: From Manifolds to Lie Groups

- Yannik P. Wotte, Federico Califano, Stefano Stramigioli (Twente).
  Entropy 27(8):878, 19 Aug 2025; DOI 10.3390/e27080878; PMC12385718.
  <https://doi.org/10.3390/e27080878>. Read at source.
- The research: a framework for neural ODEs on manifolds and Lie
  groups, intrinsic and extrinsic. **Holds**, but note: the paper has
  no experiments and no numbers; it is a unified derivation that
  "doubles as a tutorial", and most of the Lie-group content
  reorganizes prior work (its Table 1 marks what is new).
- Setting: a neural ODE on a smooth manifold (charts, a Riemannian
  metric for gradients) or a matrix Lie group; the adjoint method,
  optimize-then-discretize only.
- Notes: extrinsic (embed the manifold, a tangent vector field)
  against intrinsic (charts, or the Lie algebra); the adjoint with a
  running cost by an augmented state (new); the adjoint on a matrix
  Lie group in Hamiltonian form, left-trivialized, with a co-state in
  the dual Lie algebra, for which no off-the-shelf integrator exists.
  Limitations: chart validity, stiff adjoints, implementation left to
  future work.
- Here: the words for dynamics-explorer's two settings (a flow in the
  room against one held to the sphere by Manifold's tangent step:
  the extrinsic formulation); nothing numerical is reproduced.

### ODE-based learning on manifold and its expressive power

- Hirotada Honda (Toyo University). Advances in Continuous and
  Discrete Models 2026:25, 29 Jan 2026; DOI
  10.1186/s13662-026-04057-4; open access; no arXiv version.
  <https://link.springer.com/article/10.1186/s13662-026-04057-4>.
  Read at source.
- The research: which smooth transformations flows on manifolds
  represent, and when they are learnable. **Holds, narrower than it
  sounds:** the paper poses both questions but answers the
  "unrestricted" one (a flow on a larger space: a principal bundle
  with a compact structure group and a trivial homotopy set, or any
  closed manifold after embedding in one more dimension) and leaves
  flows on the same closed manifold open. No experiments.
- Setting: time-dependent C^r vector fields, r at least 3; learnability
  in the Shalev-Shwartz and Ben-David sense (Glivenko-Cantelli
  classes).
- Notes: every equivariant diffeomorphism is a time-1 map when the
  homotopy set is trivial and the class is non-uniformly learnable;
  every diffeomorphism of a closed manifold embedded in R^d is the
  restriction of a flow on R^{d+1}, orientation-reversing ones with a
  half turn; x + Psi(x) with a one-hidden-layer Psi is exactly an
  autonomous neural ODE on R^{2d}. Limitations stated: compact Lie
  groups, diffeomorphisms only, no cost constraints.
- Here: the fact manifold-flows teaches (a flow run backward cannot
  tear one piece into two; adding a dimension lets it do what it could
  not), cited for the statement, not reproduced.

### When task performance deceives: Task-geometry decoupling in learnable-curvature hyperbolic GNNs

- Lixian Chen, Jingchao Wang, Zhaorong Dai, Hanqian Liu, Danxiang Ai,
  Yang Shi. Neural Networks 203:109172 (November 2026 issue; online 29
  May 2026; SSRN preprint 13 Oct 2025); DOI
  10.1016/j.neunet.2026.109172; code
  <https://github.com/JAC4861/adaptive-k-controller>.
  <https://doi.org/10.1016/j.neunet.2026.109172>. **Abstract** and the
  open front matter only (the full text is paywalled; the SSRN PDF did
  not open).
- The research: task performance does not optimize geometric fidelity;
  a curvature control from distortion feedback. **Holds as far as
  visible:** "task-geometry decoupling" (their Definition 1: after the
  task saturates, the curvature and the distortion keep drifting) and
  a "Distortion-Aware Adaptive Controller" using stress as feedback on
  one global curvature.
- Setting: hyperbolic GCN on the Poincare ball, one global curvature
  (the authors stress the restriction); link prediction on Cora,
  PubMed, Airport, Disease.
- Notes: their Figure 1: task saturates near 400 epochs while
  distortion rises after the "decoupling point". The preprint's "up to
  95 percent" reduction of curvature variance is softened to
  "substantially" in the journal abstract. Builds on Pei et al.
  (NeurIPS 2020) curvature regularization. The theory and the tables
  could not be checked.
- Here: H3's second half (the deception) is this paper's claim, tested
  at toy scale in Saga 2 (learned-curvature) with distortion measured
  beside the loss; until then the page calls it a claim read in
  abstract.

### "Adaptive manifold curvature regularization"

- **Not found** as a distinct paper. The research names no title; no
  ScienceDirect article carries the phrase. The most plausible reading
  (confidence about half) is that it is the task-geometry paper above
  counted twice. Candidates seen and not the one described: Pei, Wei,
  Chang, Zhang, Yang, "Curvature Regularization to Prevent Distortion
  in Graph Embedding", NeurIPS 2020, arXiv 2011.14211 (a fixed
  regularizer, not adaptive; abstract read); Fu et al., "ACE-HGNN:
  Adaptive Curvature Exploration Hyperbolic Graph Neural Network",
  ICDM 2021, arXiv 2110.07888 (adaptive by reinforcement learning, not
  a regularizer; abstract read).
- Nothing in the plan rests on it: H3 stands on Gu et al. and on the
  task-geometry paper.

### Learning Mixed-Curvature Representations in Product Spaces

- Albert Gu, Frederic Sala, Beliz Gunel, Christopher Re (Stanford).
  ICLR 2019 (poster); OpenReview HJxeWnCcF7; the camera-ready PDF's
  title reads "...in Products of Model Spaces".
  <https://openreview.net/forum?id=HJxeWnCcF7>. Read at source (the
  forum in full; the PDF's method and results pages).
- The research names "mixed-curvature geometric learning" without a
  title; this is the foundational paper. **Holds.** Correction: the
  research's author list elsewhere ("Ganea") is wrong; it is Gunel.
- Setting: embed a finite metric (graph shortest paths; word
  co-occurrence) in a product of spherical, hyperbolic and flat
  factors, the curvature of each factor learned (through its radius);
  a distortion loss in squared distances; Riemannian SGD per factor
  (the Euclidean gradient projected to the tangent space, corrected by
  the metric, an exponential map). The signature (which factors) is
  chosen by a discrete sectional-curvature heuristic from triangles,
  not learned. Total dimension 10 for graphs.
- Notes: at dimension 3, a cycle embeds best in S^3, a tree in H^3, a
  ring of trees in H^2 x S^1. At dimension 10: Facebook best in H^5 x
  S^5 (distortion 0.0402, 32.6 percent better than the best single
  space); CS PhDs best in (H^2)^5. Word analogies: the flat space wins
  at dimension 100. The Karcher mean converges at a linear rate under
  a hemisphere condition. Reviewers flagged notation; the signature is
  not learned end to end.
- Here: H1's three spaces at equal dimension and its distortion
  measure (representation-explorer); H3's mixed-curvature arm (Saga
  2). The sectional-curvature heuristic from triangles is a candidate
  measure for the experiment kit.
- A counterpoint, abstract only: Haitz Saez de Ocariz Borde, "On the
  Expressive Power of Mixed-Curvature Representations in Product
  Manifolds", ICLR 2026 workshop GRaM (OpenReview WGDhHJKyll): generic
  Riemannian manifolds cannot be locally isometrically represented by
  products of constant-curvature factors.

## Standard sources

### Poincare Embeddings for Learning Hierarchical Representations

- Maximilian Nickel, Douwe Kiela (Facebook AI Research). NIPS 2017;
  arXiv 1705.08039 (22 May 2017). <https://arxiv.org/abs/1705.08039>.
  Read at source.
- Leaned on for: hyperbolic embeddings of hierarchies and Riemannian
  SGD on the Poincare ball. **Partly:** the hierarchy embedding and
  link prediction hold (WordNet reconstruction MAP 0.823 at dimension
  5 against 0.024 flat); **the paper reports no distortion**, only
  mean rank and MAP; and its optimizer is the additive retraction with
  a projection back into the ball (the gradient scaled by (1 -
  |x|^2)^2 / 4, which the paper itself calls the natural gradient),
  not the exponential map: "a full Riemannian optimization approach"
  is left to future work.
- Setting: the open unit ball with metric 2 / (1 - |x|^2) times the
  flat one; distance arcosh(1 + 2 |u - v|^2 / ((1 - |u|^2)(1 -
  |v|^2))); a softmax ranking loss with sampled negatives.
- Notes: a tree of branching b has exponentially many nodes per level,
  and in H^2 a circle's length 2 pi sinh r grows exponentially, so
  trees embed in two dimensions; hierarchy read from the norm, similarity
  from distance.
- Here: the tree data set and its embedding in the disk (H1); the
  distortion measure is this repository's addition, after Gu et al.

### Hyperbolic Neural Networks

- Octavian-Eugen Ganea, Gary Becigneul, Thomas Hofmann (ETH Zurich).
  NeurIPS 2018; arXiv 1805.09112. <https://arxiv.org/abs/1805.09112>.
  Read at source.
- Leaned on for: Mobius addition, the exponential and logarithmic maps
  on the Poincare ball. **Holds** (their equations 6, 7, 8, 12, 13,
  15): x (+) y = ((1 + 2 c x.y + c |y|^2) x + (1 - c |x|^2) y) / (1 +
  2 c x.y + c^2 |x|^2 |y|^2); d(x, y) = (2 / sqrt c) atanh(sqrt c
  |(-x) (+) y|); exp_x(v) = x (+) tanh(sqrt c lambda_x |v| / 2) v /
  (sqrt c |v|) with lambda_x = 2 / (1 - c |x|^2); the geodesic x (+)
  ((-x) (+) y) (x) t.
- Notes: Mobius addition is neither commutative nor associative;
  parallel transport from the origin is a scaling by lambda_0 /
  lambda_x. Numerical cautions: points projected to radius 1 - 1e-5,
  tanh arguments clipped. The gyrovector sources cited are Ungar's
  2008 book and lectures, not the 2005 book.
- Here: Manifold's `b_allDistance` and `b_allGeodesic` are these
  formulas at c = 1 (step manifold-tour); the ball's Project, Tangent,
  Step, Arrow and Mean follow them (step representation-hyperbolic).

### Natural Gradient Works Efficiently in Learning

- Shun-ichi Amari (RIKEN). Neural Computation 10(2):251-276, 1998;
  DOI 10.1162/089976698300017746.
  <https://doi.org/10.1162/089976698300017746>. **Abstract** (no open
  full text); corroborated by reading at source its predecessor,
  "Neural Learning in Structured Parameter Spaces: Natural Riemannian
  Gradient", NIPS 1996.
- Leaned on for: the natural gradient as the gradient under the Fisher
  metric. **Holds, with a qualification:** the natural gradient is the
  steepest direction under the Riemannian metric of the parameter
  space, G^{-1} times the gradient; for a space of probability
  distributions that metric is the Fisher information (1996, Lemma 1
  and equation 11); the 1998 paper also treats matrix spaces and
  linear systems, where the metric is their own invariant one.
- Notes: natural-gradient online learning with rate 1/t is Fisher
  efficient (attains the Cramer-Rao bound); the 1998 abstract suggests
  the plateaus of backpropagation might disappear.
- Here: the natural-gradient arm of optimization-explorer (step
  metric-training), with the empirical Fisher of a small softmax model
  (xetal-ML-asks GM4).

### Optimization Algorithms on Matrix Manifolds; An Introduction to Optimization on Smooth Manifolds

- P.-A. Absil, R. Mahony, R. Sepulchre, Princeton University Press,
  2008 (published Dec 2007; the free-chapter landing page misprints
  the year); <https://press.princeton.edu/absil>. Read at source
  (chapters 3 and 4 in full; the rest by contents).
- Nicolas Boumal, Cambridge University Press, 2023; DOI
  10.1017/9781009166164; free PDF at
  <https://www.nicolasboumal.net/book/>. Read at source (sections 3.6,
  3.8, 10.2; the contents).
- Leaned on for: retractions, the Riemannian gradient, the exponential
  map, the sphere, Stiefel and Grassmann. **Holds.** A retraction is a
  smooth map from the tangent bundle with R_x(0) = x and derivative
  the identity (Absil Definition 4.1.1; Boumal Definition 3.47); the
  sphere's cheap retraction is (x + v) / |x + v| (Absil Example 4.1.1);
  the Riemannian gradient is defined against the metric and, on an
  embedded submanifold, is the tangent projection of the ambient
  gradient (Boumal equation 3.40 for the sphere: (I - x x^T) times the
  flat gradient); the exponential map is the geodesic's endpoint
  (Boumal Definition 10.16). Stiefel and Grassmann: Absil 3.3.2,
  3.4.4, 4.8, 4.9; Boumal 2.4, 7.3, 9.16.
- Notes: smooth manifolds only; nothing on polytopes or stochastic
  optimization. Boumal follows Absil's conventions and cites it as
  2008.
- Here: Manifold's sphere (already: the formulas of step foundation
  cite Absil chapter 3), its step as the exponential map and the
  cheap retraction compared (step metric-descent); the Stiefel and
  Grassmann rows of the glossary (Saga 4).

### Sinkhorn 1964; Sinkhorn and Knopp 1967; Cuturi 2013

- Richard Sinkhorn, "A Relationship Between Arbitrary Positive
  Matrices and Doubly Stochastic Matrices", Annals of Mathematical
  Statistics 35(2):876-879, 1964; DOI 10.1214/aoms/1177703591.
  **Metadata only** (Project Euclid refused the fetch); its theorem is
  taken from Sinkhorn and Knopp's summary.
- Richard Sinkhorn, Paul Knopp, "Concerning nonnegative matrices and
  doubly stochastic matrices", Pacific Journal of Mathematics
  21(2):343-348, 1967; DOI 10.2140/pjm.1967.21.343;
  <https://msp.org/pjm/1967/21-2/p14.xhtml>. Read at source.
- Marco Cuturi, "Sinkhorn Distances: Lightspeed Computation of Optimal
  Transport", NIPS 2013 (the arXiv title ends "...Optimal
  Transportation Distances"); arXiv 1306.0895.
  <https://arxiv.org/abs/1306.0895>. Read at source.
- Leaned on for: alternating row and column normalization converging to
  a doubly stochastic matrix, with its conditions. **Holds.** For a
  strictly positive matrix there is a unique doubly stochastic D1 A D2
  and the alternating normalization converges to it (1964, as
  summarized in 1967). For a nonnegative matrix (1967, their theorem):
  the scaling D1 A D2 exists if and only if A has total support (every
  positive entry lies on a positive diagonal); the iteration converges
  if and only if A has support (some positive diagonal); with support
  but not total support the limit is doubly stochastic but not of the
  form D1 A D2. Cuturi: the entropic optimal-transport plan is the
  unique matrix with the prescribed marginals of the form diag(u) exp(-
  lambda M) diag(v), found by the same iteration (his Algorithm 1),
  with speedups over exact transport of orders of magnitude; the
  iteration count grows with lambda.
- Notes: the 1967 necessity proof uses Birkhoff's theorem. mHC's
  matrices are exp of logits, strictly positive, so the 1964 case
  applies and the limit exists; the only question is how many sweeps
  (the Birkhoff-geometry paper's bound).
- Here: GL3 (xetal-libraries-asks) and Mixing's private iteration
  until it lands; the row and column sum errors by sweep (step
  mhc-matrices).

### Birkhoff's theorem (the Birkhoff polytope)

- G. Birkhoff, "Tres observaciones sobre el algebra lineal", Univ.
  Nac. Tucuman Rev. Ser. A 5 (1946), 147-151 (or 147-150: sources
  differ). **Not found** online; the statement is taken at source from
  Sinkhorn and Knopp 1967, p. 344 ("the set of doubly stochastic
  matrices of order N is the convex hull of the N x N permutation
  matrices") and from the standard reference pages (the polytope's
  dimension is (n - 1)^2; equivalent results in Steinitz 1894 and
  Konig 1916). Von Neumann's 1953 share of the name was not verified
  from any page fetched; the glossary names the theorem for Birkhoff.
- Here: the words of the mhc-explorer page (a polytope, not a
  manifold) and the n = 3 picture of step mhc-matrices (nine entries,
  four free, six corners).

### A Global Geometric Framework for Nonlinear Dimensionality Reduction (Isomap)

- Joshua B. Tenenbaum, Vin de Silva, John C. Langford. Science
  290(5500):2319-2323, 22 Dec 2000; DOI 10.1126/science.290.5500.2319.
  <https://doi.org/10.1126/science.290.5500.2319> (read at source from
  an open reprint).
- Leaned on for: Isomap and the Swiss roll. **Holds.** Neighborhood
  graph (epsilon or k nearest), geodesic distances estimated by graph
  shortest paths, classical MDS on them; the Swiss roll is its Figure
  3, with the residual variance bottoming at dimension 2. The guarantee
  is asymptotic and for manifolds whose intrinsic geometry is a convex
  region of a flat space; for a hemisphere or a torus Isomap returns
  the best flat embedding, not an isometry.
- Here: the Swiss roll of manifold-tour (its words "the nearest point
  through the air is the wrong neighbor" are this paper's Figure 3);
  the experiment kit's data manifolds.

### Mobius addition (gyrovector spaces)

- Abraham A. Ungar, "Analytic Hyperbolic Geometry: Mathematical
  Foundations and Applications", World Scientific, 2005; DOI
  10.1142/5914 (**metadata only**). The formula read at source in
  Ungar, "Hyperbolic Barycentric Coordinates", Australian Journal of
  Mathematical Analysis and Applications 6(1), article 18, 2009,
  Definition 6.1, <https://ajmaa.org/searchroot/files/pdf/v6n1/v6i1p18.pdf>:
  u (+) v = ((1 + 2 u.v / s^2 + |v|^2 / s^2) u + (1 - |u|^2 / s^2) v)
  / (1 + 2 u.v / s^2 + |u|^2 |v|^2 / s^4), the unit ball at s = 1.
  **Holds.** Ganea et al. cite the 2008 book, not this one.
- Here: Manifold's `h:m_obius` (step manifold-tour); the library's
  docs cite Ungar 2005 and should also name the 2009 paper as the open
  source (done in this step).

### Testing the manifold hypothesis

- Charles Fefferman, Sanjoy Mitter, Hariharan Narayanan. Journal of
  the AMS 29(4):983-1049, 2016; arXiv 1310.0425 (2013).
  <https://arxiv.org/abs/1310.0425>. Read at source (the abstract,
  introduction and conclusion; the proofs skimmed).
- Leaned on for: the manifold hypothesis stated as an assumption.
  **Holds:** "The hypothesis that high dimensional data tend to lie in
  the vicinity of a low dimensional manifold is the basis of manifold
  learning"; the paper makes it a testable statistical hypothesis,
  with a sample complexity depending on the intrinsic dimension, the
  volume and the reach, not on the ambient dimension.
- Here: the glossary's "the manifold assumption" row and the
  manifold-tour page's wording.

## Corrections to docs/research.txt and docs/plan.md, collected

1. "Birkhoff Geometry of mHC": no "information retention" quantity;
   the paper's notion is a fading-memory horizon. The plan's row for
   constraint-lab and the research's table are read with that word.
2. "Adaptive manifold curvature regularization": no such distinct
   paper found; most likely the task-geometry paper twice. H3 does not
   rest on it.
3. "Mixed-curvature": the foundational paper is Gu, Sala, Gunel and Re
   (ICLR 2019), not Ganea.
4. "ODE-based learning on manifold": the results are for a larger
   space (a bundle, or one more dimension); flows on the same closed
   manifold are left open; no experiments.
5. "Geometric neural ODEs": a tutorial and unification with no
   experiments.
6. mHC for PEFT is a NeurIPS 2026 workshop paper (AXIOM), not a main
   conference paper; xHC is a NeurIPS 2026 poster, which the research
   does not say.
7. Nickel and Kiela report no distortion and use the additive
   retraction, not the exponential map; the research's "Riemannian SGD"
   is right in form only.
8. Dates and venues the research gives all checked out otherwise (HC
   ICLR 2025; mHC Dec 2025 and ICML 2026; KromHC ICML 2026; Birkhoff
   geometry Oct 2026; xHC and PEFT Jul 2026; Ricci flow ICML 2026;
   geometric neural ODEs 2025; ODE-based learning Jan 2026;
   task-geometry Nov 2026).

Not found, for the user: Birkhoff's 1946 original (its statement is
taken from Sinkhorn and Knopp), Sinkhorn's 1964 full text (its theorem
likewise), the full text of the task-geometry paper (paywalled;
abstract and front matter read), and the "adaptive manifold curvature
regularization" paper (no such title). Nothing in the plan rests on
any of these beyond what the open sources state.
