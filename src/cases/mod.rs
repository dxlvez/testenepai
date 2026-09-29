//! Investigation system: case definitions, progress, evidence and deduction.

pub mod defs;
pub mod era1;
pub mod run;

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CaseProgress {
    pub id: u8,
    pub started: bool,
    pub cast: Vec<u32>,
    pub clue_pos: Vec<Option<(f32, f32)>>,
    pub found: Vec<u8>,
    pub topics: Vec<(u8, String)>,
    pub lies_heard: Vec<(u8, String)>,
    pub lies_broken: Vec<(u8, String)>,
    pub echoes: Vec<u8>,
    pub links: Vec<(u8, u8)>,
    pub accused: Option<u8>,
    pub method: Option<u8>,
    pub motive: Option<u8>,
    pub sphere: Option<u8>,
    pub solved: bool,
    pub correct: bool,
    pub layers: u8,
    pub timeline: u32,
    pub year: i32,
    pub false_revealed: bool,
    pub notes: Vec<String>,
    pub stayed_years: i32,
}

impl CaseProgress {
    pub fn has(&self, c: u8) -> bool {
        self.found.contains(&c)
    }
}

/// How much the truth has bled into the world (0..1): drives the red creep.
pub fn redness(game: &crate::state::Game) -> f32 {
    let solved = game.cases.iter().filter(|c| c.solved).count() as f32;
    let layers: f32 = game.cases.iter().map(|c| c.layers as f32).sum();
    ((solved * 0.5 + layers * 0.35) / 30.0).clamp(0.0, 1.0)
}

/// All 30 cases.
pub fn all_cases() -> Vec<defs::CaseDef> {
    let mut v = vec![era1::case01()];
    v.extend(era1::rest());
    v.sort_by_key(|c| c.id);
    v
}
