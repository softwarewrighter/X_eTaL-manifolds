//! The page's prose, for a reader with new eyes: one who knows neither
//! geometry nor machine learning nor X_eTaL. `[shown|id]` marks a word
//! that opens its plain explanation (docs/glossary.md, the pages'
//! glossary); `**words**` is bold. Kept apart from the page so a test
//! can check every mark names an idea the glossary has and no sentence
//! passes a verdict. No number lives here: the numbers are the run's.

pub const LEDE: &str = "Six points on a ball. Pick any two and read how far apart they are two ways: the straight line through the ball, and the great circle along its skin. Then see where the middle of the six is, by the plain average and by the average that stays on the ball. Nothing is trained.";

pub const HINT: &str = "New to this? A word with a dotted underline opens a plain explanation when you click it (or press Enter on it); click again to close it. The picture of the sphere turns when you drag it, press the arrow keys once it has the focus, or swipe; click a point in it, then another, to pick a pair. Turning changes no number.";

pub const QUESTION: [&str; 3] = [
    "If points live on a sphere, does it matter whether distance is measured straight through the sphere or along its surface? And what is the average of points on a sphere, when the ordinary average falls inside it?",
    "Machine learning meets this more than it looks. A model often keeps its representations as unit vectors, lists of numbers scaled to length 1, which is the same as putting them on a sphere: the direction carries the meaning and the length carries nothing. Two such vectors are compared by the angle between them (the cosine similarity of the literature is the cosine of that angle). The angle is a distance along the sphere; the straight line between the two points is a distance through it. Which should a method use, and does it matter?",
    "This page answers on six points you can see, with every number from the program at the bottom. It shows two facts: the two distances rank every point's neighbors the same way, and the plain average of the points is not on the sphere, where the average along it is.",
];

pub const BACKGROUND: [&str; 4] = [
    "A point here is three numbers, a place in a room. A **unit vector** is a point at distance 1 from the middle of the room: all of them together make the skin of a ball, the unit sphere, a [manifold|manifold] of two dimensions drawn in three.",
    "Two points on the sphere are apart in two ways. The **chord** is the straight line between them, which cuts through the inside of the ball: Pythagoras on their three differences. The **great circle** is the shortest path along the skin, the route an airplane flies: a [geodesic|geodesic], whose length is the angle between the two points as seen from the middle, in radians (a quarter turn is 1.57).",
    "A point's **neighbors** are the other points in order of distance from it, nearest first. Nearest-neighbor methods, clustering and retrieval all work from this order and nothing else, so two distances that give the same order are, for them, the same distance.",
    "Nothing on this page is random or trained. The six points were chosen by hand; the program measures them.",
];

pub const RATIONALE: [&str; 2] = [
    "Measure the same six points two ways and see whether the ways disagree where it matters: on which points are each other's neighbors, and on where the middle is. If the chord and the great circle rank every point's neighbors alike, a method that only ranks neighbors loses nothing by using the chord, which is cheaper. If the plain average falls off the sphere, a method that averages unit vectors must either accept a point that is not one or use the [mean that stays on the sphere|frechet-mean].",
    "The two facts are small, and they are the ground the later pages stand on: the representation pages compare the sphere's distance with the straight line on data that has directions and no lengths, and the optimization pages take steps that stay on the sphere.",
];

pub const PURPOSE: [&str; 2] = [
    "This page is an introduction: one set of six points, two distances, two averages, and a pair you pick. Every number on it, every point and arc drawn, comes from the X_eTaL program shown at the bottom, run in your browser when the page opened; picking a pair chooses which of the program's numbers you read, and runs nothing.",
    "It is not an experiment: it compares two distances, not two models, and trains nothing. The figures are marked [measured or analytical|measured-analytical]: the distances and the means are measured; the sphere's wire lines are analytical, a shape drawn from its formula to show where the sphere is.",
];

pub const INTUITION: [&str; 2] = [
    "Two cities on the globe. A string through the earth between them is shorter than the flight along the surface, by a little for near cities and by much for far ones; but the nearest city to any city is the same by string or by flight. The string and the flight are the chord and the great circle.",
    "And the average of the cities' positions, as points in space, is somewhere under the ground: it is inside the earth, not on it. The middle of points on a sphere has to be looked for on the sphere, and it is not simply the underground point pushed up to the surface.",
];

pub const MATHEMATICS: [&str; 4] = [
    "Two points p and q on the unit sphere, each three numbers with sum of squares 1. The **chord** is sqrt((p1 - q1)^2 + (p2 - q2)^2 + (p3 - q3)^2). The **great circle** is the angle between them, which the program computes as 2 atan2(|p - q|, |p + q|): the length of the difference and the length of the sum are the two legs of a right triangle whose angle is half the one wanted (this form keeps its digits for points close together, where the textbook arccos of the dot product loses them).",
    "The two are tied: chord = 2 sin(arc / 2). The program checks this on every pair. Since sin grows on the way from 0 to a quarter turn and the half-arc stays within a quarter turn, a longer arc is a longer chord every time, and that is why the two rank neighbors alike. The arc is not the shorter in any pair, and the gap between them grows with the distance.",
    "The **plain mean** is the average coordinate by coordinate, (p + q + ...) / 6. Its length is under 1 unless all six points coincide, so it is inside the sphere; how far inside is 1 minus its length. The **mean on the sphere** is the point m on the sphere whose summed squared arcs to the six points is least (the Frechet or Karcher mean). The program finds it by starting from the plain mean put onto the sphere and stepping along the average of the arrows from m to the points, thirty times; at the mean the arrows cancel.",
    "The program's words for these: s_phereDistances for the great circles, u:c_hords for the chords (written out: a straight-line distance between rows is a general function this repository has asked X_eTaL-libraries for), s_phereMean for the mean on the sphere, s_phereProject to put a point onto the sphere.",
];

pub const EXPERIMENT: [&str; 2] = [
    "What is measured: for every pair of the six points, the chord and the great circle; for every point, its neighbors in order by each; the plain mean, its length, the mean on the sphere, and the distance along the sphere between the mean on the sphere and the plain mean put onto it.",
    "Written before the run: what would count against \"the two distances agree on neighbors\" is any point whose neighbors come in a different order by the two. On six points chosen by hand with no ties, chord = 2 sin(arc / 2) says this cannot happen; the measurement confirms the formula on the numbers, not the other way round. What would count against \"the plain mean is off the sphere\" is a length of 1.",
];

pub const RUN: [&str; 1] = [
    "No model is trained. The run is the program: it puts the six directions onto the sphere, measures every pair two ways, orders every point's neighbors two ways and finds the two means. The page reads those numbers back; the pair you pick chooses which of them to show.",
];

pub const ANALYSIS: [&str; 2] = [
    "The comparison: for the pair you picked, the chord and the great circle and their ratio; over all pairs, the widest of each and the widest gap between the two; and for every one of the six points, whether its neighbors come in the same order by the two (six of six). For the means: the plain mean's length and how far it is inside the sphere, and how far the mean on the sphere is, along the sphere, from the plain mean put onto it.",
    "What the numbers say: the chord is the shorter in every pair, by a part in two hundred for the nearest pair and by a third for the widest; the neighbor orders agree for every point; the plain mean sits well inside the sphere; and the mean on the sphere is a measurable distance from the plain mean pushed out, so it is a different point, not a shortcut. Nothing here says one distance or one mean is to be preferred: that depends on the use, which the later pages test.",
];

pub const EXTENSION: [&str; 2] = [
    "What this rests on (docs/papers.md): the mean on the sphere is the Frechet mean, computed as Absil, Mahony and Sepulchre's chapter 3 does with the exponential and logarithmic maps; that the chord and the arc rank alike follows from chord = 2 sin(arc / 2) and holds for any points of the sphere, not only these six.",
    "Open, for this repository: whether using the great circle instead of the chord changes what a classifier learns on data with a directional structure (hypothesis H1): baseline-knn measures the straight line's accuracy there first (0.75, and 0.775 with the lengths dropped), and representation-explorer measures the angle's. And where points are spread over more than a hemisphere, the mean on the sphere is not unique, and this page's method of finding it is not enough.",
];

pub const SUMMARY: [&str; 3] = [
    "Two facts, on six points you can turn and pick from. The straight line through the sphere and the great circle along it give different numbers, the great circle the longer in every pair, and rank every point's neighbors the same way, because a chord is 2 sin of half its arc. The plain average of points on the sphere lies inside it, and the average along the sphere is a different point from the plain average pushed out.",
    "For machine learning: a method that only ranks neighbors of unit vectors may use either distance; a method that averages them must choose, and the two choices are different points.",
    "What this page does not show: anything about learning. Whether these facts change what a model learns is the question of the pages that train one.",
];

pub const NEXT: [&str; 2] = [
    "baseline-knn: three data sets with a known structure and nearest neighbors under the straight line, the row every later comparison is set against.",
    "representation-explorer: the sphere's distance against the straight line, on data that has directions and no lengths; a mean on the sphere in use.",
];

pub const CODE: [&str; 2] = [
    "Everything the page runs, in your browser, once: the core of sphere-hello.xtl (the six points, the two distance matrices, the neighbor orders, the two means, and for the page the arcs of every pair, the points' longitude and latitude and the sphere's wire lines), then the lines that give the page each array, raveled. The command-line program is the same core followed by its printed sections and checks. Picking a pair runs no X_eTaL.",
    "How to read it: X_eTaL evaluates right to left; a name with a colon (mf:) is a library's. The library Manifold is this repository's; Trig and Check are X_eTaL-libraries'.",
];

/// Every text of the page, for the tests.
pub fn all() -> Vec<&'static str> {
    let mut v = vec![LEDE, HINT];
    for group in [&QUESTION[..], &BACKGROUND[..], &RATIONALE[..], &PURPOSE[..], &INTUITION[..], &MATHEMATICS[..], &EXPERIMENT[..], &RUN[..], &ANALYSIS[..], &EXTENSION[..], &SUMMARY[..], &NEXT[..], &CODE[..]] {
        v.extend_from_slice(group);
    }
    v
}
