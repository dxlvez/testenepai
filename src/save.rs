//! Case files (save slots). Each save is a JSON "CASE FILE" next to the exe.

use crate::cases::run::CaseDb;
use crate::state::Game;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, Default)]
pub struct SaveReq {
    pub save: Option<u32>,
    pub load: Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct SaveFile {
    pub meta: SaveMeta,
    pub game: Game,
    pub opened: Vec<(i32, i32)>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SaveMeta {
    pub slot: u32,
    pub city: String,
    pub year: i32,
    pub case_title: String,
    pub status: String,
    pub timeline: String,
    pub casualties: i32,
    pub witnesses: i32,
    pub relationships: i32,
    pub alterations: u32,
    pub played_secs: f32,
    pub date: String,
}

pub fn dir() -> std::path::PathBuf {
    let base = std::env::current_exe().ok().and_then(|p| p.parent().map(|d| d.to_path_buf())).unwrap_or_default();
    let d = base.join("arquivos_de_caso");
    let _ = std::fs::create_dir_all(&d);
    d
}

pub fn path(slot: u32) -> std::path::PathBuf {
    dir().join(format!("case_file_{:03}.json", slot))
}

pub fn list() -> Vec<SaveMeta> {
    let mut v = Vec::new();
    if let Ok(rd) = std::fs::read_dir(dir()) {
        for e in rd.flatten() {
            if let Ok(s) = std::fs::read_to_string(e.path()) {
                // only parse the meta block quickly
                if let Some(end) = s.find("\"game\"") {
                    let head = format!("{}\"x\":0}}", &s[..end]);
                    #[derive(Deserialize)]
                    struct Head {
                        meta: SaveMeta,
                    }
                    if let Ok(h) = serde_json::from_str::<Head>(&head) {
                        v.push(h.meta);
                        continue;
                    }
                }
                if let Ok(f) = serde_json::from_str::<SaveFile>(&s) {
                    v.push(f.meta);
                }
            }
        }
    }
    v.sort_by_key(|m| m.slot);
    v
}

pub fn meta_for(game: &Game, db: &CaseDb, slot: u32) -> SaveMeta {
    let def = db.get(game.case_idx as u8);
    let prog = game.cases.iter().find(|c| c.id == game.case_idx as u8);
    SaveMeta {
        slot,
        city: game.city.upper().into(),
        year: game.year,
        case_title: def.map(|d| d.title.to_uppercase()).unwrap_or_else(|| "PRÓLOGO".into()),
        status: match prog {
            Some(p) if p.solved && p.correct => "RESOLVIDO".into(),
            Some(p) if p.solved => "RESOLUÇÃO FALSA".into(),
            _ => "NÃO RESOLVIDO".into(),
        },
        timeline: game.timeline_code(),
        casualties: game.get_stat("kills"),
        witnesses: game.police.crimes.iter().map(|c| c.witnesses.len() as i32).sum(),
        relationships: game.pop.people.iter().filter(|p| p.elias.romance != crate::sim::people::Romance::None || p.elias.trust > 40).count() as i32,
        alterations: game.alterations,
        played_secs: game.played_secs,
        date: game.date_str(),
    }
}

pub fn write(game: &Game, db: &CaseDb, slot: u32, opened: &[(i32, i32)]) -> bool {
    let f = SaveFile { meta: meta_for(game, db, slot), game: game.clone(), opened: opened.to_vec() };
    match serde_json::to_string(&f) {
        Ok(s) => std::fs::write(path(slot), s).is_ok(),
        Err(_) => false,
    }
}

pub fn read(slot: u32) -> Option<SaveFile> {
    let s = std::fs::read_to_string(path(slot)).ok()?;
    serde_json::from_str(&s).ok()
}

/// Persistent knowledge across playthroughs (e.g. the secret menu option).
pub fn global_flag(name: &str) -> bool {
    std::fs::read_to_string(dir().join("fio.dat")).map(|s| s.lines().any(|l| l == name)).unwrap_or(false)
}

pub fn set_global_flag(name: &str) {
    let p = dir().join("fio.dat");
    let mut s = std::fs::read_to_string(&p).unwrap_or_default();
    if !s.lines().any(|l| l == name) {
        s.push_str(name);
        s.push('\n');
        let _ = std::fs::write(p, s);
    }
}

#[allow(clippy::too_many_arguments)]
pub fn save_load_system(
    mut req: ResMut<SaveReq>,
    mut game: ResMut<Game>,
    db: Res<CaseDb>,
    mut it: ResMut<crate::interact::Interact>,
    mut load: ResMut<crate::world::LoadCity>,
    mut toasts: ResMut<crate::ui::hud::Toasts>,
    mut rt: ResMut<crate::player::PlayerRt>,
    mut crt: ResMut<crate::cases::run::CaseRt>,
) {
    if let Some(slot) = req.save.take() {
        game.save_slot = slot;
        if write(&game, &db, slot, &it.opened) {
            // autosave also keeps the "last temporal point"
            if slot != 0 {
                let _ = write(&game, &db, 0, &it.opened);
            }
            toasts.push(format!("Arquivo de caso {:03} salvo.", slot));
        } else {
            toasts.push("Não foi possível salvar.");
        }
    }
    if let Some(slot) = req.load.take() {
        if let Some(f) = read(slot) {
            *game = f.game;
            it.opened = f.opened;
            it.searched.clear();
            load.0 = true;
            rt.carrying = None;
            rt.in_car = false;
            rt.weapon_out = false;
            rt.look_dirty = true;
            crt.spawned = None;
            crt.echo = None;
            crt.red_sight = false;
            toasts.push(format!("Arquivo de caso {:03} carregado.", slot));
        } else {
            toasts.push("Arquivo corrompido ou inexistente.");
        }
    }
}
