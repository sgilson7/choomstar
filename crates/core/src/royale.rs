//! Choomstar's royale maps: wide floors for a hundred fighters at once,
//! every one against every other, until one stands (the person,
//! 2026-11-12: "battle royale maps where there are 100 characters fighting
//! free for all and you need to be the last one standing"). Built as
//! Chumontillado builds its maps (`maps.rs`: a walled yard, boxes, turned
//! shields), with weapons lying about to be taken and a spawn for each of
//! the hundred and one.

use crate::fx::{self, int, Fx, ONE};
use crate::geom::{v3, yaw_of, V3};
use crate::map::{block, turned, Brush, Map, Surface};
use crate::rng::Rng;
use crate::weapons::Weapon;

/// The royale maps by the names missions use.
pub const ROYALES: [&str; 3] = ["scrapyard", "spires", "hub"];

/// Seats on a royale map: you and a hundred.
pub const FIGHTERS: usize = 101;

/// The floor runs this far each way from the middle (m), walled this high.
const HALF: i32 = 52;
const WALL_HIGH: i32 = 6;
/// Spawns are tried on a grid this far apart (m), each moved up to
/// `JITTER` m, and kept where a body can stand.
const SPAWN_GRID: i32 = 8;
const JITTER: i32 = 2;
/// Weapons lie on a grid this far apart (m).
const LOOT_GRID: i32 = 13;
/// What lies about: everything but the pistol and navaja every fighter
/// starts with.
const LOOT: [Weapon; 9] = [
    Weapon::Shotgun,
    Weapon::MachinePistol,
    Weapon::Rifle,
    Weapon::Ak,
    Weapon::Drum,
    Weapon::Trabuco,
    Weapon::Katana,
    Weapon::Guadana,
    Weapon::Toro,
];

/// Whether a mission's map is a royale map.
pub fn is_royale(name: &str) -> bool {
    ROYALES.contains(&name)
}

fn b(x0: i32, y0: i32, z0: i32, x1: i32, y1: i32, z1: i32, s: Surface) -> Brush {
    block(v3(int(x0), int(y0), int(z0)), v3(int(x1), int(y1), int(z1)), s)
}

/// The floor and the wall round it.
fn walled() -> Vec<Brush> {
    let h = HALF;
    vec![
        b(-h - 1, -1, -h - 1, h + 1, 0, h + 1, Surface::Floor),
        b(-h - 1, 0, -h - 1, -h, WALL_HIGH, h + 1, Surface::Wall),
        b(h, 0, -h - 1, h + 1, WALL_HIGH, h + 1, Surface::Wall),
        b(-h, 0, -h - 1, h, WALL_HIGH, -h, Surface::Wall),
        b(-h, 0, h, h, WALL_HIGH, h + 1, Surface::Wall),
    ]
}

/// A box `long` m by `wide` m, `high` m tall, its middle at (x, z) and
/// turned `yaw`.
fn piece(x: Fx, z: Fx, long: Fx, wide: Fx, high: Fx, yaw: i32, s: Surface) -> Brush {
    let shape = block(v3(-long / 2, 0, -wide / 2), v3(long / 2, high, wide / 2), s);
    turned(shape, yaw, v3(x, 0, z))
}

/// The Scrapyard: rows of shipping containers, some stacked two high, with
/// lanes between them.
fn scrapyard(rng: &mut Rng) -> Vec<Brush> {
    let mut bs = walled();
    for gx in -3..=3 {
        for gz in -3..=3 {
            if rng.chance(1, 4) || gx == 0 && gz == 0 {
                continue;
            }
            let (x, z) = (
                int(gx * 13 + rng.between(-2, 2)),
                int(gz * 13 + rng.between(-2, 2)),
            );
            let yaw = if rng.chance(1, 2) { 0 } else { fx::QUARTER_TURN };
            let (long, wide, high) = (int(7), ONE * 25 / 10, ONE * 26 / 10);
            bs.push(piece(x, z, long, wide, high, yaw, Surface::Block));
            if rng.chance(1, 4) {
                let top = turned(
                    block(
                        v3(-long / 2, high, -wide / 2),
                        v3(long / 2, high * 2, wide / 2),
                        Surface::Block,
                    ),
                    yaw,
                    v3(x, 0, z),
                );
                bs.push(top);
            }
        }
    }
    // a low crane deck in the middle to fight over
    bs.push(b(-4, 0, -4, 4, 1, 4, Surface::Block));
    bs
}

/// The Spires: a field of tall columns and a few low walls to duck behind.
fn spires(rng: &mut Rng) -> Vec<Brush> {
    let mut bs = walled();
    for gx in -4..=4 {
        for gz in -4..=4 {
            if rng.chance(1, 5) {
                continue;
            }
            let x = int(gx * 11 + rng.between(-3, 3));
            let z = int(gz * 11 + rng.between(-3, 3));
            let side = int(rng.between(1, 3));
            let high = int(rng.between(5, 14));
            bs.push(piece(x, z, side, side, high, 0, Surface::Block));
        }
    }
    for _ in 0..10 {
        let x = int(rng.between(-44, 44));
        let z = int(rng.between(-44, 44));
        let yaw = fx::TURN / 8 * rng.between(0, 7);
        bs.push(piece(x, z, int(8), ONE / 2, ONE * 12 / 10, yaw, Surface::Wall));
    }
    bs
}

/// The Hub: three broken rings of wall round a tower, the gaps staggered
/// ring to ring.
fn hub(rng: &mut Rng) -> Vec<Brush> {
    let mut bs = walled();
    bs.push(b(-4, 0, -4, 4, 9, 4, Surface::Block));
    for (ring, r) in [16, 30, 44].into_iter().enumerate() {
        let n = 10 + r / 3;
        // each segment fills 60% of its sector; the ring turns half a sector
        // from the last, so no gap lines up with the next
        // the circumference, 2πr, as 44/7 of r
        let arc = int(r) * 44 / 7 / n;
        let long = arc * 6 / 10;
        let skew = fx::TURN / n / 2 * ring as i32;
        for k in 0..n {
            if rng.chance(1, 6) {
                continue;
            }
            let a = fx::TURN * k / n + skew;
            let (x, z) = (fx::mul(int(r), fx::sin(a)), fx::mul(int(r), fx::cos(a)));
            bs.push(piece(x, z, long, ONE * 6 / 10, int(3), a, Surface::Wall));
        }
    }
    bs
}

/// The royale map of that name, with its spawns (the first is yours) and
/// its weapons.
pub fn arena(name: &str) -> Map {
    let seed = ROYALES.iter().position(|m| *m == name).unwrap_or(0) as u64;
    let mut rng = Rng::new(0x5259_414c ^ seed.wrapping_mul(0x9e37_79b9));
    let (brushes, name) = match name {
        "scrapyard" => (scrapyard(&mut rng), "royale_scrapyard"),
        "spires" => (spires(&mut rng), "royale_spires"),
        _ => (hub(&mut rng), "royale_hub"),
    };
    let mut map = Map {
        brushes,
        spawns: Vec::new(),
        name,
        pickups: Vec::new(),
        bot_loop: Vec::new(),
        vines: Vec::new(),
        cube: None,
    };
    let nav = crate::nav::Nav::of(&map);
    let open = |p: V3| nav.is_open(p);
    let edge = HALF - 4;
    let mut spawns = Vec::new();
    let mut g = -edge;
    while g <= edge {
        let mut h = -edge;
        while h <= edge {
            let at = v3(
                int(g + rng.between(-JITTER, JITTER)),
                0,
                int(h + rng.between(-JITTER, JITTER)),
            );
            if open(at) {
                // facing the middle
                let yaw = yaw_of(v3(-at.x, 0, -at.z));
                spawns.push((at, yaw));
            }
            h += SPAWN_GRID;
        }
        g += SPAWN_GRID;
    }
    // shuffled, so the seats spread over the whole floor
    for k in (1..spawns.len()).rev() {
        let j = rng.below(k as u32 + 1) as usize;
        spawns.swap(k, j);
    }
    spawns.truncate(FIGHTERS);
    map.spawns = spawns;
    let mut loot = Vec::new();
    let mut g = -edge + LOOT_GRID / 2;
    while g <= edge {
        let mut h = -edge + LOOT_GRID / 2;
        while h <= edge {
            let at = v3(int(g), 0, int(h));
            if open(at) {
                loot.push((at, LOOT[loot.len() % LOOT.len()]));
            }
            h += LOOT_GRID;
        }
        g += LOOT_GRID;
    }
    map.pickups = loot;
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_royale_map_seats_a_hundred_and_one_on_open_floor_with_weapons_about() {
        for name in ROYALES {
            let m = arena(name);
            assert_eq!(m.spawns.len(), FIGHTERS, "{name}");
            let nav = crate::nav::Nav::of(&m);
            for &(at, _) in &m.spawns {
                assert!(nav.is_open(at), "{name}: {at:?}");
            }
            assert!(m.pickups.len() >= 30, "{name}: {} weapons", m.pickups.len());
            // all on one floor a walk reaches from the first spawn
            let reach = nav.reachable(&m, m.spawns[0].0);
            assert!(reach.len() > 20_000, "{name}: {} cells", reach.len());
        }
    }

    #[test]
    fn no_two_spawns_share_a_place() {
        for name in ROYALES {
            let m = arena(name);
            for (i, a) in m.spawns.iter().enumerate() {
                for b in &m.spawns[i + 1..] {
                    assert!(a.0.sub(b.0).len_xz() > int(3), "{name}");
                }
            }
        }
    }
}
