//! The page's prose, for a reader with new eyes. `[shown|id]` marks a
//! word that opens its plain explanation (docs/glossary.md); `**words**`
//! is bold. A test checks every mark names an idea and no sentence
//! passes a verdict. No number lives here: the numbers are the run's.

pub const LEDE: &str = "Before any geometry is chosen: the plainest method, nearest neighbors by the straight line through the room, on three small data sets built with a known structure (flat clusters, directions, a tree), split the same seeded way, each run's accuracy and cost recorded. The row every later comparison is set against.";

pub const HINT: &str = "New to this? A word with a dotted underline opens a plain explanation when you click it (or press Enter on it); click again to close it. The picture of the directions turns when you drag it, press the arrow keys once it has the focus, or swipe. Turning changes no number.";

pub const QUESTION: [&str; 3] = [
    "How well does the plainest method classify three data sets whose structure we know, before any geometry is chosen?",
    "Every experiment of this repository compares geometries on a learning problem: the sphere's distance against the straight line, the hyperbolic disk against the plane. A comparison needs a row to be set against, made with the same data, the same split and the same seeds, by a method with no geometry chosen at all. This page is that row.",
    "The method is k nearest neighbors, which keeps the training points and guesses a new point's label from the labels of its k nearest; the distance is the straight line through the room. It is a small machine-learning run, not a trained model: it learns no number.",
];

pub const BACKGROUND: [&str; 4] = [
    "A **data set** here is a list of points, each a list of numbers, and a label for each point (its class, 1 to k). The experiment kit makes three from explicit seeds, so that every page that asks for them gets the same numbers: **flat clusters** (points scattered about a few centers, in the plane and in five dimensions); **directions** (points that point one of four ways, near or far from the middle: the length carries nothing); and a **tree** of 31 nodes, with the distance between any two nodes counted along the tree, each node's class its branch, and a plain layout of the tree in the plane.",
    "A **split** holds out a quarter of the points, chosen by a seed, and trains on the rest; the held-out points are guessed and the share guessed right is the **accuracy**.",
    "A run's **record** is what the experiment schema says: the accuracy, how many points were held out, how many numbers were trained (here none), and the operations counted (one distance per held-out point per training point, each of d coordinates). Runtime is measured apart and is in no record.",
    "**Distortion** measures how badly a picture keeps a set of distances: over every pair of nodes, the fraction by which the picture's distance is too long or too short, averaged, and at worst (the [distortion|distortion] of the glossary, Manifold's).",
];

pub const RATIONALE: [&str; 2] = [
    "Fix everything but the geometry first: the data, the split, the method, the record. Then run the plainest geometry and write its numbers down. A later comparison that cannot beat this row on the same data has a [null result|null-result], and says so; one that beats it says by how much, against the spread over seeds.",
    "The three data sets were built to have three different structures on purpose, so that each later geometry has a set that matches it and two that do not: the sphere for the directions, the disk for the tree, the plane for the clusters. That is the first [hypothesis|hypothesis] (H1), and this page tests none of it; it measures the straight line on all three.",
];

pub const PURPOSE: [&str; 2] = [
    "This page is the baseline row of the H1 table and a tour of the three data sets. Every point drawn, every label, every held-out point and guess, and every number in the record comes from the X_eTaL program at the bottom, run in your browser when the page opened.",
    "It compares no geometries; its one comparison is on the directional set, between the points as they are and the same points with their lengths dropped (put onto the unit sphere), both still under the straight line, which measures what the lengths cost. The figures are marked [measured or analytical|measured-analytical]: all of them here are measured.",
];

pub const INTUITION: [&str; 1] = [
    "Three kinds of structure a data set can have. Blobs: points gathered about a few centers, where the nearest points to a point are its own kind, and the straight line is a fine ruler. Directions: points that point the same way but sit near or far from the middle, where the straight line mixes the pointing with the distance from the middle and is a muddled ruler. A family tree: cousins are near in the tree and far on any flat page the tree is drawn on, and a ruler on the page is a poor copy of the tree's.",
];

pub const MATHEMATICS: [&str; 3] = [
    "**k nearest neighbors.** For a held-out point x and the training points y, the straight-line distance is sqrt(sum over coordinates of (x_i - y_i)^2). The k training points with the smallest distance are found (k = 5, or 3 on the tree's 31 nodes); the label most of them have is the guess; on a tie, the smallest label. Accuracy is the number guessed right divided by the number held out.",
    "**The split.** n seeded uniform numbers are drawn and their order taken: a seeded shuffle of the rows. The first quarter is held out, the rest trained on. The seed is 11 for every set, so the split depends on the set's size and the seed alone.",
    "**Distortion.** For a layout with straight-line distances E and a tree with distances T along it, over the pairs of different nodes: |E / T - 1|, averaged, and its largest value. 0 when every distance is kept; 1 for a pair twice or half as far as it should be.",
];

pub const EXPERIMENT: [&str; 2] = [
    "Five runs: the flat clusters in the plane (3 clusters of 40, spread 1.6, seed 1) and in five dimensions (4 of 30, spread 1.6, seed 2); the directions (4 classes of 40, cone 0.6, lengths from 0.5 to 2, seed 5) as they are and with their lengths dropped; the tree's layout (depth 4, branching 2, k = 3). Each split by seed 11, each recorded. And the distortion of the tree's layout against the tree.",
    "Written before the run: the clusters touch (spread 1.6 against centers a few units apart), so an accuracy under 1 was expected on them; the directions overlap at their edges (cone 0.6), so a lower one; the tree's layout puts each branch in its own half of the plane, so a high one, which says nothing about distances; and the layout's distortion was expected to be large at its worst pair (two leaves of neighboring branches, near on the page and far in the tree).",
];

pub const RUN: [&str; 1] = [
    "The run is the program: nearest neighbors on each held-out quarter. Nothing is trained; nearest neighbors keeps the training points and learns no number, which the record says (numbers trained: 0 in every row). The pictures below mark the held-out points and the guesses that went wrong.",
];

pub const ANALYSIS: [&str; 2] = [
    "The record, one row per run, every cell measured: the accuracy, the held-out count, the numbers trained and the operations. The sentences under it are the shell's: what moved between two runs, with no verdict. The one comparison is the directions with their lengths kept against dropped.",
    "What the numbers say: the straight line gets nine in ten of the flat clusters and nearly all of them in five dimensions; three in four of the directions, and a little more with the lengths dropped, so the lengths, which carry nothing, cost a little; every node of the tree's layout, because its halves give the branch away, while the layout keeps the tree's distances to within a fifth on average and four fifths at worst. Nothing here counts for or against H1: no geometry has been compared yet.",
];

pub const EXTENSION: [&str; 2] = [
    "The directional set's gain from dropping the lengths is not yet the sphere's own distance (the angle between points); representation-explorer measures the angle, the first test of H1. The tree's layout is a picture, not an embedding fitted to its distances; fitting one in the plane, on the sphere and in the Poincare disk at the same dimension and measuring each distortion is what the hyperbolic page does, after Nickel and Kiela (2017) and Gu, Sala, Gunel and Re (2019), read at source (docs/papers.md).",
    "What this page does not establish: that any of these structures is in a real data set. The three were built to have them; a real data set's structure is measured, not assumed.",
];

pub const SUMMARY: [&str; 2] = [
    "Nearest neighbors by the straight line, on three data sets with a known structure, the same split and seeds: 0.90 and 0.97 on the flat clusters, 0.75 on the directions and 0.775 with their lengths dropped, 1.0 on the tree's layout whose distortion against the tree is 0.18 on average and 0.80 at worst. These are the first row of the H1 table (docs/experiments.md).",
    "The kit behind them, the Kit library and the written-out split, record and comparison, is what every later page holds equal, so that its comparison is between geometries and not between data handling.",
];

pub const NEXT: [&str; 2] = [
    "representation-explorer, part 1: the sphere's distance against the straight line on the directional and the flat sets, with these rows as the baseline.",
    "representation-explorer, part 2: the plane, the sphere and the Poincare disk fitted to the tree at the same dimension; the distortion of each against the layout's 0.18 and 0.80.",
];

pub const CODE: [&str; 2] = [
    "Everything the page runs, in your browser, once: the core of baseline-knn.xtl (the kit written out, the three data sets from the Kit library, the five runs and their records, the distortion), then the lines that give the page each array, raveled. The command-line program is the same core followed by its printed sections and checks.",
    "How to read it: X_eTaL evaluates right to left; a name with a colon is a library's: kt: the Kit library and mf: Manifold, both this repository's; ml: Learn, X_eTaL-ML's; ch: Check, X_eTaL-libraries'. The functions written out at the top (the split, the run, the comparison, the pairwise distances) are the workarounds for asks GM5 and GL1.",
];

pub fn all() -> Vec<&'static str> {
    let mut v = vec![LEDE, HINT];
    for group in [&QUESTION[..], &BACKGROUND[..], &RATIONALE[..], &PURPOSE[..], &INTUITION[..], &MATHEMATICS[..], &EXPERIMENT[..], &RUN[..], &ANALYSIS[..], &EXTENSION[..], &SUMMARY[..], &NEXT[..], &CODE[..]] {
        v.extend_from_slice(group);
    }
    v
}
