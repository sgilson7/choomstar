//! Choomstar's screens as the page draws them: the star chart, the mission
//! list beside it with the picked mission's briefing, a mission's end, and
//! the lines over play. Every place and every word comes from here (CM-4);
//! the page draws them and knows no rule.

use serde::Serialize;

use crate::api::{action, panel, Hud, Panel, Row};
use crate::game::Game;
use crate::mission::{self, pack, Part, Spec, BRONZE, GOLD, NONE, SILVER};

/// A mission on the chart: its place (thousandths of the chart's side),
/// what it is, its best medal, and whether it is open.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Node {
    pub k: usize,
    pub x: i32,
    pub y: i32,
    pub kind: String,
    pub label: String,
    pub name: String,
    pub medal: u8,
    /// the medal's letter (B, S, G), so it reads without its colour
    pub mark: String,
    pub open: bool,
    pub picked: bool,
    pub secret: bool,
}

/// A medal as shown large: its level and letter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MedalMark {
    pub level: u8,
    pub mark: String,
}

pub fn letter(g: &Game, m: u8) -> String {
    if m == NONE {
        String::new()
    } else {
        g.strings.get(&format!("mark_letter_{m}"))
    }
}

/// A ring of the chart: a tier, its radius and name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Ring {
    pub r: i32,
    pub label: String,
}

/// A spoke's end and its kind's name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Spoke {
    pub x: i32,
    pub y: i32,
    pub kind: String,
    pub label: String,
}

/// A route into a mission from the one that opens it; lit once open.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Route {
    pub from: [i32; 2],
    pub to: [i32; 2],
    pub lit: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Chart {
    /// the chart's side and middle, thousandths (the page's view box)
    pub side: i32,
    pub middle: i32,
    pub rings: Vec<Ring>,
    pub spokes: Vec<Spoke>,
    pub routes: Vec<Route>,
    pub nodes: Vec<Node>,
    /// the medals earned, of all there are
    pub medals: [i32; 2],
    /// a node's radius and the gap between its medal rings (thousandths)
    pub node_r: i32,
    pub ring_gap: i32,
    pub heading: String,
}

/// A row of the mission list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Entry {
    pub k: usize,
    pub number: String,
    pub name: String,
    pub kind: String,
    pub kind_label: String,
    pub tier: String,
    pub medal: u8,
    pub mark: String,
    pub open: bool,
    pub picked: bool,
    pub action: crate::api::Action,
}

const SIDE: i32 = 1000;
const NODE_R: i32 = 17;
const RING_GAP: i32 = 5;
/// Spoke labels sit this far out (thousandths).
const SPOKE_LABEL: i32 = 496;

fn medal_key(m: u8) -> &'static str {
    match m {
        GOLD => "medal_gold",
        SILVER => "medal_silver",
        BRONZE => "medal_bronze",
        _ => "medal_none",
    }
}

/// Seconds as m:ss.
pub fn clock(secs: i32) -> String {
    format!("{}:{:02}", secs / 60, secs % 60)
}

/// Ticks as m:ss.t
pub fn clock_ticks(t: i32) -> String {
    let tenths = t * 10 / 60;
    format!("{}:{:02}.{}", tenths / 600, tenths / 10 % 60, tenths % 10)
}

fn shown(g: &Game, k: usize) -> bool {
    let m = &pack().missions[k];
    !m.secret || mission::open(m, &g.progress)
}

fn name_of(g: &Game, k: usize) -> String {
    if shown(g, k) {
        pack().missions[k].name.clone()
    } else {
        g.strings.get("secret_name")
    }
}

pub fn chart(g: &Game) -> Chart {
    let all = &pack().missions;
    let best = |k: usize| g.progress.get(&all[k].id).map_or(NONE, |b| b.medal);
    let nodes: Vec<Node> = (0..all.len())
        .map(|k| {
            let (x, y) = mission::chart_at(k);
            Node {
                k,
                x,
                y,
                kind: all[k].kind.clone(),
                label: format!("{}", k + 1),
                name: name_of(g, k),
                medal: best(k),
                mark: letter(g, best(k)),
                open: mission::open(&all[k], &g.progress),
                picked: k == g.picked,
                secret: all[k].secret && !shown(g, k),
            }
        })
        .collect();
    let routes = (0..all.len())
        .filter_map(|k| {
            let from = mission::route_from(k)?;
            let (a, b) = (mission::chart_at(from), mission::chart_at(k));
            Some(Route {
                from: [a.0, a.1],
                to: [b.0, b.1],
                lit: nodes[k].open,
            })
        })
        .collect();
    let rings = (0..=6)
        .map(|t| Ring {
            r: mission::ring(t),
            label: g.strings.get(&format!("tier_{t}")),
        })
        .collect();
    let spokes = mission::KINDS
        .iter()
        .enumerate()
        .map(|(i, kind)| {
            let a = -crate::fx::QUARTER_TURN + crate::fx::TURN / mission::KINDS.len() as i32 * i as i32;
            Spoke {
                x: 500 + crate::fx::mul(SPOKE_LABEL * crate::fx::ONE, crate::fx::cos(a)) / crate::fx::ONE,
                y: 500 + crate::fx::mul(SPOKE_LABEL * crate::fx::ONE, crate::fx::sin(a)) / crate::fx::ONE,
                kind: kind.to_string(),
                label: g.strings.get(&format!("kind_{kind}")),
            }
        })
        .collect();
    let earned: i32 = g.progress.values().map(|b| b.medal as i32).sum();
    Chart {
        side: SIDE,
        middle: SIDE / 2,
        rings,
        spokes,
        routes,
        nodes,
        medals: [earned, all.len() as i32 * GOLD as i32],
        node_r: NODE_R,
        ring_gap: RING_GAP,
        heading: g.strings.get("chart"),
    }
}

fn goal_text(g: &Game, m: &Spec) -> String {
    let s = &g.strings;
    let goal = &m.goal;
    let n = |key: &str, n: i32| s.get(key).replace("{n}", &n.to_string());
    match goal.kind.as_str() {
        "break_targets" => n("goal_break_targets", goal.targets),
        "reach_end" => n("goal_reach_end", goal.gates),
        "survive" => s.get("goal_survive").replace("{time}", &clock(goal.seconds)),
        "defeat" => n("goal_defeat", goal.count),
        "win_rounds" if goal.ladder > 0 => {
            n("goal_ladder", goal.ladder).replace("{lives}", &goal.rounds.to_string())
        }
        "win_rounds" => n("goal_win_rounds", goal.rounds),
        "team_score" => n("goal_team_score", goal.score),
        "clear_waves" => n("goal_clear_waves", goal.waves),
        "defeat_boss" => s.get("goal_defeat_boss"),
        "salvage" => n("goal_salvage", goal.relics).replace("{need}", &goal.need.to_string()),
        _ => n("goal_gauntlet", goal.stages).replace("{repairs}", &goal.repairs.to_string()),
    }
}

fn mark(g: &Game, m: &Spec, medal: u8, value: i32) -> String {
    let s = &g.strings;
    let name = s.get(medal_key(medal));
    match m.medals.measure.as_str() {
        "time" => s
            .get("mark_time")
            .replace("{medal}", &name)
            .replace("{time}", &clock(value)),
        "health" => s
            .get("mark_health")
            .replace("{medal}", &name)
            .replace("{n}", &value.to_string()),
        _ => s
            .get("mark_relics")
            .replace("{medal}", &name)
            .replace("{n}", &value.to_string()),
    }
}

fn best_text(g: &Game, m: &Spec) -> String {
    let s = &g.strings;
    match g.progress.get(&m.id) {
        Some(b) if b.medal > NONE => {
            let medal = s.get(medal_key(b.medal));
            if m.medals.measure == "time" {
                s.get("best_time")
                    .replace("{time}", &clock_ticks(b.score))
                    .replace("{medal}", &medal)
            } else {
                s.get("best_count")
                    .replace("{n}", &b.score.to_string())
                    .replace("{medal}", &medal)
            }
        }
        _ => s.get("best_none"),
    }
}

fn carry(g: &Game, m: &Spec) -> String {
    match m.loadout.as_str() {
        "any" => g.strings.get("carry_any"),
        "none" => g.strings.get("carry_none"),
        w => {
            let k = mission::WEAPONS.iter().position(|x| *x == w).unwrap_or(0);
            g.strings.get(crate::weapons::ALL[k].stats().name)
        }
    }
}

fn map_name(g: &Game, m: &Spec) -> String {
    g.strings.get(&format!("map_{}", m.map))
}

/// The missions screen: the list, and the picked mission's briefing.
pub fn mission_panels(g: &Game) -> Vec<Panel> {
    let s = &g.strings;
    let all = &pack().missions;
    let mut list = panel("mission-list");
    list.heading = s.get("mission_list");
    list.entries = (0..all.len())
        .map(|k| {
            let m = &all[k];
            let mut a = action(g, "mission.pick", &k.to_string(), "mission_list", false);
            a.label = name_of(g, k);
            Entry {
                k,
                number: format!("{:02}", k + 1),
                name: name_of(g, k),
                kind: m.kind.clone(),
                kind_label: s.get(&format!("kind_{}", m.kind)),
                tier: s.get(&format!("tier_{}", m.tier().min(7))),
                medal: g.progress.get(&m.id).map_or(NONE, |b| b.medal),
                mark: letter(g, g.progress.get(&m.id).map_or(NONE, |b| b.medal)),
                open: mission::open(m, &g.progress),
                picked: k == g.picked,
                action: a,
            }
        })
        .collect();
    list.actions = vec![action(g, "screen.menu", "", "to_menu", false)];
    let k = g.picked.min(all.len() - 1);
    let m = &all[k];
    let mut brief = panel("briefing");
    brief.rows_heading = s.get("briefing");
    if !shown(g, k) {
        brief.heading = s.get("secret_name");
        brief.text = s.get("secret_brief");
    } else {
        brief.heading = m.name.clone();
        brief.text = m.brief.clone();
        brief.rows = vec![
            Row {
                key: s.get("row_kind"),
                does: s.get(&format!("kind_{}", m.kind)),
            },
            Row {
                key: s.get("row_tier"),
                does: s.get(&format!("tier_{}", m.tier().min(7))),
            },
            Row {
                key: s.get("row_map"),
                does: map_name(g, m),
            },
            Row {
                key: s.get("row_carry"),
                does: carry(g, m),
            },
            Row {
                key: s.get("row_goal"),
                does: goal_text(g, m),
            },
        ];
        let md = &m.medals;
        for (medal, v) in [(GOLD, md.gold), (SILVER, md.silver), (BRONZE, md.bronze)] {
            brief.rows.push(Row {
                key: s.get(medal_key(medal)),
                does: mark(g, m, medal, v),
            });
        }
        brief.rows.push(Row {
            key: s.get("row_best"),
            does: best_text(g, m),
        });
    }
    let open = mission::open(m, &g.progress);
    if !open {
        let list: Vec<String> = mission::missing(m, &g.progress)
            .iter()
            .map(|r| {
                let q = mission::index(&r.mission).unwrap_or(0);
                s.get("need_medal")
                    .replace("{medal}", &s.get(medal_key(mission::medal_rank(&r.medal))))
                    .replace("{name}", &name_of(g, q))
            })
            .collect();
        brief.text = format!(
            "{} {}",
            brief.text,
            s.get("locked_by").replace("{list}", &list.join(", "))
        );
    }
    let mut start = action(g, "mission.start", "", "start_mission", true);
    start.enabled = open;
    brief.actions = vec![start];
    if !g.save_status.is_empty() {
        let mut note = panel("save-note");
        note.text = s.get(g.save_status);
        return vec![list, brief, note];
    }
    vec![list, brief]
}

/// A mission's end: cleared or not, the medal, the score, the best.
pub fn results_panels(g: &Game) -> Vec<Panel> {
    let s = &g.strings;
    let k = g.picked;
    let m = &pack().missions[k];
    let mut p = panel("results");
    let Some((o, before)) = g.ended else {
        p.heading = m.name.clone();
        p.actions = vec![action(g, "results.chart", "", "to_chart", false)];
        return vec![p];
    };
    p.heading = s.get(if o.cleared { "cleared" } else { "failed" });
    p.text = format!("{} {}", m.name, s.get(o.why));
    p.medal = Some(MedalMark {
        level: o.medal,
        mark: letter(g, o.medal),
    });
    let score = if !o.cleared {
        String::from("—")
    } else if m.medals.measure == "time" {
        clock_ticks(o.score)
    } else {
        o.score.to_string()
    };
    p.rows = vec![
        Row {
            key: s.get("row_medal"),
            does: s.get(medal_key(o.medal)),
        },
        Row {
            key: s.get("row_score"),
            does: score,
        },
        Row {
            key: s.get("row_best"),
            does: best_text(g, m),
        },
    ];
    let improved = o.cleared && (before.medal == NONE || mission::better(m, o.score, before.score));
    if improved {
        p.rows.push(Row {
            key: String::new(),
            does: s.get("new_best"),
        });
    }
    p.actions = vec![
        action(g, "mission.retry", "", "retry", true),
        action(g, "mission.next", "", "next_mission", false),
        action(g, "results.chart", "", "to_chart", false),
    ];
    vec![p]
}

/// The lines over play in a mission.
pub fn hud(g: &Game, playing: bool) -> Vec<Hud> {
    let Some(run) = &g.mission else {
        return Vec::new();
    };
    let s = &g.strings;
    let w = &g.world;
    let m = run.spec();
    let line = |id: &str, text: String, on: bool| Hud {
        id: id.into(),
        text,
        shown: playing && on,
        ..Hud::default()
    };
    let mut out = vec![line("m-name", m.name.clone(), true)];
    let clock_text = if m.limit() > 0 {
        s.get("hud_clock_limit")
            .replace("{time}", &clock_ticks(run.ticks))
            .replace("{limit}", &clock(m.limit()))
    } else {
        s.get("hud_clock").replace("{time}", &clock_ticks(run.ticks))
    };
    out.push(line("m-clock", clock_text, true));
    let goal = |label: &str| {
        s.get("hud_goal")
            .replace("{label}", &s.get(label))
            .replace("{done}", &run.done.to_string())
            .replace("{need}", &run.need.to_string())
    };
    let text = match run.current() {
        Part::Targets => goal("label_targets"),
        Part::Course => goal("label_gates"),
        Part::Horde { count, seconds } if *count == 0 => s
            .get("hud_survive")
            .replace("{time}", &clock((*seconds - run.done).max(0))),
        Part::Horde { .. } => goal("label_felled"),
        Part::Duel => s
            .get("hud_rounds")
            .replace("{won}", &run.done.to_string())
            .replace("{lost}", &run.lost.to_string()),
        Part::Team => s
            .get("hud_team")
            .replace("{us}", &run.score.0.to_string())
            .replace("{them}", &run.score.1.to_string()),
        Part::Arena { waves } => s
            .get("hud_wave")
            .replace("{n}", &run.wave.max(1).to_string())
            .replace("{of}", &waves.to_string()),
        Part::Boss => s.get("goal_defeat_boss"),
        Part::Salvage => goal("label_relics"),
    };
    out.push(line("m-goal", text, true));
    if m.kind == "gauntlet" {
        out.push(line(
            "m-stage",
            s.get("hud_stage")
                .replace("{n}", &(run.part + 1).to_string())
                .replace("{of}", &run.parts.len().to_string()),
            true,
        ));
    }
    if *run.current() == Part::Salvage {
        let text = if run.delivered >= run.need {
            s.get("hud_lift_open")
        } else {
            s.get("hud_carry").replace("{n}", &run.carried.to_string())
        };
        out.push(line("m-carry", text, true));
    }
    // the next medal a time can still earn
    if m.medals.measure == "time" {
        let secs = run.ticks / 60;
        let next = [(GOLD, m.medals.gold), (SILVER, m.medals.silver)]
            .into_iter()
            .find(|(_, t)| secs < *t);
        if let Some((medal, t)) = next {
            out.push(line(
                "m-medal",
                s.get("hud_next_medal")
                    .replace("{medal}", &s.get(medal_key(medal)))
                    .replace("{time}", &clock(t)),
                true,
            ));
        }
    }
    if let Some(pct) = run.boss_pct(w) {
        let name = run
            .boss_name()
            .map(String::from)
            .unwrap_or_else(|| s.get("boss_knight"));
        out.push(Hud {
            id: "m-boss".into(),
            text: name,
            shown: playing,
            fill: Some(pct.clamp(0, 100) * 100),
            ..Hud::default()
        });
    }
    out.push(line("m-restart", s.get("hud_restart"), run.ticks < 300));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clocks_read_minutes_seconds_and_tenths() {
        assert_eq!(clock(90), "1:30");
        assert_eq!(clock_ticks(60 * 83 + 30), "1:23.5");
    }

    #[test]
    fn the_chart_has_every_mission_five_open_and_routes_into_all_but_the_first_tier() {
        let g = crate::game::tests::game();
        let c = chart(&g);
        assert_eq!(c.nodes.len(), 50);
        assert_eq!(c.nodes.iter().filter(|n| n.open).count(), 5);
        assert_eq!(c.routes.len(), 45);
        assert!(c.nodes[49].secret, "the finale is hidden until opened");
        assert_eq!(c.nodes[49].name, g.strings.get("secret_name"));
    }

    #[test]
    fn a_locked_mission_says_what_opens_it_and_cannot_start() {
        let mut g = crate::game::tests::game();
        g.act("mission.pick", "5").unwrap();
        let panels = mission_panels(&g);
        let brief = panels.iter().find(|p| p.id == "briefing").unwrap();
        assert!(brief.text.contains("Bronze on First Light"), "{}", brief.text);
        assert!(!brief.actions[0].enabled);
        assert!(g.act("mission.start", "").is_err());
    }
}
