//! Turning a picture in three dimensions: the page's projection of the
//! coordinates X_eTaL printed onto the screen, by a yaw (round the up
//! axis) and a pitch (tipping toward the viewer), orthographic (no
//! perspective: a turned picture is the same picture seen from
//! elsewhere, nothing nearer drawn bigger). Presentation only (plan
//! G13, CLAUDE.md rule 3a): it moves points, segments and wire lines
//! X_eTaL computed and computes no coordinate of its own. This is the
//! demo-private workaround for ask GM3 (docs/xetal-ML-asks.md), the
//! shared turning view of X_eTaL-ML: when that lands at the pin this
//! module goes.

use crate::micro::P3;

/// How a picture is turned: yaw round the up axis and pitch toward
/// the viewer, in radians. Page state, not a result.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Turn {
    pub yaw: f64,
    pub pitch: f64,
}

impl Turn {
    /// A pitch is kept within a quarter turn either way, so up stays up.
    pub fn tipped(self, dyaw: f64, dpitch: f64) -> Turn {
        Turn { yaw: self.yaw + dyaw, pitch: (self.pitch + dpitch).clamp(-1.55, 1.55) }
    }
    /// In whole degrees, for the reader: yaw brought round to 0 to 359.
    pub fn degrees(self) -> (i32, i32) {
        let yaw = (self.yaw.to_degrees().round() as i32).rem_euclid(360);
        (yaw, self.pitch.to_degrees().round() as i32)
    }
}

/// Which of X_eTaL's three coordinates the page draws as up: the
/// sphere and the torus are written with z up, the Swiss roll with
/// its height as y (as the data set is always written).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Up {
    Z,
    Y,
}

/// A point on the screen: x to the right, y down (SVG's way), in the
/// units of the picture, and its depth toward the viewer (larger is
/// nearer), for drawing what is in front last and lighter what is
/// behind.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dot {
    pub x: f64,
    pub y: f64,
    pub depth: f64,
}

/// A picture is SIZE by SIZE.
pub const SIZE: f64 = 420.0;

/// The projection: a point turned, scaled so that `radius` reaches
/// the picture's edge, and put on the screen.
pub fn project(p: P3, up: Up, turn: Turn, radius: f64) -> Dot {
    // Up as z, by a quarter turn about x when the data has it as y
    // (a rotation, so the picture keeps its handedness).
    let (x, y, z) = match up {
        Up::Z => (p[0], p[1], p[2]),
        Up::Y => (p[0], -p[2], p[1]),
    };
    // Yaw round the up axis.
    let (cy, sy) = (turn.yaw.cos(), turn.yaw.sin());
    let (x1, y1) = (x * cy - y * sy, x * sy + y * cy);
    // The viewer looks along y1; pitch tips the picture about the screen's horizontal.
    let (cp, sp) = (turn.pitch.cos(), turn.pitch.sin());
    let (up2, depth) = (z * cp + y1 * sp, z * sp - y1 * cp);
    let s = 0.43 * SIZE / radius;
    Dot { x: SIZE / 2.0 + s * x1, y: SIZE / 2.0 - s * up2, depth: depth / radius }
}

/// The farthest any point of the lines is from the origin: what a
/// picture's scale is set by (a size, not a result).
pub fn radius(lines: &[Vec<P3>]) -> f64 {
    lines.iter().flatten().map(|p| (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt()).fold(0.0, f64::max).max(1e-9)
}

/// A segment of a wire line on the screen, with how near it is (0 at
/// the back, 1 at the front).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Segment {
    pub a: Dot,
    pub b: Dot,
    pub near: f64,
}

/// Every segment of the wire lines, farthest first.
pub fn segments(lines: &[Vec<P3>], up: Up, turn: Turn, radius: f64) -> Vec<Segment> {
    let mut v: Vec<Segment> = lines
        .iter()
        .flat_map(|line| {
            line.windows(2).map(move |w| {
                let (a, b) = (project(w[0], up, turn, radius), project(w[1], up, turn, radius));
                Segment { a, b, near: ((a.depth + b.depth) / 2.0 + 1.0) / 2.0 }
            })
        })
        .collect();
    v.sort_by(|p, q| p.near.total_cmp(&q.near));
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    const FLAT: Turn = Turn { yaw: 0.0, pitch: 0.0 };

    #[test]
    fn unturned_x_goes_right_and_z_goes_up() {
        let o = project([0.0, 0.0, 0.0], Up::Z, FLAT, 1.0);
        let x = project([1.0, 0.0, 0.0], Up::Z, FLAT, 1.0);
        let z = project([0.0, 0.0, 1.0], Up::Z, FLAT, 1.0);
        let y = project([0.0, 1.0, 0.0], Up::Z, FLAT, 1.0);
        assert!((o.x - SIZE / 2.0).abs() < 1e-9 && (o.y - SIZE / 2.0).abs() < 1e-9);
        assert!(x.x > o.x && (x.y - o.y).abs() < 1e-9);
        assert!(z.y < o.y && (z.x - o.x).abs() < 1e-9);
        // y points away from the viewer: same place as the origin, farther.
        assert!((y.x - o.x).abs() < 1e-9 && (y.y - o.y).abs() < 1e-9 && y.depth < o.depth);
    }

    #[test]
    fn a_quarter_turn_of_yaw_brings_y_to_the_front() {
        let t = Turn { yaw: std::f64::consts::FRAC_PI_2, pitch: 0.0 };
        let y = project([0.0, 1.0, 0.0], Up::Z, t, 1.0);
        let x = project([1.0, 0.0, 0.0], Up::Z, t, 1.0);
        // y is now where -x was: to the left; x went to the back.
        assert!(y.x < SIZE / 2.0 && y.depth.abs() < 1e-9);
        assert!((x.x - SIZE / 2.0).abs() < 1e-9 && x.depth < -0.99);
    }

    #[test]
    fn pitch_tips_the_top_toward_the_viewer() {
        let t = Turn { yaw: 0.0, pitch: 0.5 };
        let z = project([0.0, 0.0, 1.0], Up::Z, t, 1.0);
        assert!(z.depth > 0.0 && z.y < SIZE / 2.0);
    }

    #[test]
    fn the_rolls_height_is_drawn_up() {
        let h = project([0.0, 1.0, 0.0], Up::Y, FLAT, 1.0);
        assert!(h.y < SIZE / 2.0 && (h.x - SIZE / 2.0).abs() < 1e-9);
    }

    #[test]
    fn the_radius_reaches_the_edge_and_the_pitch_is_held() {
        let far = project([2.0, 0.0, 0.0], Up::Z, FLAT, 2.0);
        assert!((far.x - (SIZE / 2.0 + 0.43 * SIZE)).abs() < 1e-9);
        assert_eq!(FLAT.tipped(0.0, 9.0).pitch, 1.55);
        assert_eq!(Turn { yaw: -0.1, pitch: 0.0 }.degrees(), (354, 0));
        assert!((radius(&[vec![[3.0, 4.0, 0.0], [0.0, 0.0, 1.0]]]) - 5.0).abs() < 1e-12);
    }

    #[test]
    fn segments_are_sorted_back_to_front() {
        let line = vec![[0.0, 1.0, 0.0], [1.0, 0.0, 0.0], [0.0, -1.0, 0.0]];
        let s = segments(&[line], Up::Z, FLAT, 1.0);
        assert_eq!(s.len(), 2);
        assert!(s[0].near <= s[1].near);
        assert!(s[1].near > 0.5 && s[0].near < 0.5);
    }
}
