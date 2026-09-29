//! Runtime behaviour of every citizen: daily routines, walking, working,
//! sleeping, chatting, gossiping and reacting to what happens around them.

use super::path::Pather;
use super::people::*;
use crate::city::map::*;
use crate::render::character::Pose;
use crate::util::{hash2, Rng};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Act {
    Sleep,
    Work,
    Eat,
    Drink,
    Party,
    Church,
    Shop,
    Stroll,
    Home,
    Patrol,
    Visit(Pid),
    Play,
    Idle,
    Smuggle,
    Mourn,
}

impl Act {
    pub fn label(self) -> &'static str {
        match self {
            Act::Sleep => "dormindo",
            Act::Work => "trabalhando",
            Act::Eat => "comendo",
            Act::Drink => "bebendo",
            Act::Party => "se divertindo",
            Act::Church => "na missa",
            Act::Shop => "fazendo compras",
            Act::Stroll => "passeando",
            Act::Home => "em casa",
            Act::Patrol => "patrulhando",
            Act::Visit(_) => "visitando alguém",
            Act::Play => "brincando",
            Act::Idle => "à toa",
            Act::Smuggle => "em negócios escusos",
            Act::Mourn => "de luto",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum AState {
    Normal,
    /// Fleeing from a position until a game minute
    Flee { from: Vec2, until: f32 },
    /// Walking to investigate something
    Investigate { at: Vec2, until: f32 },
    /// Going to the police station to report crime id
    Report { crime: u32 },
    /// Hostile: attacking Elias
    Hostile,
    /// Chasing Elias (police)
    Chase,
    /// Suspect running away from Elias (pursuit)
    Escape { hideout: usize },
    Cower { until: f32 },
    Unconscious { until: f32 },
    Dead,
    /// being dragged / carried by Elias
    Carried,
    /// following Elias (date, network member escort)
    Follow { until: f32 },
    /// frozen in a conversation with Elias
    Talking,
    Arrested,
    /// tied up with rope (sits on the floor, may call for help)
    Tied { since: f32, gagged: bool },
    /// walking with Elias at gunpoint (kidnapped)
    Hostage,
    /// hands up during a hold-up
    HandsUp { until: f32 },
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Agent {
    pub pid: Pid,
    pub pos: Vec2,
    pub facing: f32,
    pub path: Vec<Vec2>,
    pub pi: usize,
    pub act: Act,
    pub target_b: Option<usize>,
    pub target: Vec2,
    pub arrived: bool,
    pub state: AState,
    pub pose: Pose,
    pub speed: f32,
    pub replan_at: f32,
    pub bubble: Option<(String, f32)>,
    pub chat: Option<(Pid, f32)>,
    pub chat_line: u8,
    pub health: f32,
    pub alert: f32,
    pub seen_elias_crime: bool,
    pub knows_bodies: Vec<Pid>,
    pub ammo: i32,
    pub shoot_cd: f32,
    pub stuck: f32,
    pub last_pos: Vec2,
    pub y: f32,
    pub sedated: bool,
    pub bleeding: f32,
    /// case script override: stay at this position
    pub pinned: Option<Vec2>,
    pub in_car: bool,
    pub greet_cd: f32,
    pub drunk: f32,
}

impl Agent {
    pub fn new(pid: Pid, pos: Vec2, armed: bool) -> Agent {
        Agent {
            pid,
            pos,
            facing: 0.0,
            path: Vec::new(),
            pi: 0,
            act: Act::Idle,
            target_b: None,
            target: pos,
            arrived: true,
            state: AState::Normal,
            pose: Pose::Idle,
            speed: 0.0,
            replan_at: 0.0,
            bubble: None,
            chat: None,
            chat_line: 0,
            health: 100.0,
            alert: 0.0,
            seen_elias_crime: false,
            knows_bodies: Vec::new(),
            ammo: if armed { 12 } else { 0 },
            shoot_cd: 0.0,
            stuck: 0.0,
            last_pos: pos,
            y: 0.0,
            sedated: false,
            bleeding: 0.0,
            pinned: None,
            in_car: false,
            greet_cd: 0.0,
            drunk: 0.0,
        }
    }
    pub fn active(&self) -> bool {
        !matches!(self.state, AState::Dead | AState::Unconscious { .. } | AState::Carried | AState::Arrested | AState::Tied { .. } | AState::Hostage)
    }
    pub fn say(&mut self, s: impl Into<String>, secs: f32) {
        self.bubble = Some((s.into(), secs));
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Rumor {
    pub id: u32,
    pub text: String,
    pub about: Option<Pid>,
    pub about_elias: bool,
    pub heat: i32,
    pub known: Vec<Pid>,
    pub day: i32,
}

#[derive(Resource, Default)]
pub struct Sim {
    pub agents: Vec<Agent>,
    pub by_pid: std::collections::HashMap<Pid, usize>,
    pub rumors: Vec<Rumor>,
    pub next_rumor: u32,
    pub tick: u64,
    pub pather: Option<Pather>,
    pub rng: Option<Rng>,
}

impl Sim {
    pub fn agent(&self, pid: Pid) -> Option<&Agent> {
        self.by_pid.get(&pid).map(|&i| &self.agents[i])
    }
    pub fn agent_mut(&mut self, pid: Pid) -> Option<&mut Agent> {
        match self.by_pid.get(&pid) {
            Some(&i) => Some(&mut self.agents[i]),
            None => None,
        }
    }
    pub fn rebuild_index(&mut self) {
        self.by_pid = self.agents.iter().enumerate().map(|(i, a)| (a.pid, i)).collect();
    }
    pub fn add_rumor(&mut self, text: impl Into<String>, about: Option<Pid>, about_elias: bool, heat: i32, first: Vec<Pid>, day: i32) -> u32 {
        let id = self.next_rumor;
        self.next_rumor += 1;
        self.rumors.push(Rumor { id, text: text.into(), about, about_elias, heat, known: first, day });
        id
    }
}

pub fn is_weekend(day: i32) -> bool {
    day % 7 == 6
}

/// Decide what a person wants to be doing now.
pub fn plan(p: &Person, pop: &Population, m: &Map, hour: f32, day: i32, rain: f32, year: i32) -> (Act, Option<usize>) {
    let age = p.age(year);
    let seed = p.seed;
    let wake = 6.0 + (seed % 3) as f32 * 0.5;
    let bed = 22.0 + (seed % 5) as f32 * 0.5 + p.traits.sociability as f32 / 60.0;
    let (s0, s1) = p.job.shift();
    let h = hour;
    let in_shift = |x: f32| {
        if s1 > 24.0 {
            x >= s0 || x < s1 - 24.0
        } else {
            x >= s0 && x < s1
        }
    };
    let night_worker = s1 > 24.0 || s0 >= 16.0;
    let works_weekend = matches!(p.job, Job::Police | Job::Bartender | Job::Musician | Job::Nurse | Job::Doctor | Job::HotelClerk | Job::Cook | Job::Waiter | Job::Dancer | Job::Madam | Job::Priest | Job::Gangster | Job::Smuggler | Job::Farmer);
    let home = p.home;
    // mourning: a loved one died recently
    if p.destiny.iter().any(|d| d == "mourning") && (8.0..20.0).contains(&h) && seed % 2 == 0 {
        return (Act::Mourn, home);
    }
    // working?
    if let Some(w) = p.work {
        if in_shift(h) && (works_weekend || !is_weekend(day)) {
            if matches!(p.job, Job::Police) && (seed + day as u32) % 3 != 0 {
                return (Act::Patrol, None);
            }
            if p.job == Job::Smuggler {
                return (Act::Smuggle, Some(w));
            }
            return (Act::Work, Some(w));
        }
    }
    // sleeping
    let sleeping = if night_worker && p.work.is_some() {
        let ws = (s1 - 24.0).max(0.0) + 0.5;
        h >= ws && h < ws + 8.0
    } else if age < 12 {
        !(7.0..20.5).contains(&h)
    } else {
        h >= bed || h < wake
    };
    if sleeping {
        return (Act::Sleep, home.or_else(|| {
            // homeless: an abandoned building or the church
            m.find_building(BKind::Abandoned).or_else(|| m.find_building(BKind::Church))
        }));
    }
    // children play
    if age < 14 {
        if (9.0..16.0).contains(&h) && rain < 0.4 {
            return (Act::Play, None);
        }
        return (Act::Home, home);
    }
    // Sunday mass
    if is_weekend(day) && (8.0..11.0).contains(&h) && p.traits.faith > 45 {
        return (Act::Church, m.find_building(BKind::Church));
    }
    // meals
    if (12.0..13.0).contains(&h) || (19.0..20.0).contains(&h) {
        if p.job.income() >= 7 && seed % 2 == 0 {
            let rs = m.buildings_of(BKind::Restaurant);
            if !rs.is_empty() {
                return (Act::Eat, Some(rs[(seed as usize) % rs.len()]));
            }
        }
        return (Act::Eat, home);
    }
    // errands in the morning
    if (9.0..12.0).contains(&h) && (seed + day as u32) % 3 == 0 {
        let kinds = [BKind::Market, BKind::General, BKind::Pharmacy, BKind::Clothing];
        let k = kinds[((seed + day as u32) % 4) as usize];
        if let Some(b) = pick_building(m, k, seed) {
            if m.buildings[b].kind.is_open(h) {
                return (Act::Shop, Some(b));
            }
        }
    }
    // evening leisure
    if h >= 17.0 || h < 2.0 {
        let roll = (hash2(seed as i32, day, 3) % 100) as u8;
        if p.job == Job::Gangster || p.job == Job::Drifter {
            return (Act::Drink, pick_building(m, BKind::Bar, seed));
        }
        if p.traits.sociability > 55 && roll < 55 {
            let club = age < 40 && roll < 25;
            let k = if club { BKind::Club } else { BKind::Bar };
            if let Some(b) = pick_building(m, k, seed + day as u32) {
                if m.buildings[b].kind.is_open(h) {
                    return (if club { Act::Party } else { Act::Drink }, Some(b));
                }
            }
        }
        if p.traits.faith > 80 && roll < 30 && h < 20.0 {
            return (Act::Church, m.find_building(BKind::Church));
        }
        // visit a friend
        if roll < 70 {
            if let Some(r) = p.rels.iter().find(|r| matches!(r.kind, RelKind::Friend | RelKind::Lover) && r.val > 30) {
                let f = pop.get(r.to);
                if f.alive() && f.home.is_some() && f.city == p.city {
                    return (Act::Visit(r.to), f.home);
                }
            }
        }
        if rain < 0.3 && roll < 85 && h < 21.0 {
            return (Act::Stroll, None);
        }
        return (Act::Home, home);
    }
    // daytime without work
    if rain < 0.4 && (hash2(seed as i32, day, ((h as i32) / 2) as u32) % 3) == 0 {
        return (Act::Stroll, None);
    }
    if p.job == Job::Drifter {
        return (Act::Stroll, None);
    }
    (Act::Home, home)
}

fn pick_building(m: &Map, k: BKind, seed: u32) -> Option<usize> {
    let v = m.buildings_of(k);
    if v.is_empty() {
        None
    } else {
        Some(v[(seed as usize) % v.len()])
    }
}

/// Choose the exact position inside a building for an activity.
pub fn spot_for(p: &Person, m: &Map, b: usize, act: Act, rng: &mut Rng) -> (Vec2, Pose) {
    let bl = &m.buildings[b];
    let want = match act {
        Act::Sleep => Some(SpotKind::Bed),
        Act::Work => Some(SpotKind::Work),
        Act::Eat | Act::Drink | Act::Visit(_) | Act::Home => Some(SpotKind::Seat),
        Act::Church | Act::Mourn => Some(SpotKind::Pray),
        Act::Party => Some(SpotKind::Stand),
        _ => None,
    };
    if act == Act::Sleep {
        if let Some(i) = p.bed {
            if Some(b) == p.home && i < bl.spots.len() {
                return (bl.spots[i].pos + vec2(0.0, 0.5), Pose::Lie);
            }
        }
        if let Some(s) = bl.spots.iter().find(|s| s.kind == SpotKind::Bed && s.owner.is_none()) {
            return (s.pos + vec2(0.0, 0.5), Pose::Lie);
        }
    }
    if act == Act::Work && matches!(p.job, Job::Musician) {
        if let Some(s) = bl.spots_of(SpotKind::Stage).next() {
            return (s.pos, Pose::Play);
        }
    }
    if act == Act::Work && matches!(p.job, Job::Dancer) {
        return (random_floor(m, b, rng), Pose::Dance);
    }
    if act == Act::Work && p.job == Job::Priest {
        if let Some(s) = bl.spots_of(SpotKind::Work).next() {
            return (s.pos, Pose::Talk);
        }
    }
    if let Some(k) = want {
        let spots: Vec<&Spot> = bl.spots.iter().filter(|s| s.kind == k).collect();
        if !spots.is_empty() {
            let s = spots[rng.idx(spots.len())];
            let pose = match k {
                SpotKind::Seat => Pose::Sit,
                SpotKind::Pray => Pose::Pray,
                SpotKind::Work => {
                    if matches!(p.job, Job::Clerk | Job::Journalist | Job::Banker | Job::Police | Job::Lawyer | Job::Hacker | Job::RadioHost) {
                        Pose::Sit
                    } else {
                        Pose::Work
                    }
                }
                _ => Pose::Idle,
            };
            return (s.pos, pose);
        }
    }
    if act == Act::Party {
        return (random_floor(m, b, rng), if rng.chance(0.6) { Pose::Dance } else { Pose::Idle });
    }
    (random_floor(m, b, rng), Pose::Idle)
}

pub fn random_floor(m: &Map, b: usize, rng: &mut Rng) -> Vec2 {
    let bl = &m.buildings[b];
    for _ in 0..40 {
        let x = bl.x + 1 + rng.range(0, (bl.w - 2).max(1));
        let y = bl.y + 1 + rng.range(0, (bl.h - 2).max(1));
        if !m.blocked(x, y) && matches!(m.get(x, y), Tile::Floor | Tile::Plaza) {
            return tile_center(x, y);
        }
    }
    bl.center_px()
}

/// A random outdoor spot for strolling (parks, plaza, sidewalks near home).
pub fn stroll_target(m: &Map, near: Vec2, rng: &mut Rng) -> Vec2 {
    for _ in 0..60 {
        let d = vec2(rng.rangef(-25.0, 25.0), rng.rangef(-25.0, 25.0));
        let p = near + d;
        let (x, y) = to_tile(p);
        if m.inb(x, y) && matches!(m.get(x, y), Tile::Sidewalk | Tile::Plaza | Tile::Gravel | Tile::Grass | Tile::Dock) && !m.blocked(x, y) {
            return tile_center(x, y);
        }
    }
    near
}
