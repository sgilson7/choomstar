//! Long Time Comin's foes and their orbs, ported for Choomstar's arenas and
//! bosses (CM-8). The volleys are Long Time Comin's `orbs.rs` `fire` at
//! commit 609b130, with its constants: a ring tilts through the target by
//! at most `RING_TILT`, an aimed fan spreads and sweeps, a sphere stacks
//! rings over the sky, an orb curls, speeds up or slows between its bounds.
//! Its patterns, foes and balance numbers are data (`data/ltc.json`).
//!
//! Two of its shapes are drawn simpler here (SECOND-ORDER R2.4): a held
//! formation (`form`) hangs as a ring round its centre and sets off inward,
//! and a ground wave is a flat ring of orbs along the floor (jump it).
//!
//! A foe is not a seat. It is drawn by this module and struck through the
//! world's straw targets: each foe stands as a few hit bodies (`parts`)
//! that the mission keeps full, summing what they took into the foe's
//! health, so Chumontillado's weapons hit a foe exactly as they hit a
//! target (CM-1).

use std::collections::BTreeMap;
use std::sync::OnceLock;

use serde::Deserialize;

use crate::fx::{self, Fx, ONE};
use crate::geom::{facing, right_of, v3, V3};
use crate::map::Map;

/// A ring tilts toward a target above or below it by at most this much
/// (Long Time Comin's `orbs::RING_TILT`).
pub const RING_TILT: i32 = fx::deg(35);
/// The body an orb meets: Chumontillado's movement box (m), its radius and
/// height.
const BODY_RADIUS: Fx = crate::movement::HALF_WIDTH;
const BODY_HEIGHT: Fx = ONE * 18 / 10;
/// Ticks between a foe's touches.
const TOUCH_AGAIN: i32 = 30;
/// A wave's orb sits this far over the floor (m) and is this big.
const WAVE_UP: Fx = ONE * 3 / 10;
const WAVE_SIZE: Fx = ONE * 35 / 100;
/// A blinker moves this far from you (m) when it blinks.
const BLINK_NEAR: i32 = 7;
const BLINK_FAR: i32 = 13;
/// Orbs past this many are not fired (the page's drawing budget).
pub const ORBS_MOST: usize = 700;

#[derive(Debug, Clone, Deserialize)]
pub struct Balance {
    pub orb_damage: i32,
    pub wave_damage: i32,
    pub orb_life_ms: i32,
    pub orb_hit_pct: i32,
    pub parry_reach_cm: i32,
    pub parry_speed_cms: i32,
    pub parry_damage: i32,
    pub wave_orbs: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Attack {
    pub pattern: String,
    pub windup_ms: i32,
    pub rest_ms: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Phase {
    pub from_pct: i32,
    #[serde(default)]
    pub summons: Vec<String>,
    pub attacks: Vec<Attack>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FoeSpec {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub hp: i32,
    pub radius_cm: i32,
    pub hover_cm: i32,
    pub speed_cms: i32,
    pub range_cm: i32,
    pub contact: i32,
    #[serde(default)]
    pub attacks: Vec<Attack>,
    #[serde(default)]
    pub phases: Vec<Phase>,
    #[serde(default)]
    pub summon: Vec<String>,
    #[serde(default)]
    pub summon_ms: i32,
    #[serde(default)]
    pub summon_cap: i32,
}

fn one() -> i32 {
    1
}

#[derive(Debug, Clone, Deserialize)]
pub struct Volley {
    pub shape: String,
    #[serde(default = "one")]
    pub count: i32,
    #[serde(default)]
    pub speed_cms: i32,
    #[serde(default)]
    pub accel_cms2: i32,
    #[serde(default)]
    pub min_cms: i32,
    #[serde(default)]
    pub max_cms: i32,
    #[serde(default)]
    pub curl_degs: i32,
    #[serde(default)]
    pub alternate: bool,
    #[serde(default)]
    pub size_cm: i32,
    #[serde(default)]
    pub colour: String,
    #[serde(default)]
    pub spin_deg: i32,
    #[serde(default = "one")]
    pub repeat: i32,
    #[serde(default)]
    pub every_ms: i32,
    #[serde(default)]
    pub delay_ms: i32,
    #[serde(default)]
    pub pitch_deg: i32,
    #[serde(default)]
    pub spread_deg: i32,
    #[serde(default)]
    pub sweep_deg: i32,
    #[serde(default)]
    pub rings: i32,
    #[serde(default)]
    pub gold: bool,
    #[serde(default)]
    pub aim: bool,
    #[serde(default)]
    pub damage: i32,
    #[serde(default)]
    pub radius_cm: i32,
    #[serde(default)]
    pub hold_ms: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Pattern {
    pub volleys: Vec<Volley>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Data {
    pub balance: Balance,
    pub colours: BTreeMap<String, [i32; 3]>,
    pub enemies: Vec<FoeSpec>,
    pub patterns: BTreeMap<String, Pattern>,
}

impl Data {
    pub fn parse(text: &str) -> Result<Data, String> {
        serde_json::from_str(text).map_err(|e| format!("data/ltc.json: {e}"))
    }

    pub fn foe(&self, id: &str) -> Option<usize> {
        self.enemies.iter().position(|e| e.id == id)
    }

    pub fn colour(&self, name: &str) -> [i32; 3] {
        self.colours.get(name).copied().unwrap_or([255, 255, 255])
    }
}

/// The foes and patterns, read once.
pub fn data() -> &'static Data {
    static DATA: OnceLock<Data> = OnceLock::new();
    DATA.get_or_init(|| Data::parse(include_str!("../../../data/ltc.json")).expect("data/ltc.json"))
}

/// cm/s to m a tick (60 a second), and cm/s² to m a tick a tick.
pub fn per_tick(cms: i32) -> Fx {
    (cms as i64 * ONE as i64 / 6000) as Fx
}
fn per_tick2(cms2: i32) -> Fx {
    (cms2 as i64 * ONE as i64 / 360_000) as Fx
}
pub fn ticks(ms: i32) -> i32 {
    ms * 60 / 1000
}
pub fn cm(c: i32) -> Fx {
    (c as i64 * ONE as i64 / 100) as Fx
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Orb {
    pub pos: V3,
    pub prev: V3,
    pub dir: V3,
    pub speed: Fx,
    pub accel: Fx,
    pub min_speed: Fx,
    pub max_speed: Fx,
    /// turned about the vertical each tick (angle units)
    pub curl: i32,
    pub radius: Fx,
    pub rgb: [i32; 3],
    pub gold: bool,
    pub age: i32,
    pub life: i32,
    pub damage: i32,
    /// the foe that fired it
    pub owner: usize,
    /// struck back by a melee swing: it now hurts foes, not you
    pub parried: bool,
    /// ticks it hangs still before it sets off, the speed it sets off at,
    /// and the point it sets off toward
    pub wait: i32,
    pub go: Fx,
    pub hub: V3,
}

impl Orb {
    /// One tick of flight; false when spent (it met the map, or its time
    /// is up).
    pub fn step(&mut self, map: &Map) -> bool {
        self.prev = self.pos;
        self.age += 1;
        if self.wait > 0 {
            self.wait -= 1;
            if self.wait == 0 {
                let to = self.hub.sub(self.pos);
                if to != V3::ZERO {
                    self.dir = to.norm();
                }
                self.speed = self.go;
            }
            return self.age < self.life;
        }
        if self.curl != 0 {
            let (s, c) = (fx::sin(self.curl), fx::cos(self.curl));
            let d = self.dir;
            self.dir = v3(
                fx::mul(d.x, c) + fx::mul(d.z, s),
                d.y,
                fx::mul(d.z, c) - fx::mul(d.x, s),
            );
            if self.age % 16 == 0 {
                self.dir = self.dir.norm();
            }
        }
        if self.accel != 0 {
            self.speed = (self.speed + self.accel).clamp(self.min_speed, self.max_speed);
        }
        self.pos = self.pos.add(self.dir.scale(self.speed));
        self.age < self.life && !solid(map, self.pos, self.radius / 2)
    }

    /// Does it touch a body standing with its feet at `feet`? The body is
    /// an upright capsule (Long Time Comin's `Orb::touches`).
    pub fn touches(&self, feet: V3, hit_pct: i32) -> bool {
        let r = self.radius * hit_pct / 100 + BODY_RADIUS;
        let lo = feet.y + BODY_RADIUS;
        let hi = feet.y + BODY_HEIGHT - BODY_RADIUS;
        let nearest = v3(feet.x, self.pos.y.clamp(lo, hi), feet.z);
        within(self.pos, nearest, r)
    }
}

pub fn within(a: V3, b: V3, r: Fx) -> bool {
    let d = a.sub(b);
    let d2 = d.x as i64 * d.x as i64 + d.y as i64 * d.y as i64 + d.z as i64 * d.z as i64;
    d2 < r as i64 * r as i64
}

/// Whether a ball at `p` of radius `r` is inside any of the map's solids.
pub fn solid(map: &Map, p: V3, r: Fx) -> bool {
    map.brushes.iter().any(|b| {
        p.x + r >= b.mins.x
            && p.x - r <= b.maxs.x
            && p.y + r >= b.mins.y
            && p.y - r <= b.maxs.y
            && p.z + r >= b.mins.z
            && p.z - r <= b.maxs.z
            && b.holds(p)
    })
}

fn yaw_pitch_of(d: V3) -> (i32, i32) {
    (fx::atan2(-d.x, -d.z), fx::wrap(fx::atan2(d.y, d.len_xz())))
}

/// One repeat of a volley from `origin` at `target` (Long Time Comin's
/// `orbs::fire`). `repeat` counts the repeats already fired, for the spin
/// between them; `floor` is the floor under the foe, for a wave.
pub fn fire(v: &Volley, origin: V3, target: V3, repeat: i32, owner: usize, floor: Fx) -> Vec<Orb> {
    let d = data();
    let b = &d.balance;
    let rgb = d.colour(&v.colour);
    let (aim_yaw, aim_pitch) = yaw_pitch_of(target.sub(origin));
    let spin = fx::deg(v.spin_deg).wrapping_mul(repeat);
    let speed = per_tick(v.speed_cms);
    let curl = fx::deg(v.curl_degs) / 60;
    let damage = if v.damage > 0 { v.damage } else { b.orb_damage };
    let make = |pos: V3, dir: V3, curl: i32| Orb {
        pos,
        prev: pos,
        dir,
        speed,
        accel: per_tick2(v.accel_cms2),
        min_speed: per_tick(v.min_cms),
        max_speed: per_tick(v.max_cms).max(speed),
        curl,
        radius: cm(v.size_cm),
        rgb,
        gold: v.gold,
        age: 0,
        life: ticks(b.orb_life_ms),
        damage,
        owner,
        parried: false,
        wait: 0,
        go: speed,
        hub: pos,
    };
    let curl_of = |i: i32| if v.alternate && i % 2 == 1 { -curl } else { curl };
    let mut out = Vec::new();
    match v.shape.as_str() {
        "ring" => {
            let tilt = fx::wrap(aim_pitch).clamp(-RING_TILT, RING_TILT) + fx::deg(v.pitch_deg);
            let along = facing(aim_yaw, tilt);
            let across = right_of(aim_yaw).neg();
            let base = if v.aim { 0 } else { aim_yaw.wrapping_neg() }.wrapping_add(spin);
            for i in 0..v.count {
                let theta = base.wrapping_add(fx::TURN / v.count * i);
                let dir = along.scale(fx::cos(theta)).add(across.scale(fx::sin(theta)));
                out.push(make(origin, dir.norm(), curl_of(i)));
            }
        }
        "aimed" => {
            let sweep = if v.sweep_deg != 0 && v.repeat > 1 {
                -fx::deg(v.sweep_deg) / 2 + fx::deg(v.sweep_deg) / (v.repeat - 1) * repeat
            } else {
                0
            };
            let spread = fx::deg(v.spread_deg);
            for i in 0..v.count {
                let off = if v.count == 1 {
                    0
                } else {
                    -spread / 2 + spread * i / (v.count - 1)
                };
                let yaw = aim_yaw.wrapping_add(off).wrapping_add(spin).wrapping_add(sweep);
                out.push(make(origin, facing(yaw, aim_pitch), curl_of(i)));
            }
        }
        "sphere" => {
            for r in 0..v.rings.max(1) {
                let pitch = if v.rings <= 1 {
                    0
                } else {
                    -fx::deg(50) + fx::deg(100) * r / (v.rings - 1)
                };
                let n = (fx::mul(v.count * ONE, fx::cos(pitch)) / ONE).max(3);
                let base = spin.wrapping_add(fx::TURN / n / 2 * (r % 2));
                for i in 0..n {
                    out.push(make(
                        origin,
                        facing(base.wrapping_add(fx::TURN / n * i), pitch),
                        curl_of(i),
                    ));
                }
            }
        }
        "wave" => {
            // R2.4: a flat ring of orbs along the floor, out to the wave's reach
            let at = v3(origin.x, floor + WAVE_UP, origin.z);
            let n = b.wave_orbs;
            let reach_ticks = if speed > 0 { cm(v.size_cm) / speed } else { 0 };
            for i in 0..n {
                let mut o = make(at, facing(spin.wrapping_add(fx::TURN / n * i), 0), 0);
                o.radius = WAVE_SIZE;
                o.damage = b.wave_damage;
                o.life = reach_ticks.max(1);
                out.push(o);
            }
        }
        _ => {
            // R2.4: a held formation hangs round its centre, then sets off in
            let r = cm(v.radius_cm.max(200));
            let hub = target;
            for i in 0..v.count {
                let a = spin.wrapping_add(fx::TURN / v.count.max(1) * i);
                let pos = hub.add(v3(fx::mul(r, fx::cos(a)), 0, fx::mul(r, fx::sin(a))));
                let mut o = make(pos, V3::ZERO, 0);
                o.speed = 0;
                o.wait = ticks(v.hold_ms.max(400)).max(1);
                o.hub = hub;
                out.push(o);
            }
        }
    }
    out
}

/// How a foe moves, from its kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gait {
    /// still where it was put (turrets, the owl on its perch)
    Still,
    /// flies, keeping its range from you
    Floats,
    /// walks the floor toward you, to its range
    Walks,
    /// flies straight at you to touch
    Dives,
    /// still, and moves elsewhere after each attack
    Blinks,
}

pub fn gait(kind: &str) -> Gait {
    match kind {
        "floater" | "boss" => Gait::Floats,
        "walker" | "agent" => Gait::Walks,
        "moth" => Gait::Dives,
        "blinker" => Gait::Blinks,
        _ => Gait::Still,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Beat {
    Windup(i32),
    Rest(i32),
}

/// A foe in the arena.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Foe {
    pub spec: usize,
    /// its middle
    pub at: V3,
    pub prev: V3,
    /// the floor it stands or hovers over
    pub floor: Fx,
    pub hp: i32,
    pub most: i32,
    pub phase: usize,
    pub attack: usize,
    pub beat: Beat,
    /// volleys still to fire: (ticks until, volley, repeat)
    pub queue: Vec<(i32, usize, i32)>,
    pub pattern: String,
    pub touch: i32,
    pub summon_in: i32,
    pub flash: i32,
    /// its hit bodies' first index among the world's targets, and how many
    pub parts: (usize, usize),
    /// felled: the mission removes it
    pub down: bool,
}

/// What a foe did this tick.
#[derive(Debug, Default)]
pub struct Did {
    pub orbs: Vec<Orb>,
    pub summons: Vec<String>,
    pub touched: i32,
}

impl Foe {
    pub fn new(spec: usize, at: V3, floor: Fx) -> Foe {
        let s = &data().enemies[spec];
        let hover = cm(s.hover_cm).max(cm(s.radius_cm));
        Foe {
            spec,
            at: v3(at.x, floor + hover, at.z),
            prev: v3(at.x, floor + hover, at.z),
            floor,
            hp: s.hp,
            most: s.hp,
            phase: 0,
            attack: 0,
            beat: Beat::Rest(ticks(900)),
            queue: Vec::new(),
            pattern: String::new(),
            touch: 0,
            summon_in: ticks(s.summon_ms.max(1)),
            flash: 0,
            parts: (0, 0),
            down: false,
        }
    }

    pub fn spec(&self) -> &'static FoeSpec {
        &data().enemies[self.spec]
    }

    pub fn radius(&self) -> Fx {
        cm(self.spec().radius_cm)
    }

    fn attacks(&self) -> &'static [Attack] {
        let s = self.spec();
        match s.phases.get(self.phase) {
            Some(p) => &p.attacks,
            None => &s.attacks,
        }
    }

    /// One tick: move, touch, wind up, fire. `chest` and `feet` are the
    /// player's; `spots` are places on the floor it may blink to.
    pub fn step(&mut self, me: usize, chest: V3, feet: V3, spots: &[V3], rng: &mut crate::rng::Rng) -> Did {
        let s = self.spec();
        let mut did = Did::default();
        self.prev = self.at;
        self.flash = (self.flash - 1).max(0);
        self.touch = (self.touch - 1).max(0);
        // a phase begins as health falls past it, and calls its summons
        let pct = self.hp * 100 / self.most.max(1);
        while self.phase + 1 < s.phases.len() && pct <= s.phases[self.phase + 1].from_pct {
            self.phase += 1;
            self.attack = 0;
            self.queue.clear();
            did.summons.extend(s.phases[self.phase].summons.iter().cloned());
        }
        if !s.summon.is_empty() {
            self.summon_in -= 1;
            if self.summon_in <= 0 {
                self.summon_in = ticks(s.summon_ms.max(1000));
                did.summons
                    .push(s.summon[rng.below(s.summon.len() as u32) as usize].clone());
            }
        }
        let speed = per_tick(s.speed_cms);
        let range = cm(s.range_cm);
        let hover = cm(s.hover_cm).max(self.radius());
        let to = v3(chest.x - self.at.x, 0, chest.z - self.at.z);
        let flat = to.len_xz();
        match gait(&s.kind) {
            Gait::Still | Gait::Blinks => {}
            Gait::Floats => {
                let want = flat - range;
                if want.abs() > ONE && to != V3::ZERO {
                    let step = want.clamp(-speed, speed);
                    self.at = self.at.add(to.norm().scale(step));
                }
                self.at.y = self.floor + hover;
            }
            Gait::Walks => {
                if flat > range && to != V3::ZERO {
                    let next = self.at.add(to.norm().scale(speed.min(flat - range)));
                    if spots.iter().any(|p| p.sub(next).len_xz() < ONE) {
                        self.at = next;
                    }
                }
            }
            Gait::Dives => {
                let d = chest.sub(self.at);
                if d != V3::ZERO {
                    self.at = self.at.add(d.norm().scale(speed.min(d.len())));
                }
            }
        }
        // a touch hurts
        let reach = self.radius() + BODY_RADIUS + ONE / 4;
        if self.touch == 0 && within(self.at, chest, reach) {
            self.touch = TOUCH_AGAIN;
            did.touched = s.contact;
        }
        let _ = feet;
        // the attack's beat: wind up, fire its volleys, rest
        let attacks = self.attacks();
        if !attacks.is_empty() {
            match self.beat {
                Beat::Rest(t) if t > 1 => self.beat = Beat::Rest(t - 1),
                Beat::Rest(_) => {
                    let a = &attacks[self.attack % attacks.len()];
                    self.pattern = a.pattern.clone();
                    self.beat = Beat::Windup(ticks(a.windup_ms).max(1));
                }
                Beat::Windup(t) if t > 1 => self.beat = Beat::Windup(t - 1),
                Beat::Windup(_) => {
                    let a = &attacks[self.attack % attacks.len()];
                    if let Some(p) = data().patterns.get(&a.pattern) {
                        for (k, v) in p.volleys.iter().enumerate() {
                            for r in 0..v.repeat.max(1) {
                                self.queue.push((ticks(v.delay_ms) + ticks(v.every_ms) * r, k, r));
                            }
                        }
                    }
                    self.attack += 1;
                    self.beat = Beat::Rest(ticks(a.rest_ms).max(1));
                    if gait(&s.kind) == Gait::Blinks && !spots.is_empty() {
                        // blinks to a place 7 to 13 m from you
                        let near: Vec<&V3> = spots
                            .iter()
                            .filter(|p| {
                                let d = p.sub(feet).len_xz();
                                d >= fx::int(BLINK_NEAR) && d <= fx::int(BLINK_FAR)
                            })
                            .collect();
                        if !near.is_empty() {
                            let p = *near[rng.below(near.len() as u32) as usize];
                            self.floor = p.y;
                            self.at = v3(p.x, p.y + hover, p.z);
                            self.prev = self.at;
                        }
                    }
                }
            }
        }
        let pattern = data().patterns.get(&self.pattern);
        let mut left = Vec::new();
        for (t, k, r) in std::mem::take(&mut self.queue) {
            if t > 0 {
                left.push((t - 1, k, r));
            } else if let Some(v) = pattern.and_then(|p| p.volleys.get(k)) {
                did.orbs.extend(fire(v, self.at, chest, r, me, self.floor));
            }
        }
        self.queue = left;
        did
    }

    /// Winding up: how far through, 0 to ONE.
    pub fn winding(&self) -> Fx {
        match self.beat {
            Beat::Windup(t) => {
                let a = self.attacks();
                let all = a
                    .get((self.attack) % a.len().max(1))
                    .map_or(1, |a| ticks(a.windup_ms).max(1));
                ONE - ONE * t / all
            }
            Beat::Rest(_) => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_foes_and_patterns_load_and_every_attack_names_a_pattern() {
        let d = data();
        assert_eq!(d.enemies.len(), 10);
        for e in &d.enemies {
            for a in e
                .attacks
                .iter()
                .chain(e.phases.iter().flat_map(|p| p.attacks.iter()))
            {
                assert!(
                    d.patterns.contains_key(&a.pattern),
                    "{} names {}",
                    e.id,
                    a.pattern
                );
            }
        }
    }

    #[test]
    fn a_ring_of_sixteen_leaves_on_sixteen_ways_and_flies_at_its_speed() {
        // ring_16: 16 orbs at 720 cm/s = 0.12 m a tick
        let v = &data().patterns["ring_16"].volleys[0];
        let orbs = fire(v, v3(0, ONE, 0), v3(0, ONE, -fx::int(10)), 0, 0, 0);
        assert_eq!(orbs.len(), 16);
        assert_eq!(orbs[0].speed, ONE * 12 / 100);
        // all different ways, all unit length
        for (i, a) in orbs.iter().enumerate() {
            assert!((a.dir.len() - ONE).abs() < ONE / 100);
            for b in &orbs[i + 1..] {
                assert!(a.dir.sub(b.dir).len() > ONE / 4);
            }
        }
    }

    #[test]
    fn an_aimed_single_flies_straight_at_its_target() {
        // snipe's first volley: one orb at 1800 cm/s toward the target
        let v = &data().patterns["snipe"].volleys[0];
        let orbs = fire(v, v3(0, ONE, 0), v3(fx::int(10), ONE, 0), 0, 0, 0);
        assert_eq!(orbs.len(), 1);
        assert!(orbs[0].dir.x > ONE * 99 / 100, "{:?}", orbs[0].dir);
        assert_eq!(orbs[0].speed, ONE * 3 / 10);
        assert_eq!(orbs[0].damage, 16);
    }

    #[test]
    fn an_orb_touches_a_body_it_meets_and_not_one_a_metre_off() {
        let mut o = fire(
            &data().patterns["snipe"].volleys[0],
            V3::ZERO,
            v3(ONE, 0, 0),
            0,
            0,
            0,
        )
        .remove(0);
        o.pos = v3(0, ONE, 0);
        assert!(o.touches(v3(0, 0, 0), 62));
        assert!(!o.touches(v3(fx::int(1), 0, 0), 62));
    }

    #[test]
    fn a_turret_winds_up_then_fires_its_first_attack() {
        let s = data().foe("sentinel").unwrap();
        let mut f = Foe::new(s, V3::ZERO, 0);
        let mut rng = crate::rng::Rng::new(1);
        let mut fired = 0;
        // rest 900 ms (54 ticks), wind up 500 ms (30), then the spiral
        for _ in 0..100 {
            fired += f
                .step(0, v3(0, ONE, -fx::int(8)), v3(0, 0, -fx::int(8)), &[], &mut rng)
                .orbs
                .len();
        }
        assert!(fired >= 4, "{fired}");
        assert_eq!(f.pattern, "spiral_quad");
    }

    #[test]
    fn the_interdictor_enters_its_second_phase_at_two_thirds_and_calls_its_summons() {
        let s = data().foe("bailiff").unwrap();
        let mut f = Foe::new(s, V3::ZERO, 0);
        let mut rng = crate::rng::Rng::new(1);
        f.hp = f.most * 66 / 100;
        let did = f.step(0, v3(0, ONE, -fx::int(8)), V3::ZERO, &[], &mut rng);
        assert_eq!(f.phase, 1);
        assert_eq!(did.summons, vec!["moth", "moth", "lantern"]);
    }
}
