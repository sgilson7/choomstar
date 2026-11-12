//! Choomstar's missions: the pack (`data/missions.json`), which missions are
//! open, the medals, and a mission's run on Chumontillado's world.
//!
//! A mission is data (CM-9): a kind, a map, a loadout, a goal, medal marks
//! and foes. The rules here read it and change the world only through the
//! hooks `world.rs` keeps for missions (sides, a bot's own skill and pace,
//! targets that stay down, `World::strike`), so movement and weapons are
//! Chumontillado's own (CM-1). Every draw comes from a seeded `Rng`, so a
//! mission plays the same on the same inputs (CM-10).

use std::collections::BTreeMap;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use crate::arms::Arms;
use crate::fx::{self, Fx, ONE};
use crate::geom::{v3, Affine, V3};
use crate::look::{rgb, Look};
use crate::ltc::{self, Foe, Orb};
use crate::pilot::{Pilot, Skill};
use crate::prim::Mesh;
use crate::render::Shapes;
use crate::rng::Rng;
use crate::weapons::Weapon;
use crate::world::{Dummy, Player, Rules, World, DUMMY_TALL, HEALTH};

/// The maps by the names missions use, in `maps::map` order.
pub const MAPS: [&str; crate::maps::COUNT] = [
    "plaza",
    "catedral",
    "galeria",
    "mercado",
    "puerto",
    "acueducto",
    "molino",
    "alcazar",
    "dehesa",
    "sierra",
    "arrabal",
    "tobogan",
    "selva",
    "barranco",
    "cubo",
];

/// The skins by the names missions use, in `skins::SKINS` order.
pub const SKINS: [&str; crate::skins::COUNT] = [
    "hidalgo",
    "infanta",
    "harlequin",
    "sordo",
    "friar",
    "torero",
    "black_knight",
    "ivory_knight",
];

/// The weapons by the names missions use, in `weapons::ALL` order.
pub const WEAPONS: [&str; crate::weapons::COUNT] = [
    "pistol",
    "shotgun",
    "machine_pistol",
    "rifle",
    "pincel",
    "drum",
    "trabuco",
    "toro",
    "campana",
    "collage",
    "katana",
    "navaja",
    "guadana",
    "punal",
    "garrocha",
    "ak",
];

/// The kinds, in the chart's spoke order (the journal's star chart).
pub const KINDS: [&str; 13] = [
    "targets", "fixed", "combat", "melee", "team", "horde", "salvage", "arena", "boss", "gauntlet",
    "parkour", "longjump", "royale",
];

pub const NONE: u8 = 0;
pub const BRONZE: u8 = 1;
pub const SILVER: u8 = 2;
pub const GOLD: u8 = 3;

/// Ticks a second.
const SECOND: i32 = 60;
/// A gate is passed within this of its foot, across (m) and up.
const GATE_REACH: Fx = ONE * 22 / 10;
const GATE_UP: Fx = fx::int(4);
/// A relic, a repair cell, is taken within this (m); the lift takes
/// relics within this.
const TAKE_REACH: Fx = ONE * 15 / 10;
const LIFT_REACH: Fx = fx::int(3);
/// Standing on the open lift empty-handed this long takes it.
const LIFT_TICKS: i32 = 90;
/// A long jump's field is marked every this many metres past the lip, this
/// far, by lines this wide.
const MARK_EVERY: i32 = 10;
const MARKS_PAST_LIP: i32 = 200;
const MARK_WIDE: i32 = 60;
/// A repair cell's health.
const REPAIR: i32 = 35;
/// How far apart (m) placed things are kept, and how far from the spawn.
const SPREAD: i32 = 5;
const SPAWN_CLEAR: i32 = 8;
/// Zombies come from at least this far (m) from you, if the map allows.
const ZOMBIE_FAR: i32 = 14;
/// The most zombies on the map at once, and seats in all.
const ZOMBIES_MOST: i32 = 14;
const SEATS_MOST: usize = 24;
/// Places tried for where you stand again in a royale.
const SAFE_TRIES: i32 = 48;
/// A moving target swings this far each way (m) over this many ticks.
const SWAY: i32 = 3;
const SWAY_TICKS: i32 = 240;
/// Ticks between a cleared wave and the next.
const BREATHER: i32 = 120;
/// Chumontillado's round count before play (3 s) and a round's end shown
/// before the next (4 s), for a mission's start, its rounds and stages.
pub const COUNT_TICKS: i32 = crate::rounds::COUNT_TICKS;
pub const OVER_TICKS: i32 = crate::rounds::OVER_TICKS;
/// A foe's hit body is kept this full; what it lost is the foe's damage.
const PART_FULL: i32 = 100_000;
/// The Black Knight as a boss: its health, and where it takes up the trabuco.
const KNIGHT_HEALTH: i32 = 600;
const KNIGHT_RAGE_PCT: i32 = 50;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Req {
    pub mission: String,
    pub medal: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Goal {
    pub kind: String,
    pub targets: i32,
    pub limit: i32,
    pub gates: i32,
    pub seconds: i32,
    pub count: i32,
    pub rounds: i32,
    pub ladder: i32,
    pub score: i32,
    pub waves: i32,
    pub relics: i32,
    pub need: i32,
    pub stages: i32,
    pub repairs: i32,
    pub moving: bool,
    /// a royale's difficulty: easy, normal, hard or brutal
    pub difficulty: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Medals {
    pub measure: String,
    pub bronze: i32,
    pub silver: i32,
    pub gold: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct FoeRef {
    pub bot: String,
    pub level: i32,
    pub weapon: String,
    pub side: String,
    pub zombie: String,
    pub waves: String,
    pub from: i32,
    pub enemy: String,
    pub boss: String,
    pub stage: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Spec {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub map: String,
    pub loadout: String,
    pub requires: Vec<Req>,
    pub goal: Goal,
    pub medals: Medals,
    pub foes: Vec<FoeRef>,
    pub brief: String,
    #[serde(default)]
    pub secret: bool,
    #[serde(default)]
    pub row: Option<i32>,
}

impl Spec {
    /// Its tier: the count of its requirements, or the row it is given.
    pub fn tier(&self) -> i32 {
        self.row.unwrap_or(self.requires.len() as i32)
    }

    pub fn map_index(&self) -> usize {
        MAPS.iter().position(|m| *m == self.map).unwrap_or(0)
    }

    /// The time it allows, in seconds (0: none).
    pub fn limit(&self) -> i32 {
        self.goal.limit
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Pack {
    pub missions: Vec<Spec>,
}

pub fn parse(text: &str) -> Result<Pack, String> {
    serde_json::from_str(text).map_err(|e| format!("data/missions.json: {e}"))
}

/// Choomstar's first pack, read once.
pub fn pack() -> &'static Pack {
    static PACK: OnceLock<Pack> = OnceLock::new();
    PACK.get_or_init(|| parse(include_str!("../../../data/missions.json")).expect("data/missions.json"))
}

pub fn index(id: &str) -> Option<usize> {
    pack().missions.iter().position(|m| m.id == id)
}

pub fn medal_rank(name: &str) -> u8 {
    match name {
        "gold" => GOLD,
        "silver" => SILVER,
        _ => BRONZE,
    }
}

/// The best a player has done on one mission.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Best {
    pub medal: u8,
    /// in the measure's unit: ticks for time, health or relics
    pub score: i32,
}

pub type Progress = BTreeMap<String, Best>;

/// Whether a mission is open: every medal it asks for is earned (Vagrancy's
/// road; CM-3).
pub fn open(spec: &Spec, progress: &Progress) -> bool {
    spec.requires
        .iter()
        .all(|r| progress.get(&r.mission).map_or(NONE, |b| b.medal) >= medal_rank(&r.medal))
}

/// The requirements not yet met.
pub fn missing<'a>(spec: &'a Spec, progress: &Progress) -> Vec<&'a Req> {
    spec.requires
        .iter()
        .filter(|r| progress.get(&r.mission).map_or(NONE, |b| b.medal) < medal_rank(&r.medal))
        .collect()
}

/// The medal a finished mission earns from its score: lower is better for
/// time (in ticks, against marks in seconds), higher for health and relics.
pub fn medal_for(spec: &Spec, score: i32) -> u8 {
    let m = &spec.medals;
    if m.measure == "time" {
        let secs = (score + SECOND - 1) / SECOND;
        if secs <= m.gold {
            GOLD
        } else if secs <= m.silver {
            SILVER
        } else {
            BRONZE
        }
    } else if score >= m.gold {
        GOLD
    } else if score >= m.silver {
        SILVER
    } else {
        BRONZE
    }
}

/// Whether score `a` beats score `b` for this mission's measure.
pub fn better(spec: &Spec, a: i32, b: i32) -> bool {
    if spec.medals.measure == "time" {
        a < b
    } else {
        a > b
    }
}

/// A mission finished, or a best kept: the new best.
pub fn record(progress: &mut Progress, spec: &Spec, score: i32) -> Best {
    let medal = medal_for(spec, score);
    let old = progress.get(&spec.id).copied().unwrap_or_default();
    let best = Best {
        medal: medal.max(old.medal),
        score: if old.medal == NONE || better(spec, score, old.score) {
            score
        } else {
            old.score
        },
    };
    progress.insert(spec.id.clone(), best);
    best
}

/// One route into each mission (Vagrancy): from its requirement of its own
/// kind first, else in the highest tier, else the latest.
pub fn route_from(k: usize) -> Option<usize> {
    let all = &pack().missions;
    let m = &all[k];
    m.requires
        .iter()
        .filter_map(|r| index(&r.mission))
        .max_by_key(|&q| (all[q].kind == m.kind, all[q].tier(), std::cmp::Reverse(q)))
}

/// The star chart's place for each mission, in thousandths of the chart's
/// side (the journal's star chart, CM-4): tier t on a ring of radius
/// 470 − 68t, each kind on its own spoke, the sixth tier near the middle
/// and the secret finale at it.
pub fn chart_at(k: usize) -> (i32, i32) {
    let m = &pack().missions[k];
    if m.secret {
        return (500, 500);
    }
    let t = m.tier();
    let r = if t <= 5 { 470 - t * 68 } else { 82 };
    let spoke = KINDS.iter().position(|x| *x == m.kind).unwrap_or(0) as i32;
    let a = -fx::QUARTER_TURN + fx::TURN / KINDS.len() as i32 * spoke;
    (
        500 + fx::mul(r * ONE, fx::cos(a)) / ONE,
        500 + fx::mul(r * ONE, fx::sin(a)) / ONE,
    )
}

/// The ring radius of tier `t` on the chart (thousandths).
pub fn ring(t: i32) -> i32 {
    if t <= 5 {
        470 - t * 68
    } else {
        82
    }
}

fn weapon(name: &str) -> Option<Weapon> {
    WEAPONS
        .iter()
        .position(|w| *w == name)
        .map(|k| crate::weapons::ALL[k])
}

fn skin(name: &str) -> u8 {
    SKINS.iter().position(|s| *s == name).unwrap_or(0) as u8
}

/// A bot's level, 1 to 5, as Chumontillado's three skills and its health.
fn level(l: i32) -> (Skill, i32) {
    match l {
        i32::MIN..=2 => (Skill::Easy, HEALTH),
        3 => (Skill::Normal, HEALTH),
        4 => (Skill::Hard, HEALTH),
        _ => (Skill::Hard, HEALTH * 5 / 4),
    }
}

/// Arms holding one weapon only, full.
fn arms_of(w: Weapon) -> Arms {
    let mut a = Arms::default();
    a.owned = 0;
    a.pick_up(w);
    if w.swing().is_some() {
        a.melee = w;
    }
    a
}

/// What happened at the end.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Outcome {
    pub cleared: bool,
    pub score: i32,
    pub medal: u8,
    /// a status key: why it ended
    pub why: &'static str,
}

/// What the end of a round or stage says while the next one waits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Banner {
    /// you took round `n`
    RoundWon(i32),
    /// the seat wearing this skin took it, or nobody did (your own fall)
    RoundLost(Option<u8>),
    /// fight `n` of a gauntlet is cleared
    StageCleared(i32),
}

/// What follows a round's end once it has been shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Next {
    Round,
    Part,
}

/// What a seat is, to the mission.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    You,
    Ally,
    Bot,
    Zombie,
    Knight,
}

/// One part of a mission: the whole of most, a stage of a gauntlet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Part {
    Targets,
    Course,
    /// zombies: felled to count, or a time to last
    Horde {
        count: i32,
        seconds: i32,
    },
    /// rounds against bots: won, lost, and the ladder's rung
    Duel,
    Team,
    Arena {
        waves: i32,
    },
    Boss,
    Salvage,
    /// a long-jump course: how far along it you go, and the jump off its lip
    LongJump,
    /// a royale: a hundred bots, every one against every other, one life
    /// each; you have the lives the difficulty gives
    Royale,
}

/// A royale's difficulty: the lives you have and the bots' level (the
/// person, 2026-11-12: "a differing number of lives based on the
/// difficulty, while all the enemies only have 1").
pub fn difficulty(name: &str) -> (i32, i32) {
    match name {
        "easy" => (5, 1),
        "hard" => (2, 3),
        "brutal" => (1, 4),
        _ => (3, 2),
    }
}

/// A long-jump course's numbers, as its mission reads them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Jump {
    pub start_z: Fx,
    pub lip: V3,
    pub field_y: Fx,
    pub length: i32,
}

#[derive(Debug, Clone)]
pub struct Run {
    pub k: usize,
    pub parts: Vec<(Part, Vec<FoeRef>)>,
    pub part: usize,
    /// ticks counted (from the first move) and ticks since the part began
    pub ticks: i32,
    pub part_ticks: i32,
    pub started: bool,
    pub outcome: Option<Outcome>,
    /// what the HUD counts: done of need
    pub done: i32,
    pub need: i32,
    pub roles: Vec<Role>,
    pub spots: Vec<V3>,
    pub spawn: V3,
    pub gates: Vec<V3>,
    pub gate: usize,
    pub relics: Vec<(V3, bool)>,
    pub carried: i32,
    pub delivered: i32,
    pub lift_wait: i32,
    pub sway: Vec<V3>,
    pub foes: Vec<Foe>,
    pub orbs: Vec<Orb>,
    pub zombies_out: i32,
    pub won: i32,
    pub lost: i32,
    pub rung: usize,
    pub score: (i32, i32),
    pub wave: i32,
    pub breather: i32,
    pub repairs: i32,
    pub repair: Option<V3>,
    /// a long-jump course's numbers, and where you landed past its lip (m)
    pub jump: Option<Jump>,
    pub landed: Option<i32>,
    pub boss: Option<usize>,
    /// a royale: your lives left, and bots still standing
    pub lives: i32,
    pub standing: i32,
    /// ticks left of the count before play, and of a round's end shown,
    /// with what it says and what comes after
    pub count: i32,
    pub over: i32,
    pub banner: Option<Banner>,
    next: Next,
    alive_was: Vec<bool>,
    rng: Rng,
}

impl Run {
    pub fn spec(&self) -> &'static Spec {
        &pack().missions[self.k]
    }

    /// Mission `k` from its start, in skin `my_skin`: the run and its world.
    pub fn start(k: usize, my_skin: u8, lifelike: bool) -> (Run, World) {
        let spec = &pack().missions[k];
        let mut w = World::room(crate::world::Bot::Moves);
        w.dummies.clear();
        w.players.truncate(1);
        w.start_round(spec.map_index());
        // a long-jump course is a map of its own (courses.rs)
        let jump = crate::courses::is_course(&spec.map).then(|| {
            let c = crate::courses::course(&spec.map);
            let j = Jump {
                start_z: c.start_z,
                lip: c.lip,
                field_y: c.field_y,
                length: c.length,
            };
            w.nav = Some(std::rc::Rc::new(crate::nav::Nav::of(&c.map)));
            w.turned = std::rc::Rc::new(Vec::new());
            w.pickups.clear();
            let spawn = c.map.spawns[0];
            w.map = std::rc::Rc::new(c.map);
            let p = &mut w.players[0];
            p.spawn = spawn;
            p.respawn();
            p.prev = p.body.origin;
            j
        });
        // a royale map is a map of its own too (royale.rs), its weapons
        // lying where it puts them
        if crate::royale::is_royale(&spec.map) {
            let m = crate::royale::arena(&spec.map);
            w.nav = Some(std::rc::Rc::new(crate::nav::Nav::of(&m)));
            w.turned = std::rc::Rc::new(Vec::new());
            w.pickups = m
                .pickups
                .iter()
                .map(|&(at, weapon)| crate::world::Pickup {
                    at,
                    weapon,
                    taken: false,
                })
                .collect();
            let p = &mut w.players[0];
            p.spawn = m.spawns[0];
            p.respawn();
            p.arms = Arms::for_round();
            p.prev = p.body.origin;
            w.map = std::rc::Rc::new(m);
        }
        let team = spec.kind == "team";
        w.rules = Rules {
            hunt: true,
            stay_down: !team,
            no_reset: true,
            frozen: true,
        };
        let me = &mut w.players[0];
        me.skin = my_skin;
        me.lifelike = lifelike;
        me.team = 1;
        match spec.loadout.as_str() {
            "any" => {}
            "none" => {
                // R2.5: the parkour courses carry the pistol, the weapon
                // that changes no movement, and nothing lies on the map
                me.arms = arms_of(Weapon::Pistol);
                w.pickups.clear();
            }
            name => {
                if let Some(g) = weapon(name) {
                    me.arms = arms_of(g);
                }
                w.pickups.clear();
            }
        }
        let spawn = w.players[0].body.origin;
        let nav = w.nav.clone().expect("a round's nav");
        let spots = nav.reachable(&w.map, spawn);
        let parts = match spec.kind.as_str() {
            "gauntlet" => spec
                .foes
                .iter()
                .map(|f| {
                    let part = match f.stage.as_str() {
                        "horde" => Part::Horde {
                            count: 12,
                            seconds: 0,
                        },
                        "combat" => Part::Duel,
                        "arena" => Part::Arena { waves: 2 },
                        _ => Part::Boss,
                    };
                    (part, vec![f.clone()])
                })
                .collect(),
            kind => {
                let g = &spec.goal;
                let part = match (kind, g.kind.as_str()) {
                    (_, "break_targets") => Part::Targets,
                    (_, "reach_end") => Part::Course,
                    (_, "survive") => Part::Horde {
                        count: 0,
                        seconds: g.seconds,
                    },
                    (_, "defeat") => Part::Horde {
                        count: g.count,
                        seconds: 0,
                    },
                    (_, "team_score") => Part::Team,
                    (_, "clear_waves") => Part::Arena { waves: g.waves },
                    (_, "defeat_boss") => Part::Boss,
                    (_, "salvage") => Part::Salvage,
                    (_, "long_jump") => Part::LongJump,
                    (_, "last_standing") => Part::Royale,
                    _ => Part::Duel,
                };
                vec![(part, spec.foes.clone())]
            }
        };
        let mut run = Run {
            k,
            parts,
            part: 0,
            ticks: 0,
            part_ticks: 0,
            started: false,
            outcome: None,
            done: 0,
            need: 0,
            roles: vec![Role::You],
            spots,
            spawn,
            gates: Vec::new(),
            gate: 0,
            relics: Vec::new(),
            carried: 0,
            delivered: 0,
            lift_wait: 0,
            sway: Vec::new(),
            foes: Vec::new(),
            orbs: Vec::new(),
            zombies_out: 0,
            won: 0,
            lost: 0,
            rung: 0,
            score: (0, 0),
            wave: 0,
            breather: 0,
            repairs: spec.goal.repairs,
            repair: None,
            landed: None,
            boss: None,
            lives: difficulty(&spec.goal.difficulty).0,
            standing: 0,
            count: COUNT_TICKS,
            over: 0,
            banner: None,
            next: Next::Round,
            alive_was: vec![true],
            jump,
            rng: Rng::new(0x4348_4f4f_u64 ^ (k as u64).wrapping_mul(0x9e37_79b9)),
        };
        run.begin_part(&mut w);
        // the count before play: nobody moves until it ends
        w.rules.frozen = true;
        (run, w)
    }

    pub fn current(&self) -> &Part {
        &self.parts[self.part.min(self.parts.len() - 1)].0
    }

    fn foes_now(&self) -> Vec<FoeRef> {
        self.parts[self.part.min(self.parts.len() - 1)].1.clone()
    }

    /// `n` places spread over the reachable floor, each at least `SPREAD`
    /// m from the others and `SPAWN_CLEAR` m from the spawn where the map
    /// allows, chosen farthest first from a seeded start.
    fn spread(&mut self, n: usize, from: V3) -> Vec<V3> {
        let far: Vec<V3> = self
            .spots
            .iter()
            .copied()
            .filter(|p| p.sub(from).len_xz() >= fx::int(SPAWN_CLEAR))
            .collect();
        let pool = if far.len() >= n { far } else { self.spots.clone() };
        if pool.is_empty() {
            return vec![from; n];
        }
        let mut out = vec![pool[self.rng.below(pool.len() as u32) as usize]];
        while out.len() < n {
            // the place farthest from every one placed
            let next = pool
                .iter()
                .copied()
                .max_by_key(|p| out.iter().map(|q| p.sub(*q).len_xz()).min().unwrap_or(0))
                .unwrap_or(from);
            if out.iter().any(|q| q.sub(next).len_xz() < fx::int(SPREAD) / 2) {
                out.push(pool[self.rng.below(pool.len() as u32) as usize]);
            } else {
                out.push(next);
            }
        }
        out
    }

    /// A place `lo` to `hi` m from `from`, else the farthest there is.
    fn somewhere(&mut self, from: V3, lo: i32, hi: i32) -> V3 {
        let ring: Vec<V3> = self
            .spots
            .iter()
            .copied()
            .filter(|p| {
                let d = p.sub(from).len_xz();
                d >= fx::int(lo) && d <= fx::int(hi)
            })
            .collect();
        if !ring.is_empty() {
            return ring[self.rng.below(ring.len() as u32) as usize];
        }
        self.spots
            .iter()
            .copied()
            .max_by_key(|p| p.sub(from).len_xz())
            .unwrap_or(from)
    }

    /// Of a few places on the floor, the one whose nearest standing bot is
    /// farthest: where you stand again in a royale.
    fn safest(&mut self, w: &World) -> V3 {
        let bots: Vec<V3> = w.players[1..]
            .iter()
            .filter(|p| p.alive())
            .map(|p| p.body.origin)
            .collect();
        let mut best = (self.spawn, -1);
        for _ in 0..SAFE_TRIES {
            if self.spots.is_empty() {
                break;
            }
            let at = self.spots[self.rng.below(self.spots.len() as u32) as usize];
            let near = bots.iter().map(|b| b.sub(at).len_xz()).min().unwrap_or(i32::MAX);
            if near > best.1 {
                best = (at, near);
            }
        }
        best.0
    }

    /// A free seat for a bot: a felled bot's, or a new one.
    fn seat(&mut self, w: &mut World, role: Role) -> Option<usize> {
        let k = (1..w.players.len())
            .find(|&k| self.roles[k] != Role::You && !w.players[k].alive() && w.rules.stay_down);
        let k = match k {
            Some(k) => k,
            None if w.players.len() < SEATS_MOST
                || role == Role::Bot
                    && w.players.len() < crate::royale::FIGHTERS
                    && crate::royale::is_royale(&self.spec().map) =>
            {
                w.players.push(Player::at((self.spawn, 0)));
                self.roles.push(role);
                self.alive_was.push(true);
                w.players.len() - 1
            }
            None => return None,
        };
        self.roles[k] = role;
        Some(k)
    }

    /// A bot of a mission's foe list into a seat, at `at`.
    fn put_bot(&mut self, w: &mut World, f: &FoeRef, at: V3) -> Option<usize> {
        let ally = f.side == "ally";
        let k = self.seat(w, if ally { Role::Ally } else { Role::Bot })?;
        let (skill, health) = level(f.level);
        let yaw = crate::geom::yaw_of(self.spawn.sub(at));
        let mut p = Player::at((at, yaw));
        p.skin = skin(&f.bot);
        p.team = if ally { 1 } else { 2 };
        p.skill = Some(skill);
        p.health = health;
        p.pilot = Some(Pilot {
            shoots: true,
            ..Pilot::default()
        });
        p.arms = match weapon(&f.weapon) {
            Some(g) => arms_of(g),
            None => Arms::for_round(),
        };
        w.players[k] = p;
        self.alive_was[k] = true;
        Some(k)
    }

    /// A zombie of kind `kind` at `at`.
    fn put_zombie(&mut self, w: &mut World, kind: &str, at: V3) -> Option<usize> {
        let k = self.seat(w, Role::Zombie)?;
        // (pace %, health, weapon, skin): a shambler, a runner, a brute
        let (pace, health, g, s) = match kind {
            "runner" => (85, 40, Weapon::Navaja, crate::skins::ZOMBIE),
            "brute" => (60, 220, Weapon::Guadana, crate::skins::ZOMBIE),
            _ => (45, 60, Weapon::Punal, crate::skins::ZOMBIE),
        };
        let yaw = crate::geom::yaw_of(self.spawn.sub(at));
        let mut p = Player::at((at, yaw));
        p.skin = s;
        p.team = 2;
        p.skill = Some(Skill::Easy);
        p.run_pct = pace;
        p.health = health;
        p.pilot = Some(Pilot {
            shoots: true,
            ..Pilot::default()
        });
        p.arms = arms_of(g);
        w.players[k] = p;
        self.alive_was[k] = true;
        Some(k)
    }

    /// A Long Time Comin' foe, by id (an unknown summon comes as a moth).
    fn put_foe(&mut self, w: &mut World, id: &str, at: V3) {
        let d = ltc::data();
        let s = d.foe(id).or_else(|| d.foe("moth")).unwrap_or(0);
        let foe = Foe::new(s, at, at.y);
        self.foes.push(foe);
        self.sync_parts(w);
    }

    /// The foes' hit bodies, rebuilt: one for a small foe, five in a cross
    /// for a large one, each the height of the foe.
    fn sync_parts(&mut self, w: &mut World) {
        w.dummies.retain(|d| d.kind == 0);
        for (i, f) in self.foes.iter_mut().enumerate() {
            if f.down {
                f.parts = (0, 0);
                continue;
            }
            let r = f.radius();
            let tall = (r * 2).max(ONE * 8 / 10);
            let feet = f.at.sub(v3(0, tall / 2, 0));
            let offsets: Vec<V3> = if r <= ONE * 6 / 10 {
                vec![V3::ZERO]
            } else {
                let o = r / 2;
                vec![V3::ZERO, v3(o, 0, 0), v3(-o, 0, 0), v3(0, 0, o), v3(0, 0, -o)]
            };
            let first = w.dummies.len();
            for o in &offsets {
                w.dummies.push(Dummy {
                    feet: feet.add(*o),
                    health: PART_FULL,
                    down: 0,
                    flash: 0,
                    tall,
                    kind: (i % 250) as u8 + 1,
                });
            }
            f.parts = (first, offsets.len());
        }
    }

    fn begin_part(&mut self, w: &mut World) {
        self.part_ticks = 0;
        self.done = 0;
        self.need = 0;
        self.wave = 0;
        self.breather = 0;
        let foes = self.foes_now();
        let spec = self.spec();
        let me = w.players[0].body.origin;
        match self.current().clone() {
            Part::Targets => {
                let n = spec.goal.targets.max(1) as usize;
                let at = self.spread(n, me);
                w.dummies = at
                    .iter()
                    .map(|&feet| Dummy {
                        feet,
                        health: HEALTH,
                        down: 0,
                        flash: 0,
                        tall: DUMMY_TALL,
                        kind: 0,
                    })
                    .collect();
                self.sway = at;
                self.need = n as i32;
            }
            Part::Course => {
                let n = spec.goal.gates.max(1) as usize;
                // the map's own route: Chumontillado places its weapons
                // along the way a run goes (down the Tobogán's slide, over
                // the Sierra), so a course follows them; the open floor
                // makes up any the map lacks
                let mut left: Vec<V3> = w.map.pickups.iter().map(|&(at, _)| at).collect();
                if left.len() < n {
                    let more = self.spread(n - left.len(), me);
                    left.extend(more);
                }
                // in the order a walk from the spawn meets them
                let mut from = me;
                self.gates.clear();
                while !left.is_empty() {
                    let (i, _) = left
                        .iter()
                        .enumerate()
                        .min_by_key(|(_, p)| p.sub(from).len_xz())
                        .expect("a gate left");
                    from = left.remove(i);
                    self.gates.push(from);
                }
                // as many as the mission asks, evenly along the way
                let all = std::mem::take(&mut self.gates);
                self.gates = (0..n)
                    .map(|i| all[i * (all.len() - 1) / (n - 1).max(1)])
                    .collect();
                self.gate = 0;
                self.need = n as i32;
            }
            Part::Horde { count, seconds } => {
                self.need = if count > 0 { count } else { seconds };
            }
            Part::Duel => {
                let ladder = spec.goal.ladder.max(0);
                self.need = if ladder > 0 {
                    ladder
                } else {
                    spec.goal.rounds.max(1)
                };
                self.rung = 0;
                self.start_round(w);
            }
            Part::Team => {
                self.need = spec.goal.score;
                let spawns = w.map.spawns.clone();
                for (i, f) in foes.iter().enumerate() {
                    let ally = f.side == "ally";
                    let at = if ally {
                        self.somewhere(me, 2, 6)
                    } else {
                        spawns[(1 + i) % spawns.len()].0
                    };
                    if let Some(k) = self.put_bot(w, f, at) {
                        w.players[k].spawn = (at, w.players[k].aim.yaw);
                    }
                }
            }
            Part::Arena { waves } => {
                self.need = waves;
            }
            Part::Boss => {
                self.need = 1;
                let id = foes.first().map(|f| f.boss.clone()).unwrap_or_default();
                let at = self.somewhere(me, 12, 22);
                if id == "black_knight" {
                    let f = FoeRef {
                        bot: "black_knight".into(),
                        level: 4,
                        weapon: "katana".into(),
                        ..FoeRef::default()
                    };
                    if let Some(k) = self.put_bot(w, &f, at) {
                        self.roles[k] = Role::Knight;
                        w.players[k].health = KNIGHT_HEALTH;
                        self.boss = Some(k);
                    }
                } else {
                    let ltc_id = if id == "section_chief" {
                        "chief"
                    } else {
                        id.as_str()
                    };
                    self.put_foe(w, ltc_id, at);
                    self.boss = Some(self.foes.len() - 1);
                }
            }
            Part::LongJump => {
                self.need = spec.medals.silver;
            }
            Part::Royale => {
                // a hundred bots, one at each spawn but yours, each on no
                // side (team 0 is every other seat's foe), so they fall to
                // each other as well as to you
                let (_, level) = difficulty(&spec.goal.difficulty);
                let spawns = w.map.spawns.clone();
                for (i, &(at, yaw)) in spawns.iter().enumerate().skip(1) {
                    let f = FoeRef {
                        bot: SKINS[i % (SKINS.len() - 1)].into(),
                        level,
                        ..FoeRef::default()
                    };
                    if let Some(k) = self.put_bot(w, &f, at) {
                        let p = &mut w.players[k];
                        p.team = 0;
                        p.aim.yaw = yaw;
                        p.spawn = (at, yaw);
                    }
                }
                self.standing = w.players.len() as i32 - 1;
                self.need = self.standing;
            }
            Part::Salvage => {
                let n = spec.goal.relics.max(1) as usize;
                self.relics = self.spread(n, me).into_iter().map(|p| (p, false)).collect();
                self.need = spec.goal.need;
                for f in &foes {
                    if !f.enemy.is_empty() {
                        for _ in 0..2 {
                            let at = self.somewhere(me, 12, 30);
                            self.put_foe(w, &f.enemy, at);
                        }
                    }
                }
            }
        }
    }

    /// A round of a duel: you whole at your spawn, the rung's bots at the
    /// map's other spawns.
    fn start_round(&mut self, w: &mut World) {
        let spec = self.spec();
        let foes = self.foes_now();
        for k in 1..w.players.len() {
            if self.roles[k] != Role::You {
                w.players[k].dead = crate::world::DOWN_FOR_ROUND;
                w.players[k].health = 0;
            }
        }
        let health = w.players[0].health;
        let arms = w.players[0].arms;
        w.players[0].respawn();
        if spec.kind == "gauntlet" {
            w.players[0].health = health;
        }
        w.players[0].arms = if spec.loadout == "any" {
            Arms::for_round()
        } else {
            arms
        };
        let spawns = w.map.spawns.clone();
        let rung: Vec<FoeRef> = if spec.goal.ladder > 0 {
            foes.get(self.rung).cloned().into_iter().collect()
        } else if spec.kind == "gauntlet" {
            let pick = SKINS[1 + self.rng.below(SKINS.len() as u32 - 1) as usize];
            vec![FoeRef {
                bot: pick.into(),
                level: 3,
                ..FoeRef::default()
            }]
        } else {
            foes
        };
        for (i, f) in rung.iter().enumerate() {
            let at = spawns[(1 + i) % spawns.len()].0;
            self.put_bot(w, f, at);
        }
    }

    fn end(&mut self, cleared: bool, score: i32, why: &'static str) {
        if self.outcome.is_some() {
            return;
        }
        let spec = self.spec();
        let medal = if cleared { medal_for(spec, score) } else { NONE };
        self.outcome = Some(Outcome {
            cleared,
            score,
            medal,
            why,
        });
    }

    /// The score the medals read when it ends now.
    fn score_now(&self, w: &World) -> i32 {
        match self.spec().medals.measure.as_str() {
            "health" => w.players[0].health.max(0),
            "relics" => self.delivered,
            "distance" => self.done,
            "lives" => self.lives,
            _ => self.ticks,
        }
    }

    fn part_done(&mut self, w: &mut World) {
        if self.part + 1 >= self.parts.len() {
            let s = self.score_now(w);
            self.end(true, s, "mission.cleared");
            return;
        }
        // a gauntlet's stage cleared: said, then the next after a count
        self.banner = Some(Banner::StageCleared(self.part as i32 + 1));
        self.over = OVER_TICKS;
        self.next = Next::Part;
        w.rules.frozen = true;
        for k in 1..w.players.len() {
            w.players[k].dead = crate::world::DOWN_FOR_ROUND;
            w.players[k].health = 0;
        }
        self.foes.clear();
        self.orbs.clear();
        self.sync_parts(w);
        if self.repairs > 0 {
            self.repairs -= 1;
            let me = w.players[0].body.origin;
            self.repair = Some(self.somewhere(me, 3, 6));
        }
    }

    /// A round's end shown, the next round or stage set up, and the count
    /// before it; or the count ticking down. Whether play waits this tick.
    fn waiting(&mut self, w: &mut World) -> bool {
        use crate::sound::{Heard, Voice};
        let say = |w: &mut World, voice| {
            w.heard.push(Heard {
                voice,
                at: V3::ZERO,
                by: 0,
            })
        };
        if self.over > 0 {
            self.over -= 1;
            if self.over == 0 {
                self.banner = None;
                match self.next {
                    Next::Round => self.start_round(w),
                    Next::Part => {
                        self.part += 1;
                        self.begin_part(w);
                    }
                }
                self.count = COUNT_TICKS;
            }
            return true;
        }
        if self.count > 0 {
            self.count -= 1;
            if self.count == 0 {
                w.rules.frozen = false;
                say(w, Voice::Go);
            } else if self.count % SECOND == 0 || self.count == COUNT_TICKS - 1 {
                say(w, Voice::Count);
            }
            return true;
        }
        false
    }

    /// One tick, after the world's. `bits` are your held keys this tick.
    pub fn step(&mut self, w: &mut World, bits: u32) {
        if self.outcome.is_some() {
            return;
        }
        if self.waiting(w) {
            return;
        }
        if !self.started && bits & !crate::input::RESET != 0 {
            self.started = true;
        }
        if self.started {
            self.ticks += 1;
        }
        self.part_ticks += 1;
        let spec = self.spec();
        let me = w.players[0].body.origin;
        let chest = me.add(v3(0, ONE * 11 / 10, 0));
        // the time allowed
        if spec.limit() > 0 && self.ticks > spec.limit() * SECOND {
            let s = self.score_now(w);
            if *self.current() == Part::Salvage && self.delivered >= self.need
                || *self.current() == Part::LongJump && self.done >= spec.medals.bronze
            {
                self.end(true, s, "mission.cleared");
            } else {
                self.end(false, s, "mission.out_of_time");
            }
            return;
        }
        // felled, outside a team match (where you stand again) and a duel
        // (where it loses the round)
        let mine_down = !w.players[0].alive();
        let part = self.current().clone();
        if mine_down
            && part != Part::Team
            && part != Part::Duel
            && part != Part::LongJump
            && part != Part::Royale
        {
            self.end(false, 0, "mission.felled");
            return;
        }
        // a repair cell
        if let Some(r) = self.repair {
            if me.sub(r).len_xz() < TAKE_REACH && (me.y - r.y).abs() < fx::int(2) {
                w.players[0].health = (w.players[0].health + REPAIR).min(HEALTH);
                self.repair = None;
            }
        }
        // who felled whom
        let felled = w.felled.clone();
        let seats = w.players.len();
        // a royale: felled, a life gone; with none left it is over, else
        // you stand again where the nearest bot is farthest
        if part == Part::Royale && mine_down && self.alive_was[0] {
            self.lives -= 1;
            if self.lives <= 0 {
                self.alive_was[0] = false;
                self.end(false, 0, "mission.out_of_lives");
                return;
            }
            let at = self.safest(w);
            let yaw = crate::geom::yaw_of(v3(-at.x, 0, -at.z));
            w.players[0].spawn = (at, yaw);
        }
        // seats standing again (a team match): their weapons back
        for k in 0..seats {
            let alive = w.players[k].alive();
            let again = part == Part::Team || part == Part::Royale && k == 0;
            if alive && !self.alive_was.get(k).copied().unwrap_or(true) && again {
                if k == 0 {
                    if spec.loadout == "any" {
                        w.players[0].arms = Arms::for_round();
                    }
                } else {
                    w.players[k].arms = Arms::for_round();
                }
            }
            if k < self.alive_was.len() {
                self.alive_was[k] = alive;
            }
        }
        // the foes' hit bodies: what they took is the foes' damage
        for f in self.foes.iter_mut().filter(|f| !f.down) {
            let (first, n) = f.parts;
            for d in w.dummies.iter_mut().skip(first).take(n) {
                let took = PART_FULL - d.health;
                if took > 0 {
                    f.hp -= took;
                    f.flash = crate::world::FLASH;
                    d.health = PART_FULL;
                }
            }
        }
        // orbs fly; they hurt you, or a parried one hurts foes
        let map = std::rc::Rc::clone(&w.map);
        let hit_pct = ltc::data().balance.orb_hit_pct;
        let swinging = w.players[0].swing.is_some();
        let aim = w.players[0].look();
        let eye = w.players[0].eye_now();
        let parry_reach = ltc::cm(ltc::data().balance.parry_reach_cm);
        let mut struck = 0;
        let mut from = me;
        let mut i = 0;
        while i < self.orbs.len() {
            let o = &mut self.orbs[i];
            let alive = o.step(&map);
            let mut gone = !alive;
            if !gone && o.parried {
                if let Some(f) = self
                    .foes
                    .iter_mut()
                    .find(|f| !f.down && ltc::within(f.at, o.pos, f.radius() + o.radius))
                {
                    f.hp -= ltc::data().balance.parry_damage;
                    f.flash = crate::world::FLASH;
                    gone = true;
                }
            } else if !gone
                && o.gold
                && swinging
                && ltc::within(o.pos, eye, parry_reach)
                && o.pos.sub(eye).dot(aim) > 0
            {
                // a gold orb struck back by a melee swing (CM-Q9)
                o.parried = true;
                o.dir = aim;
                o.speed = ltc::per_tick(ltc::data().balance.parry_speed_cms);
                o.curl = 0;
                o.accel = 0;
            } else if !gone && w.players[0].alive() && o.touches(me, hit_pct) {
                struck += o.damage;
                from = o.prev;
                gone = true;
            }
            if gone {
                self.orbs.swap_remove(i);
            } else {
                i += 1;
            }
        }
        // the foes move and fire
        let mut summons = Vec::new();
        let mut fired = Vec::new();
        let spots = std::mem::take(&mut self.spots);
        for (i, f) in self.foes.iter_mut().enumerate() {
            if f.down {
                continue;
            }
            let did = f.step(i, chest, me, &spots, &mut self.rng);
            if did.touched > 0 {
                struck += did.touched;
                from = f.at;
            }
            fired.extend(did.orbs);
            for s in did.summons {
                summons.push((s, f.at));
            }
        }
        self.spots = spots;
        let room = ltc::ORBS_MOST.saturating_sub(self.orbs.len());
        self.orbs.extend(fired.into_iter().take(room));
        if struck > 0 && w.players[0].alive() {
            w.strike(0, struck, from);
        }
        for (id, at) in summons {
            let near = self.somewhere(at, 3, 9);
            self.put_foe(w, &id, near);
        }
        // a foe at no health is down
        let mut changed = false;
        for f in self.foes.iter_mut() {
            if !f.down && f.hp <= 0 {
                f.down = true;
                changed = true;
            }
        }
        if changed {
            self.sync_parts(w);
        } else {
            // the hit bodies follow their foes
            for f in &self.foes {
                let (first, n) = f.parts;
                let r = f.radius();
                let tall = (r * 2).max(ONE * 8 / 10);
                let feet = f.at.sub(v3(0, tall / 2, 0));
                let o = r / 2;
                let offsets = [V3::ZERO, v3(o, 0, 0), v3(-o, 0, 0), v3(0, 0, o), v3(0, 0, -o)];
                for (j, d) in w.dummies.iter_mut().skip(first).take(n).enumerate() {
                    d.feet = feet.add(offsets[j]);
                }
            }
        }
        if self.breather > 0 {
            self.breather -= 1;
            return;
        }
        match part {
            Part::Targets => {
                if spec.goal.moving {
                    let t = self.part_ticks % SWAY_TICKS;
                    let a = fx::TURN / SWAY_TICKS * t;
                    for (d, base) in w.dummies.iter_mut().zip(&self.sway) {
                        if d.down == 0 {
                            d.feet = base.add(v3(fx::mul(fx::int(SWAY), fx::sin(a)), 0, 0));
                        }
                    }
                }
                self.done = w.dummies.iter().filter(|d| d.down > 0).count() as i32;
                if self.done >= self.need {
                    self.part_done(w);
                }
            }
            Part::Course => {
                if let Some(g) = self.gates.get(self.gate) {
                    // within reach of its middle, whichever way is up (the Cubo)
                    let d = me.sub(*g);
                    if d.len() < GATE_REACH + GATE_UP / 2
                        || (d.len_xz() < GATE_REACH && d.y > -ONE && d.y < GATE_UP)
                    {
                        self.gate += 1;
                        self.done = self.gate as i32;
                    }
                }
                if self.gate >= self.gates.len() {
                    self.part_done(w);
                }
            }
            Part::Horde { count, seconds } => {
                self.horde(w, &felled);
                if count > 0 {
                    if self.done >= count {
                        self.part_done(w);
                    }
                } else {
                    self.done = self.part_ticks / SECOND;
                    if self.done >= seconds {
                        self.part_done(w);
                    }
                }
            }
            Part::Duel => {
                let foes_up = (1..seats).any(|k| self.roles[k] == Role::Bot && w.players[k].alive());
                if mine_down {
                    self.lost += 1;
                    if self.lost >= spec.goal.rounds.max(1) {
                        self.end(false, 0, "mission.out_of_rounds");
                        return;
                    }
                    // who felled you takes the round; it is said, then the
                    // next round after its count
                    let by = felled
                        .iter()
                        .find(|&&(by, k)| k == 0 && by != 0 && by < seats)
                        .map(|&(by, _)| w.players[by].skin);
                    self.banner = Some(Banner::RoundLost(by));
                    self.over = OVER_TICKS;
                    self.next = Next::Round;
                    self.orbs.clear();
                    w.rules.frozen = true;
                } else if !foes_up {
                    self.won += 1;
                    if spec.goal.ladder > 0 {
                        self.rung += 1;
                    }
                    self.done = if spec.goal.ladder > 0 {
                        self.rung as i32
                    } else {
                        self.won
                    };
                    if self.done >= self.need {
                        self.part_done(w);
                    } else {
                        self.banner = Some(Banner::RoundWon(self.won + self.lost));
                        self.over = OVER_TICKS;
                        self.next = Next::Round;
                        w.rules.frozen = true;
                    }
                }
            }
            Part::Royale => {
                for &(by, k) in &felled {
                    if by == 0 && k != 0 && k < seats {
                        self.done += 1;
                    }
                }
                self.standing = (1..seats).filter(|&k| w.players[k].alive()).count() as i32;
                if self.standing == 0 {
                    self.part_done(w);
                }
            }
            Part::Team => {
                for &(by, k) in &felled {
                    if k >= seats || by >= seats || by == k {
                        continue;
                    }
                    let (a, b) = (w.players[by].team, w.players[k].team);
                    if a == 1 && b == 2 {
                        self.score.0 += 1;
                    } else if a == 2 && b == 1 {
                        self.score.1 += 1;
                    }
                }
                self.done = self.score.0;
                if self.score.0 >= self.need {
                    self.part_done(w);
                } else if self.score.1 >= self.need {
                    self.end(false, 0, "mission.team_lost");
                }
            }
            Part::Arena { waves } => {
                if self.foes.iter().all(|f| f.down) {
                    if self.wave >= waves {
                        self.part_done(w);
                        return;
                    }
                    self.wave += 1;
                    self.done = self.wave - 1;
                    self.foes.clear();
                    let ids: Vec<String> = self
                        .foes_now()
                        .iter()
                        .map(|f| {
                            if f.enemy.is_empty() {
                                "sentinel".to_string()
                            } else {
                                f.enemy.clone()
                            }
                        })
                        .collect();
                    let ids = if ids.iter().all(|i| i == "sentinel") && spec.kind == "gauntlet" {
                        vec!["sentinel".to_string(), "drifter".to_string(), "moth".to_string()]
                    } else {
                        ids
                    };
                    let n = (1 + self.wave).min(6) as usize;
                    for j in 0..n {
                        let id = ids[(self.wave as usize + j) % ids.len()].clone();
                        let at = self.somewhere(me, 8, 18);
                        self.put_foe(w, &id, at);
                    }
                    if self.wave > 1 {
                        self.breather = BREATHER / 2;
                    }
                }
            }
            Part::Boss => {
                let down = match self.boss {
                    Some(k)
                        if self.parts[self.part]
                            .1
                            .first()
                            .is_some_and(|f| f.boss == "black_knight") =>
                    {
                        let p = &mut w.players[k];
                        // past half health it takes up the trabuco too
                        if p.health * 100 / KNIGHT_HEALTH <= KNIGHT_RAGE_PCT && !p.arms.owns(Weapon::Trabuco)
                        {
                            p.arms.pick_up(Weapon::Trabuco);
                        }
                        !p.alive()
                    }
                    Some(k) => self.foes.get(k).is_none_or(|f| f.down),
                    None => true,
                };
                if down {
                    self.done = 1;
                    self.part_done(w);
                }
            }
            Part::LongJump => {
                let Some(j) = self.jump else { return };
                let p = &w.players[0];
                let at = p.body.origin;
                let m = |v: Fx| v / ONE;
                if at.z > j.lip.z {
                    // how far along the course, while still on it
                    if at.y > j.field_y + ONE * 2 && !mine_down {
                        self.done = self.done.max(m(j.start_z - at.z).clamp(0, j.length));
                    }
                } else if p.body.on_ground && at.y < j.lip.y - fx::int(5) {
                    // landed on the field: the course and the jump past its lip
                    let jumped = m(j.lip.z - at.z);
                    self.landed = Some(jumped);
                    self.done = j.length + jumped;
                    let cleared = self.done >= spec.medals.bronze;
                    self.end(cleared, self.done, "mission.landed");
                    return;
                }
                if mine_down || at.y < j.field_y - ONE * 2 {
                    let cleared = self.done >= spec.medals.bronze;
                    let s = self.done;
                    self.end(cleared, s, "mission.fell");
                }
            }
            Part::Salvage => {
                self.horde(w, &felled);
                for (p, taken) in self.relics.iter_mut() {
                    if !*taken && me.sub(*p).len_xz() < TAKE_REACH && (me.y - p.y).abs() < fx::int(2) {
                        *taken = true;
                        self.carried += 1;
                    }
                }
                let on_lift = me.sub(self.spawn).len_xz() < LIFT_REACH;
                if on_lift && self.carried > 0 {
                    self.delivered += self.carried;
                    self.carried = 0;
                }
                self.done = self.delivered;
                if self.delivered >= self.relics.len() as i32 {
                    self.part_done(w);
                } else if on_lift && self.delivered >= self.need && self.carried == 0 {
                    self.lift_wait += 1;
                    if self.lift_wait >= LIFT_TICKS {
                        self.part_done(w);
                    }
                } else {
                    self.lift_wait = 0;
                }
            }
        }
    }

    /// Zombies come in: counted when you fell them, more as time goes.
    fn horde(&mut self, w: &mut World, felled: &[(usize, usize)]) {
        let seats = w.players.len();
        for &(by, k) in felled {
            if by == 0 && k < seats && self.roles[k] == Role::Zombie {
                if let Part::Horde { count, .. } = self.current() {
                    if *count > 0 {
                        self.done += 1;
                    }
                }
            }
        }
        let out = (1..seats)
            .filter(|&k| self.roles[k] == Role::Zombie && w.players[k].alive())
            .count() as i32;
        self.zombies_out = out;
        let secs = self.part_ticks / SECOND;
        let foes = self.foes_now();
        let guard = foes.iter().any(|f| f.waves == "guard");
        let (cap, every) = if guard {
            (3 + self.relics.len() as i32 / 3, SECOND * 3)
        } else if matches!(self.current(), Part::Horde { .. }) && self.spec().kind == "gauntlet" {
            (6, SECOND)
        } else {
            ((2 + secs / 8).min(ZOMBIES_MOST), SECOND * 3 / 4)
        };
        if out >= cap || self.part_ticks % every != 0 {
            return;
        }
        let kinds: Vec<&str> = foes
            .iter()
            .filter(|f| !f.zombie.is_empty() && secs >= f.from)
            .map(|f| f.zombie.as_str())
            .collect();
        let kinds = if kinds.is_empty() {
            vec!["shambler", "runner"]
        } else {
            kinds
        };
        let kind = kinds[self.rng.below(kinds.len() as u32) as usize].to_string();
        let me = w.players[0].body.origin;
        let at = if guard && !self.relics.is_empty() {
            let r = self.relics[self.rng.below(self.relics.len() as u32) as usize].0;
            self.somewhere(r, 2, 6)
        } else {
            self.somewhere(me, ZOMBIE_FAR, ZOMBIE_FAR * 3)
        };
        self.put_zombie(w, &kind, at);
    }

    /// The boss's health, percent, if there is one.
    pub fn boss_pct(&self, w: &World) -> Option<i32> {
        let k = self.boss?;
        if *self.current() != Part::Boss {
            return None;
        }
        let knight = self.parts[self.part]
            .1
            .first()
            .is_some_and(|f| f.boss == "black_knight");
        Some(if knight {
            w.players
                .get(k)
                .map_or(0, |p| p.health.max(0) * 100 / KNIGHT_HEALTH)
        } else {
            self.foes.get(k).map_or(0, |f| f.hp.max(0) * 100 / f.most.max(1))
        })
    }

    /// The boss's name, for the bar.
    pub fn boss_name(&self) -> Option<&'static str> {
        let k = self.boss?;
        let knight = self.parts[self.part]
            .1
            .first()
            .is_some_and(|f| f.boss == "black_knight");
        if knight {
            return None;
        }
        self.foes.get(k).map(|f| f.spec().name.as_str())
    }

    /// The mission's things in the world: gates, relics, the lift, the
    /// repair cell, the foes and their orbs.
    pub fn draw(&self, alpha: Fx, look: &Look, out: &mut Shapes) {
        let pal = &look.palette;
        let ink = look.ink();
        let cyan = rgb([127, 211, 255]);
        let gold = rgb([230, 187, 76]);
        let moment = (self.part_ticks).wrapping_mul(ONE).wrapping_add(alpha);
        for (i, g) in self.gates.iter().enumerate().skip(self.gate) {
            let next = i == self.gate;
            let at = Affine::translate(g.add(v3(0, ONE * 15 / 10, 0))).then(&Affine::scale(v3(
                GATE_REACH,
                ONE * 3 / 2,
                GATE_REACH,
            )));
            out.glow(
                Mesh::Tube,
                at,
                if next { cyan } else { rgb(pal.steel) },
                if next { ONE } else { ONE / 4 },
            );
        }
        for (p, taken) in &self.relics {
            if *taken {
                continue;
            }
            let at = Affine::translate(p.add(v3(0, ONE * 8 / 10, 0)))
                .then(&Affine::rot_y(fx::mul(moment, fx::deg(3))))
                .then(&Affine::scale(v3(ONE * 3 / 10, ONE * 4 / 10, ONE * 3 / 10)));
            out.inked(Mesh::Octa, at, gold, ink);
            let glow =
                Affine::translate(p.add(v3(0, ONE / 20, 0))).then(&Affine::scale(v3(ONE, ONE / 10, ONE)));
            out.glow(Mesh::Sphere, glow, gold, ONE / 2);
        }
        if *self.current() == Part::Salvage {
            let open = self.delivered >= self.need;
            let at = Affine::translate(self.spawn.add(v3(0, ONE / 20, 0))).then(&Affine::scale(v3(
                LIFT_REACH,
                ONE / 10,
                LIFT_REACH,
            )));
            out.glow(
                Mesh::Tube,
                at,
                if open { cyan } else { rgb([224, 65, 43]) },
                ONE * 3 / 4,
            );
        }
        // a long jump's field: a line across it every 10 m past the lip,
        // the gold mark in gold, and the lip itself in cyan
        if let Some(j) = self.jump {
            let past_gold = self.spec().medals.gold - j.length;
            for d in (0..=MARKS_PAST_LIP).step_by(MARK_EVERY as usize) {
                let (colour, strength) = if d == 0 {
                    (cyan, ONE)
                } else if d == past_gold / MARK_EVERY * MARK_EVERY {
                    (gold, ONE)
                } else {
                    (rgb(pal.steel), ONE / 3)
                };
                let y = if d == 0 { j.lip.y } else { j.field_y };
                let at = Affine::translate(v3(0, y + ONE / 20, j.lip.z - fx::int(d)))
                    .then(&Affine::scale(v3(fx::int(MARK_WIDE), ONE / 20, ONE * 3 / 10)));
                out.glow(Mesh::Cube, at, colour, strength);
            }
        }
        if let Some(r) = self.repair {
            let at = Affine::translate(r.add(v3(0, ONE * 8 / 10, 0)))
                .then(&Affine::rot_y(fx::mul(moment, fx::deg(2))))
                .then(&Affine::scale(v3(ONE * 35 / 100, ONE * 35 / 100, ONE * 35 / 100)));
            out.inked(Mesh::Cube, at, rgb([120, 230, 160]), ink);
        }
        for f in self.foes.iter().filter(|f| !f.down) {
            let at = f.prev.lerp(f.at, alpha);
            let r = f.radius();
            let body = if f.flash > 0 {
                rgb(pal.flash)
            } else {
                rgb([200, 208, 218])
            };
            let ball = Affine::translate(at)
                .then(&Affine::rot_y(fx::mul(moment, fx::deg(1))))
                .then(&Affine::scale(v3(r * 2, r * 2, r * 2)));
            out.inked(Mesh::Facet, ball, body, ink);
            let s = f.spec();
            let eye_rgb = rgb(s
                .attacks
                .first()
                .or_else(|| s.phases.first().and_then(|p| p.attacks.first()))
                .and_then(|a| ltc::data().patterns.get(&a.pattern))
                .and_then(|p| p.volleys.first())
                .map_or([255, 46, 136], |v| ltc::data().colour(&v.colour)));
            let eye = Affine::translate(at).then(&Affine::scale(v3(r, r, r)));
            out.glow(Mesh::Sphere, eye, eye_rgb, ONE / 2 + f.winding() / 2);
            if f.winding() > 0 {
                let w = r * 2 + fx::mul(r, f.winding());
                let halo = Affine::translate(at).then(&Affine::scale(v3(w, ONE / 20, w)));
                out.glow(Mesh::Tube, halo, eye_rgb, f.winding());
            }
        }
        for o in &self.orbs {
            let at = o.prev.lerp(o.pos, alpha);
            let d = o.radius * 2;
            let c = if o.parried { cyan } else { rgb(o.rgb) };
            out.glow(
                Mesh::Sphere,
                Affine::translate(at).then(&Affine::scale(v3(d, d, d))),
                c,
                ONE,
            );
            let core = d / 2;
            out.solid(
                Mesh::Sphere,
                Affine::translate(at).then(&Affine::scale(v3(core, core, core))),
                rgb([255, 250, 240]),
                ONE,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn progress(pairs: &[(&str, u8)]) -> Progress {
        pairs
            .iter()
            .map(|(id, m)| (id.to_string(), Best { medal: *m, score: 0 }))
            .collect()
    }

    #[test]
    fn the_pack_holds_fifty_missions_and_every_requirement_points_to_a_lower_tier() {
        let p = pack();
        assert_eq!(p.missions.len(), 58);
        for m in &p.missions {
            for r in &m.requires {
                let q = index(&r.mission).unwrap_or_else(|| panic!("{} requires {}", m.id, r.mission));
                assert!(p.missions[q].tier() < m.tier(), "{} requires {}", m.id, r.mission);
            }
            assert!(
                MAPS.contains(&m.map.as_str())
                    || crate::courses::is_course(&m.map)
                    || crate::royale::is_royale(&m.map),
                "{} on {}",
                m.id,
                m.map
            );
            assert!(KINDS.contains(&m.kind.as_str()), "{} is {}", m.id, m.kind);
            assert!(
                m.loadout == "any" || m.loadout == "none" || weapon(&m.loadout).is_some(),
                "{} carries {}",
                m.id,
                m.loadout
            );
        }
    }

    #[test]
    fn a_mission_opens_when_every_medal_it_asks_for_is_earned() {
        let m = &pack().missions[index("choomstar").unwrap()];
        assert!(!open(m, &progress(&[])));
        let mut have = progress(&[
            ("interdictor", BRONZE),
            ("great_horned", GOLD),
            ("black_knight", GOLD),
            ("section_chief", GOLD),
            ("gauntlet_one", GOLD),
        ]);
        assert!(!open(m, &have), "gauntlet_two missing");
        have.insert(
            "gauntlet_two".into(),
            Best {
                medal: SILVER,
                score: 0,
            },
        );
        assert!(!open(m, &have), "silver is not gold");
        have.insert(
            "gauntlet_two".into(),
            Best {
                medal: GOLD,
                score: 0,
            },
        );
        assert!(open(m, &have));
        assert_eq!(
            pack().missions.iter().filter(|m| open(m, &progress(&[]))).count(),
            5
        );
    }

    #[test]
    fn time_medals_go_by_the_seconds_and_health_medals_by_what_is_left() {
        // first_light: bronze 90 s, silver 45, gold 28
        let m = &pack().missions[0];
        assert_eq!(medal_for(m, 28 * SECOND), GOLD);
        assert_eq!(medal_for(m, 28 * SECOND + 1), SILVER);
        assert_eq!(medal_for(m, 45 * SECOND), SILVER);
        assert_eq!(medal_for(m, 46 * SECOND), BRONZE);
        // doors_open: health, silver 40, gold 75
        let h = &pack().missions[index("doors_open").unwrap()];
        assert_eq!(medal_for(h, 75), GOLD);
        assert_eq!(medal_for(h, 74), SILVER);
        assert_eq!(medal_for(h, 39), BRONZE);
    }

    #[test]
    fn a_best_keeps_the_better_score_and_the_better_medal() {
        let m = &pack().missions[0];
        let mut p = Progress::new();
        record(&mut p, m, 40 * SECOND);
        record(&mut p, m, 80 * SECOND);
        assert_eq!(
            p["first_light"],
            Best {
                medal: SILVER,
                score: 40 * SECOND
            }
        );
        record(&mut p, m, 20 * SECOND);
        assert_eq!(
            p["first_light"],
            Best {
                medal: GOLD,
                score: 20 * SECOND
            }
        );
    }

    #[test]
    fn the_chart_puts_the_finale_in_the_middle_and_launch_on_the_outer_ring() {
        assert_eq!(chart_at(index("choomstar").unwrap()), (500, 500));
        // first_light: targets spoke (straight up), tier 0 at 470
        assert_eq!(chart_at(0), (500, 30));
        assert!(route_from(0).is_none());
        // bell_tower (targets) comes from first_light, its own kind
        assert_eq!(route_from(index("bell_tower").unwrap()), Some(0));
    }

    /// A mission past its count, ready to play.
    fn begin(k: usize, skin: u8, lifelike: bool) -> (Run, World) {
        let (mut run, mut w) = Run::start(k, skin, lifelike);
        run.count = 0;
        w.rules.frozen = false;
        (run, w)
    }

    fn play(k: usize, ticks: i32) -> (Run, World) {
        let (mut run, mut w) = begin(k, 0, false);
        for _ in 0..ticks {
            w.step(&[crate::input::Input::default()]);
            run.step(&mut w, 0);
            if run.outcome.is_some() {
                break;
            }
        }
        (run, w)
    }

    #[test]
    fn a_targets_mission_places_its_targets_on_the_floor_and_breaking_them_all_clears_it() {
        let (mut run, mut w) = begin(0, 0, false);
        assert_eq!(w.dummies.len(), 10);
        assert_eq!(run.need, 10);
        for d in &w.dummies {
            assert!(run.spots.contains(&d.feet));
        }
        w.dummies.iter_mut().for_each(|d| d.down = 1);
        run.step(&mut w, crate::input::FORWARD);
        assert_eq!(run.outcome.map(|o| o.cleared), Some(true));
        // one tick counted: gold
        assert_eq!(run.outcome.map(|o| o.medal), Some(GOLD));
    }

    #[test]
    fn the_clock_starts_on_the_first_move_and_the_time_allowed_ends_it() {
        let (run, _) = play(0, 200);
        assert_eq!(run.ticks, 0, "nobody moved");
        let (mut run, mut w) = begin(0, 0, false);
        for _ in 0..(90 * SECOND + 1) {
            run.step(&mut w, crate::input::FORWARD);
        }
        assert_eq!(run.outcome.map(|o| o.why), Some("mission.out_of_time"));
    }

    #[test]
    fn a_course_counts_its_gates_in_order() {
        let (mut run, mut w) = begin(index("plaza_run").unwrap(), 0, false);
        assert_eq!(run.gates.len(), 6);
        let second = run.gates[1];
        w.players[0].body.origin = second;
        run.step(&mut w, crate::input::FORWARD);
        assert_eq!(run.gate, 0, "the second gate before the first counts nothing");
        let gates = run.gates.clone();
        for g in &gates[..gates.len() - 1] {
            w.players[0].body.origin = *g;
            run.step(&mut w, crate::input::FORWARD);
        }
        assert_eq!(run.gate, 5);
        assert!(run.outcome.is_none(), "five of six gates is not the end");
        w.players[0].body.origin = gates[5];
        run.step(&mut w, crate::input::FORWARD);
        assert_eq!(run.outcome.map(|o| o.cleared), Some(true));
    }

    #[test]
    fn the_toboggan_course_runs_from_the_platform_down_the_slide_to_the_arena() {
        let (run, w) = begin(index("toboggan").unwrap(), 0, false);
        assert_eq!(run.gates.len(), 8);
        let g = &run.gates;
        // the platform stands 90 m up at z 240 to 252; the arena floor is
        // at 0, z 20 down to -160
        assert!(g[0].z > fx::int(90) && g[0].y > fx::int(20), "{:?}", g[0]);
        assert!(g[7].y < fx::int(2) && g[7].z < 0, "{:?}", g[7]);
        let on_slide = g.iter().filter(|p| p.z > fx::int(20) && p.y > fx::int(2)).count();
        assert!(on_slide >= 4, "{on_slide} gates on the slide");
        assert!(w.pickups.is_empty(), "a course carries no pickups");
    }

    #[test]
    fn a_mission_opens_on_a_three_second_count_and_nobody_moves_in_it() {
        let (mut run, mut w) = Run::start(index("sparring").unwrap(), 0, false);
        assert!(w.rules.frozen);
        let bot = (1..w.players.len()).find(|&k| run.roles[k] == Role::Bot).unwrap();
        let at = w.players[bot].body.origin;
        for _ in 0..(COUNT_TICKS - 1) {
            w.step(&[crate::input::Input::default()]);
            run.step(&mut w, crate::input::FORWARD);
        }
        assert_eq!(w.players[bot].body.origin.len_xz(), at.len_xz(), "the bot waits");
        assert_eq!(run.ticks, 0, "the clock waits");
        run.step(&mut w, crate::input::FORWARD);
        assert!(!w.rules.frozen);
        assert_eq!(run.count, 0);
    }

    #[test]
    fn a_round_won_is_said_for_four_seconds_then_the_next_round_counts_in() {
        let (mut run, mut w) = begin(index("sparring").unwrap(), 0, false);
        let bot = (1..w.players.len()).find(|&k| run.roles[k] == Role::Bot).unwrap();
        let h = w.players[bot].health;
        w.hit(0, bot, h);
        run.step(&mut w, crate::input::FORWARD);
        assert_eq!(run.banner, Some(Banner::RoundWon(1)));
        assert!(w.rules.frozen);
        for _ in 0..(OVER_TICKS - 1) {
            run.step(&mut w, crate::input::FORWARD);
        }
        assert!(!w.players[bot].alive(), "no new round while it is said");
        run.step(&mut w, crate::input::FORWARD);
        assert_eq!(run.banner, None);
        assert_eq!(run.count, COUNT_TICKS);
        assert!((1..w.players.len()).any(|k| run.roles[k] == Role::Bot && w.players[k].alive()));
    }

    #[test]
    fn a_round_lost_names_the_bot_that_took_it() {
        let (mut run, mut w) = begin(index("sparring").unwrap(), 0, false);
        let bot = (1..w.players.len()).find(|&k| run.roles[k] == Role::Bot).unwrap();
        w.hit(bot, 0, HEALTH);
        run.step(&mut w, crate::input::FORWARD);
        assert_eq!(run.banner, Some(Banner::RoundLost(Some(skin("hidalgo")))));
        assert_eq!(run.lost, 1);
    }

    #[test]
    fn a_long_jump_scores_the_course_gone_and_the_jump_past_its_lip() {
        // launch_rail: the lip 176 m along; bronze 90, silver 176, gold 246
        let k = index("launch_rail").unwrap();
        let (mut run, mut w) = begin(k, 0, false);
        let j = run.jump.unwrap();
        assert_eq!(j.length, 176);
        assert_eq!(w.map.name, "course_rail");
        // 100 m along the course, still on it
        w.players[0].body.origin = v3(0, j.lip.y + fx::int(20), j.start_z - fx::int(100));
        run.step(&mut w, crate::input::FORWARD);
        assert_eq!(run.done, 100);
        assert!(run.outcome.is_none());
        // landed on the field 75 m past the lip: 176 + 75 = 251, gold
        w.players[0].body.origin = v3(0, j.field_y, j.lip.z - fx::int(75));
        w.players[0].body.on_ground = true;
        run.step(&mut w, crate::input::FORWARD);
        let o = run.outcome.unwrap();
        assert_eq!((o.cleared, o.score, o.medal), (true, 251, GOLD));
        assert_eq!(run.landed, Some(75));
    }

    #[test]
    fn a_long_jump_reaching_the_lip_with_no_jump_past_it_earns_silver_at_most() {
        let k = index("launch_rail").unwrap();
        let (mut run, mut w) = begin(k, 0, false);
        let j = run.jump.unwrap();
        // landed at the lip's foot
        w.players[0].body.origin = v3(0, j.field_y, j.lip.z);
        w.players[0].body.on_ground = true;
        run.step(&mut w, crate::input::FORWARD);
        assert_eq!(run.outcome.map(|o| (o.score, o.medal)), Some((176, SILVER)));
    }

    #[test]
    fn falling_off_a_long_jump_course_scores_how_far_you_got() {
        let k = index("launch_rail").unwrap();
        let (mut run, mut w) = begin(k, 0, false);
        let j = run.jump.unwrap();
        w.players[0].body.origin = v3(0, j.lip.y, j.start_z - fx::int(60));
        run.step(&mut w, crate::input::FORWARD);
        // off the side, below the field's height
        w.players[0].body.origin = v3(fx::int(30), j.field_y - fx::int(5), j.start_z - fx::int(70));
        run.step(&mut w, crate::input::FORWARD);
        let o = run.outcome.unwrap();
        assert_eq!((o.cleared, o.score, o.why), (false, 60, "mission.fell"));
    }

    #[test]
    fn a_horde_sends_zombies_on_their_own_side_at_a_shamblers_pace() {
        let (run, w) = play(index("doors_open").unwrap(), 4 * SECOND);
        let z: Vec<&Player> = (1..w.players.len())
            .filter(|&k| run.roles[k] == Role::Zombie)
            .map(|k| &w.players[k])
            .collect();
        assert!(!z.is_empty());
        for p in z {
            assert_eq!(p.team, 2);
            assert_eq!(p.run_pct, 45);
            assert!(p.arms.owns(Weapon::Punal));
        }
    }

    #[test]
    fn felled_in_a_horde_ends_it_and_no_medal_is_given() {
        let (mut run, mut w) = begin(index("doors_open").unwrap(), 0, false);
        w.strike(0, HEALTH, V3::ZERO);
        run.step(&mut w, 0);
        let o = run.outcome.unwrap();
        assert!(!o.cleared);
        assert_eq!(o.medal, NONE);
        assert_eq!(o.why, "mission.felled");
    }

    #[test]
    fn an_arena_raises_long_time_comins_foes_in_waves_and_they_fire() {
        let (run, _) = play(index("sentinel_row").unwrap(), 8 * SECOND);
        assert!(run.wave >= 1);
        assert!(!run.foes.is_empty());
        assert!(!run.orbs.is_empty() || run.outcome.is_some());
    }

    #[test]
    fn a_shot_foe_loses_what_its_hit_body_took() {
        let (mut run, mut w) = play(index("sentinel_row").unwrap(), 2);
        let (first, _) = run.foes[0].parts;
        let hp = run.foes[0].hp;
        w.dummies[first].health -= 30;
        run.step(&mut w, 0);
        assert_eq!(run.foes[0].hp, hp - 30);
        assert_eq!(w.dummies[first].health, PART_FULL);
    }

    #[test]
    fn salvage_counts_relics_brought_to_the_lift() {
        let (mut run, mut w) = begin(index("salvage_puerto").unwrap(), 0, false);
        assert_eq!(run.relics.len(), 8);
        for (p, _) in run.relics.clone().iter().take(5) {
            w.players[0].body.origin = *p;
            run.step(&mut w, crate::input::FORWARD);
        }
        assert_eq!(run.carried, 5);
        w.players[0].body.origin = run.spawn;
        run.step(&mut w, crate::input::FORWARD);
        assert_eq!(run.delivered, 5);
        // the lift is open: standing on it empty-handed takes it
        for _ in 0..LIFT_TICKS {
            run.step(&mut w, crate::input::FORWARD);
        }
        let o = run.outcome.unwrap();
        assert!(o.cleared);
        assert_eq!(o.score, 5);
        assert_eq!(o.medal, BRONZE);
    }

    #[test]
    fn a_team_match_counts_the_sides_and_allies_do_not_hurt_you() {
        let (run, w) = begin(index("two_by_two").unwrap(), 0, false);
        assert_eq!(run.need, 10);
        let ally = (1..w.players.len())
            .find(|&k| run.roles[k] == Role::Ally)
            .unwrap();
        assert_eq!(w.players[ally].team, w.players[0].team);
        assert!(!w.rules.stay_down);
    }

    /// CM-5: a player who fells every foe the moment it stands and walks
    /// straight to each gate and relic clears every mission inside its
    /// time, so no goal is out of reach of the rules.
    fn royale(id: &str) -> (Run, World) {
        begin(index(id).unwrap(), 0, false)
    }

    #[test]
    fn a_royale_seats_a_hundred_bots_on_no_side_and_gives_you_the_difficultys_lives() {
        let (run, w) = royale("scrap_crown");
        assert_eq!(w.players.len(), 101);
        assert_eq!(run.standing, 100);
        assert_eq!(run.lives, 5, "easy: five lives");
        assert_eq!(royale("last_light").0.lives, 1, "brutal: one");
        assert_eq!(royale("hub_riot").0.lives, 2, "hard: two");
        assert!(w.players[1..]
            .iter()
            .all(|p| p.team == 0 && p.pilot.is_some() && p.alive()));
        assert!(w.map.name.starts_with("royale_"));
        assert!(w.pickups.len() >= 30, "weapons lie about");
    }

    #[test]
    fn royale_bots_fell_each_other_and_a_felled_bot_stays_down() {
        let (mut run, mut w) = royale("scrap_crown");
        // you stand in the corner, out of it, whole
        let mut gone_by_bots = 0;
        for _ in 0..40 * SECOND {
            w.players[0].health = HEALTH;
            w.step(&[crate::input::Input::default()]);
            gone_by_bots += w.felled.iter().filter(|&&(by, k)| by != 0 && k != 0).count();
            run.step(&mut w, 0);
        }
        assert!(gone_by_bots >= 5, "only {gone_by_bots} felled by bots");
        assert_eq!(run.standing, 100 - gone_by_bots as i32);
        assert_eq!(run.done, 0, "none felled by you");
        let down = (1..w.players.len()).filter(|&k| !w.players[k].alive()).count();
        assert_eq!(down, gone_by_bots, "nobody stood again");
    }

    #[test]
    fn a_fall_in_a_royale_costs_a_life_and_you_stand_again_until_none_are_left() {
        let (mut run, mut w) = royale("hub_riot");
        let fell_at = w.players[0].body.origin;
        w.hit(1, 0, HEALTH * 2);
        run.step(&mut w, 0);
        assert_eq!(run.lives, 1);
        assert!(run.outcome.is_none());
        assert_ne!(w.players[0].spawn.0, fell_at, "somewhere else");
        for _ in 0..crate::world::RESPAWN + 1 {
            w.step(&[crate::input::Input::default()]);
            run.step(&mut w, 0);
        }
        assert!(w.players[0].alive(), "stood again");
        assert!(w.players[0].arms.owns(Weapon::Pistol), "with the pistol back");
        w.hit(1, 0, HEALTH * 2);
        run.step(&mut w, 0);
        let o = run.outcome.expect("over");
        assert!(!o.cleared);
        assert_eq!(o.why, "mission.out_of_lives");
    }

    #[test]
    fn the_last_one_standing_clears_a_royale_and_the_medal_is_the_lives_kept() {
        let (mut run, mut w) = royale("scrap_crown");
        w.hit(1, 0, HEALTH * 2);
        run.step(&mut w, 0);
        for k in 1..w.players.len() {
            w.hit(0, k, HEALTH * 9);
        }
        run.step(&mut w, 0);
        let o = run.outcome.expect("over");
        assert!(o.cleared);
        assert_eq!(o.score, 4);
        assert_eq!(o.medal, SILVER, "four of five: silver is three");
        assert_eq!(run.done, 100);
    }

    fn oracle(k: usize) -> Option<Outcome> {
        let (mut run, mut w) = begin(k, 0, false);
        let most = run.spec().limit().max(600) * SECOND + SECOND;
        let go = crate::input::Input {
            keys: crate::input::FORWARD,
            ..crate::input::Input::default()
        };
        let mut relic = 0;
        for _ in 0..most {
            w.step(&[go]);
            w.players[0].health = HEALTH;
            // fell the other side, as you would
            for s in 1..w.players.len() {
                if w.players[s].alive() && w.players[s].team != w.players[0].team {
                    let h = w.players[s].health;
                    w.hit(0, s, h);
                }
            }
            for f in run.foes.iter_mut() {
                f.hp = 0;
            }
            for d in w.dummies.iter_mut().filter(|d| d.kind == 0) {
                d.down = 1;
            }
            run.orbs.clear();
            match run.current() {
                Part::Course => {
                    if let Some(g) = run.gates.get(run.gate) {
                        w.players[0].body.origin = *g;
                    }
                }
                Part::LongJump => {
                    // down onto the field, as far past the lip as gold asks
                    let j = run.jump.unwrap();
                    let past = run.spec().medals.gold - j.length + 1;
                    w.players[0].body.origin = v3(0, j.field_y, j.lip.z - fx::int(past));
                }
                Part::Salvage => {
                    w.players[0].body.origin = match run.relics.get(relic) {
                        Some((p, _)) => {
                            relic += 1;
                            *p
                        }
                        None => run.spawn,
                    };
                }
                _ => {}
            }
            if let Some(r) = run.repair {
                w.players[0].body.origin = r;
            }
            run.step(&mut w, crate::input::FORWARD);
            if run.outcome.is_some() {
                return run.outcome;
            }
        }
        None
    }

    #[test]
    fn every_mission_can_be_cleared_inside_its_time() {
        for (k, m) in pack().missions.iter().enumerate() {
            let o = oracle(k);
            assert!(o.is_some_and(|o| o.cleared), "{} ({}): {:?}", m.id, m.kind, o);
        }
    }

    #[test]
    fn every_mission_starts_on_its_map_with_somewhere_to_walk() {
        for (k, m) in pack().missions.iter().enumerate() {
            let (run, w) = Run::start(k, 0, false);
            let name = if crate::courses::is_course(&m.map) {
                "course"
            } else if crate::royale::is_royale(&m.map) {
                "royale"
            } else {
                "map"
            };
            assert_eq!(w.map.name, format!("{name}_{}", m.map), "{}", m.id);
            assert!(run.spots.len() > 20, "{} has {} places", m.id, run.spots.len());
        }
    }
}
