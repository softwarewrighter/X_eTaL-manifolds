# Dynamics explorer: must a state head downhill?

Part 1: five vector fields in the plane, v(x) = A x for a 2 by 2
matrix A (attractor, rotation, spiral, saddle, expanding spiral). Each
is drawn as arrows, with the paths from four starts beside the exact
ones. The page measures each field's divergence and curl and the sum
of v . dx round a loop, and reads the rest point's stability from the
trace and determinant of A. Then integration as a choice: Euler's step
against a fourth-order Runge-Kutta step on the rotation, whose exact
paths are circles. Then the comparison as machine learning meets it:
gradient descent on a loss beside two players on the objective
f(x, y) = x y, with two known repairs. Live at
[softwarewrighter.github.io/X_eTaL-manifolds/dynamics-explorer](https://softwarewrighter.github.io/X_eTaL-manifolds/dynamics-explorer/).
Part 2 (step manifold-flows, to come) learns a field from data, in
flat space and held to a sphere.

## The question

A model's state moves a little at every step of training. What
decides where it goes, and must it be heading downhill? Gradient
descent says: downhill, on the loss; its steps follow a gradient
field. But when two parts of a model pull against each other (a
generator and a discriminator in a GAN are the known case), each
follows its own slope, and together they follow a field that need not
come from any one function.

## Background

- A **vector field** gives an arrow at every point, dx/dt = v(x); a
  state that follows it traces a **path**. Here x stands for the whole
  state, both of its numbers.
- The fields are linear, v(x) = A x with A = (a b / c d) by rows. Two
  numbers of A recur: the **trace** a + d and the **determinant**
  a d - b c. The arrow is zero at the **rest point** x = 0.
- An **integrator** follows a field in steps of size s: Euler's step
  x + s v(x), or a fourth-order Runge-Kutta step that looks at the
  field four times and averages.
- **Divergence** (do the arrows spread out or gather in) and **curl**
  (do they turn), found by central differences with a small step dh.
- A **gradient field** is minus the gradient of one function L (a
  loss): the arrows point the way L falls fastest. Round any closed
  loop the sum of v . dx is then zero; for a field that turns it is
  the curl times the area enclosed (Green's theorem).
- The **game**: player one picks x to make f = x y small, player two
  picks y to make it large; each steps along its own slope at rate
  eta. The solution is (0, 0), where neither can gain by moving alone.

## The idea

Keep the rule (follow the arrows) and change the field: the paths
fall in, circle, spiral in, come in along one line and leave along
another, or spiral out, and the trace and determinant of A predict
which before the paths are looked at. Keep the field and change how
it is followed: on the rotation the distance from the circle is the
integrator's error, readable at every step. The game's two slopes
together are the rotation, so stepping both players at once is
Euler's step on a rotation; if that is the mechanism, a repair should
change how the field is stepped, not the field.

## What this is, and is not

An experiment on five formulas and one tiny game, not on a trained
model: every field is linear, so every path, divergence, curl and loop
sum has an exact value beside the measurement. These are
illustrations in the plane and one small game, not a field learned
from data. It does not show how often the training of a real GAN
fails: the game has one number per player and a known answer.

## The intuition

A leaf on a pond: it drains into a plughole (attractor), circles in a
swirl (rotation), spirals into a draining swirl, or comes in along one
current and leaves along another (saddle). A ball rolling on a hill
cannot go round a loop and come back to where it started, since it
would have to climb back up: a gradient field has a height, and along
its paths the height only falls. Two people on one dial, one wanting
it low and one high, each answering the other's last move, go round
the setting where neither could gain alone; if both move at once with
moves a little too large, each round goes wider.

## The mathematics

- The exact path is x(t) = exp(t A) x0, written out for 2 by 2: with
  tau half the trace and B = A - tau I (so B B = -det(B) I),
  exp(t A) = e^(tau t) (cos(w t) I + sin(w t) / w B) with
  w = sqrt(det B) when det B > 0; cosh and sinh when det B < 0;
  e^(tau t) (I + t B) when det B = 0.
- On the rotation an Euler step multiplies the distance from the rest
  point by exactly sqrt(1 + s^2); a Runge-Kutta step by the size of
  1 + z + z^2 / 2 + z^3 / 6 + z^4 / 24 at z = i s, about
  1 - s^6 / 144.
- For v = A x the divergence is the trace a + d and the curl c - b.
  For a symmetric A (b = c), v is minus the gradient of
  L(x) = -(a x^2 + 2 b x y + d y^2) / 2, and its curl is 0.
- The eigenvalues of A are T / 2 +- sqrt(T^2 / 4 - D) (T the trace,
  D the determinant), a complex pair tau +- i w when T^2 / 4 < D;
  every path falls in if both real parts are negative, some path
  leaves if one is positive.
- The game's rules as matrices, p <- M p: simultaneous
  (1 -eta / eta 1), factor sqrt(1 + eta^2); taking turns
  (1 -eta / eta 1 - eta^2), determinant 1, eigenvalues of size 1 for
  eta < 2, keeping x^2 + y^2 - eta x y fixed (Bailey, Gidel and
  Piliouras 2019); the look-ahead step, extragradient
  (1 - eta^2 -eta / eta 1 - eta^2), factor sqrt(1 - eta^2 + eta^4)
  (Korpelevich 1976, analyzed on this game by Gidel et al. 2019);
  descent on L = (x^2 + y^2) / 2, (1 - eta) I.

## What is compared

- A gradient field against a general vector field that turns: the
  attractor and the saddle (symmetric A) against the rotation and the
  two spirals, by the curl and the sum round a loop.
- Euler's step against Runge-Kutta's on the rotation, at step sizes
  0.05, 0.1, 0.2 and 0.4, and with the same work (the same looks at
  the field over the same time).
- Gradient descent on a loss against simultaneous descent and ascent
  on the game, from the same start (1, 1) at the same rate 0.2; on the
  game, simultaneous steps, taking turns and the look-ahead step.

What was expected, written into the program's checks before it first
ran: a loop sum of 0 for the symmetric fields and not for the turning
ones; the sign of the largest real part decides whether paths fall in
or some leave; Euler's distance from the circle grows by exactly its
factor; descent settles, simultaneous steps go out, taking turns stays
within 2 of the solution keeping its quantity fixed, the look-ahead
step comes in.

## The run

No model is trained. Each field's paths are 40 Runge-Kutta steps of
0.1 from four starts; the rotation is stepped 200 times by each
integrator at each step size; the game 100 steps of each rule. The
preset is `presets/fields.txt` (data, read by keeping its digits and
dropping its words).

## The program

[`dynamics-explorer.xtl`](dynamics-explorer.xtl): the preset; the
fields and the grid of arrows (`u:f_ield`); the two integrators
(`u:e_uler`, `u:r_k4`, `u:p_ath`); the exact paths (`u:f_low`, the
closed form of exp(t A)); divergence and curl by central differences
(`u:m_easures`); the sum round a loop and the area (`u:l_oop`,
`u:a_rea`), the square worked side by side, the gradient check of the
symmetric fields; the eigenvalues from trace and determinant
(`u:e_igen`) and each path's end over start; Euler against
Runge-Kutta on the rotation, the factors, the same work; descent and
the game (`u:d_escent`, `u:s_imultaneous`, `u:t_urns`, `u:l_ookAhead`),
the rules as matrices, the quantity taking turns keeps. Then 16
checks.

## How it works

Every path is one `p_ower` loop over (point, rows so far); the
integrator is chosen by a flag in `u:o_ne`. Every field is a 2 by 2
matrix, so the field at many points is one product, `P mx:m_ul o_\ A`.
The page reads back the printed arrays and draws them; it computes no
number.

## Results

From the baseline (`reg/cli-dynamics-explorer.out`).

- **Measures.** Divergence and curl by differences agree with the
  trace and c - b to 2e-12 at every grid point. Round the 64-piece
  loop on the unit circle (area 3.1365): the attractor and saddle add
  -7.5e-17 and -1.9e-16; the rotation and both spirals add 6.2731, the
  curl 2 times the area. Round the square, each of the rotation's
  four sides adds 2 (the "height" ends at -8); each of the attractor's
  adds 0.
- **Stability.** Attractor: real parts -0.5 and -1, every path ends
  nearer (0.018 to 0.135 of its start). Rotation: real parts 0,
  every path ends as far out as it started (to 3e-7). Spiral: -0.2,
  0.449. Saddle: 1 and -1, one path comes in (0.018), the others leave
  (to 54.6). Expanding spiral: 0.2, 2.226. Runge-Kutta's paths are
  within 4e-6 (relative) of the exact ones for every field.
- **Integrators on the rotation, 200 steps.** Euler ends 0.284,
  1.70, 49.5 and 2.79e6 outside the circle at steps 0.05, 0.1, 0.2,
  0.4, exactly as sqrt(1 + s^2) to the 200th power says; Runge-Kutta
  ends 2.2e-8, 1.4e-6, 8.8e-5 and 0.0056 inside. With the same 200
  looks at the field over time 20: Euler at 0.1 ends 1.70 outside,
  Runge-Kutta at 0.4 ends 0.0014 inside.
- **The game, from (1, 1) at rate 0.2, 100 steps.** Descent on the
  loss: factor 0.8 per step, 2.9e-10 from the solution at the end.
  Simultaneous steps: factor 1.0198, 10.05 at the end. Taking turns:
  between 1.28 and 1.41, 1.30 at the end, x^2 + y^2 - 0.2 x y fixed at
  1.8. The look-ahead step: factor 0.9806, 0.200 at the end. The
  game's field is the rotation, to 3e-12.

Reading: the curl and the loop sum told the fields from a loss from
those that turn, as the mathematics says they must (for linear fields
a check of the arithmetic, not a discovery). How the field is stepped
changed the path too: Euler's steps leave the rotation's circles at
every step size. The game behaved as the rotation stepped by Euler's
rule, and the two repairs did what the papers prove on this case: the
look-ahead step converges, taking turns stays bounded without
converging. The limits: linear fields only (one rest point, no closed
loops that attract), and a bilinear game with one number per player.

## Beyond this page

Fields that are not linear (several rest points, closed loops that
attract), fields learned from data, fields on a curved space: part 2.
Averaging the iterates of taking turns also converges on this game
(Gidel et al. 2019), not measured here. Optimization explorer's paths
were flows of a gradient field under a metric: a ruler P turns -g
into -P g, which in general is not a gradient field, but the loss
still falls along it.

## Run it

```bash
just run dynamics-explorer         # the sections and 16 checks
just show dynamics-explorer        # as a notebook
just test-demo dynamics-explorer   # CLI baseline, web tests, wasm build, browser check
```

The page (`web/`): the five fields with their arrows and paths, the
field picked by buttons with the change narrated from measured values;
the measures and rest points of all five; the square worked in full;
the rotation with the step size and the steps shown picked by
buttons, the distance from the circle by step, the factors; the game's
four paths, the distance by step, the matrices; the comparison tables;
the glossary; the program and the preset. Every number is one the
program printed; the buttons run nothing.

## Summary

The field decides where a state goes, and it need not be downhill on
anything: two of the five fields are minus the gradient of a loss
(no curl, a loop sum of zero) and three turn (a loop sum equal to the
curl times the area). The trace and determinant of A predicted, before
the paths were looked at, which fields draw paths in and which let
some leave, and the paths agreed. Following a field is a choice of its
own: Euler's step spirals off the rotation's circles at every step
size by a factor known in advance; Runge-Kutta stays near them, also
with the same work. Gradient descent on a loss settles; two players on
f = x y, stepping at once, follow Euler's rule on a rotation and
spiral out; taking turns keeps them circling in a band, and the
look-ahead step brings them in at two looks per step.

## Next

Part 2 of this page (step manifold-flows): a small field learned from
data, carrying points in flat space and on the sphere, held to it by a
step that stays on it; what a flow that can be run backward cannot do.

## Workarounds

- The integrators (`u:e_uler`, `u:r_k4`, `u:p_ath`) are
  X_eTaL-libraries' by ask GL4 (`docs/xetal-libraries-asks.md`),
  written out in the program.
- The divergence and curl by central differences (`u:m_easures`) are
  X_eTaL-libraries' by GL5, written out.
- The arrows, paths and charts are drawn by the page from the numbers
  the program prints; field and path pictures are GL6's.
- The preset reader is GL9's: the preset's digits are kept and its
  words dropped, so no word in `presets/fields.txt` may hold a digit.
