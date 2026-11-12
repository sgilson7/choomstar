//! Choomstar's long-jump courses: maps of their own, built as Chumontillado
//! builds its maps (`maps.rs`: boxes, the Tobogán's sheared surf troughs,
//! the Selva's vines), set in open space after Ape Race's Sector Ape: decks
//! and debris adrift over a far sun, wreck beams with vines hanging from
//! them, and at the end a lip that throws you out over a landing field far
//! below (the person, 2026-11-12: "you are scored for how long you can go in
//! the course, and how long of a jump you can hit at the end").
//!
//! A course runs along −z from its deck. Its length is how far along the
//! lip stands; the landing field lies past the lip, `FIELD_DROP` m under it.

use crate::fx::{int, Fx, ONE};
use crate::geom::{v3, yaw_of, V3};
use crate::map::{block, sheared, Map, Surface, Vine};

/// The courses by the names missions use.
pub const COURSES: [&str; 4] = ["rail", "debris", "wreck", "sector"];

/// The deck stands this high (m): the sun is far below.
const DECK_Y: i32 = 200;
/// The landing field lies this far under the lip (m), runs this far past
/// it, and is this wide.
const FIELD_DROP: i32 = 40;
const FIELD_LONG: i32 = 400;
const FIELD_WIDE: i32 = 140;
/// A trough's faces: out 10 m and up 12 m each side of its crease, as the
/// Tobogán's.
const TROUGH_OUT: i32 = 10;
const TROUGH_UP: i32 = 12;
/// A vine's feet hang this far over the floor it is reached from (m).
const VINE_FEET: i32 = 3;

/// A course and the numbers its mission reads.
pub struct Course {
    pub map: Map,
    /// where the run starts along z, and the lip it leaves from
    pub start_z: Fx,
    pub lip: V3,
    /// the landing field's height
    pub field_y: Fx,
    /// metres from the start to the lip
    pub length: i32,
}

/// Lays a course down piece by piece, along −z.
struct Lay {
    bs: Vec<crate::map::Brush>,
    vines: Vec<Vine>,
    /// where the next piece starts: along z, and its floor's height (m)
    z: i32,
    y: i32,
}

impl Lay {
    fn new() -> Lay {
        Lay {
            bs: Vec::new(),
            vines: Vec::new(),
            z: 0,
            y: DECK_Y,
        }
    }

    /// A flat deck `long` m, `wide` m across.
    fn deck(&mut self, long: i32, wide: i32) -> &mut Lay {
        let h = wide / 2;
        self.bs.push(block(
            v3(int(-h), int(self.y - 1), int(self.z - long)),
            v3(int(h), int(self.y), int(self.z)),
            Surface::Floor,
        ));
        // a rail of trim along each edge, as a deck's bulwark
        for x in [-h, h - 1] {
            self.bs.push(block(
                v3(int(x), int(self.y), int(self.z - long)),
                v3(int(x + 1), int(self.y) + ONE / 2, int(self.z)),
                Surface::Trim,
            ));
        }
        self.z -= long;
        self
    }

    /// A surf trough `long` m that falls `drop` m (rises, if negative): two
    /// faces too steep to stand on, meeting in a crease at the floor's line.
    fn trough(&mut self, long: i32, drop: i32) -> &mut Lay {
        let (za, zb) = (int(self.z), int(self.z - long));
        let (ya, yb) = (int(self.y), int(self.y - drop));
        for side in [-1, 1] {
            let (out, up) = (int(TROUGH_OUT) * side, int(TROUGH_UP));
            let corners = if side < 0 {
                [(0, 0), (out, up), (out, 0)]
            } else {
                [(0, 0), (out, 0), (out, up)]
            };
            self.bs.push(sheared(corners, za, zb, ya, yb, Surface::Ramp));
        }
        self.z -= long;
        self.y -= drop;
        self
    }

    /// Open space `long` m, the next piece `rise` m higher (lower, if
    /// negative).
    fn gap(&mut self, long: i32, rise: i32) -> &mut Lay {
        self.z -= long;
        self.y += rise;
        self
    }

    /// `n` pieces of debris adrift, each `size` m square, `gap` m apart, each
    /// `step` m lower than the last: a line to hop.
    fn debris(&mut self, n: i32, size: i32, gap: i32, step: i32) -> &mut Lay {
        for k in 0..n {
            if k > 0 {
                self.gap(gap, -step);
            }
            // a little to one side and the other, so the line weaves
            let off = if k % 2 == 0 { -2 } else { 2 };
            let h = size / 2;
            self.bs.push(block(
                v3(int(off - h), int(self.y - 2), int(self.z - size)),
                v3(int(off + h), int(self.y), int(self.z)),
                Surface::Floor,
            ));
            self.z -= size;
        }
        self
    }

    /// A gap `long` m with a wreck's beam over it and `n` vines hanging from
    /// it, their feet `VINE_FEET` m over the floor before it.
    fn vines(&mut self, long: i32, n: i32) -> &mut Lay {
        let top = self.y + VINE_FEET + 11;
        self.bs.push(block(
            v3(int(-3), int(top), int(self.z - long)),
            v3(int(3), int(top + 2), int(self.z)),
            Surface::Block,
        ));
        for k in 0..n {
            let z = self.z - long * (k + 1) / (n + 1);
            self.vines.push(Vine {
                top: v3(0, int(top), int(z)),
                length: int(11),
            });
        }
        self.z -= long;
        self
    }

    /// The landing field past the lip, and the course.
    fn finish(mut self, name: &'static str) -> Course {
        let lip = v3(0, int(self.y), int(self.z));
        let field_y = self.y - FIELD_DROP;
        let h = FIELD_WIDE / 2;
        self.bs.push(block(
            v3(int(-h), int(field_y - 1), int(self.z - FIELD_LONG)),
            v3(int(h), int(field_y), int(self.z)),
            Surface::Floor,
        ));
        let start = v3(0, int(DECK_Y), -int(2));
        Course {
            map: Map {
                brushes: self.bs,
                spawns: vec![(start, yaw_of(v3(0, 0, -ONE)))],
                name,
                pickups: Vec::new(),
                bot_loop: Vec::new(),
                vines: self.vines,
                cube: None,
            },
            start_z: start.z,
            lip,
            field_y: int(field_y),
            length: -self.z,
        }
    }
}

/// Whether a mission's map is a course.
pub fn is_course(name: &str) -> bool {
    COURSES.contains(&name)
}

/// The course of that name.
pub fn course(name: &str) -> Course {
    let mut l = Lay::new();
    match name {
        // Launch Rail: off the deck into a long trough, down 45 m, and up a
        // lip: learn to hold a surf and leave it fast
        "rail" => {
            l.deck(16, 16).trough(130, 45).trough(30, -12);
            l.finish("course_rail")
        }
        // Debris Field: hop a weaving line of debris down, surf a trough,
        // hop again, then the lip
        "debris" => {
            l.deck(12, 14).debris(7, 7, 5, 1).gap(4, -2).trough(60, 20);
            l.gap(3, -2).debris(3, 8, 7, 1).gap(3, 0).trough(25, -10);
            l.finish("course_debris")
        }
        // Wreck Vines: down a trough to a wreck, then two gaps too long to
        // jump, crossed on vines from the wreck's beams, then the lip
        "wreck" => {
            l.deck(12, 14).trough(50, 20).gap(2, -1).deck(10, 12);
            l.vines(18, 3).deck(10, 12).vines(24, 4).deck(10, 12);
            l.trough(40, 15).trough(25, -10);
            l.finish("course_wreck")
        }
        // Sector Ape: all of it, long: trough, debris, vines, a long trough
        // and the biggest lip
        _ => {
            l.deck(12, 14).trough(70, 30).gap(3, -2).debris(5, 7, 6, 1);
            l.gap(3, -1).deck(8, 12).vines(20, 3).deck(8, 12);
            l.trough(110, 40).gap(3, -2).debris(3, 8, 7, 0).gap(3, 0);
            l.vines(22, 3).deck(8, 12).trough(60, 25).trough(35, -14);
            l.finish("course_sector")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_course_starts_on_its_deck_and_ends_on_a_lip_over_its_field() {
        for name in COURSES {
            let c = course(name);
            assert!(c.length > 100, "{name} is {} m", c.length);
            assert_eq!(c.lip.z, -int(c.length));
            assert_eq!(c.field_y, c.lip.y - int(FIELD_DROP));
            assert_eq!(
                c.map.lowest(),
                c.field_y - ONE,
                "{name}: the field is the floor of the world"
            );
            // the deck is under the start
            let s = c.map.spawns[0].0;
            assert!(c.map.brushes.iter().any(|b| b.surface == Surface::Floor
                && b.mins.z <= s.z
                && b.maxs.z >= s.z
                && b.maxs.y == s.y));
        }
    }

    #[test]
    fn the_wreck_hangs_seven_vines_over_its_two_gaps() {
        assert_eq!(course("wreck").map.vines.len(), 7);
        assert_eq!(course("rail").map.vines.len(), 0);
    }
}
