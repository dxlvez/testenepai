//! Case runtime: preparing the cast in the city, placing evidence, echoes,
//! red sight, and resolving an accusation into a timeline alteration.

use super::defs::*;
use super::CaseProgress;
use crate::city::map::*;
use crate::sim::people::*;
use crate::state::Game;
use crate::util::Rng;
use bevy::prelude::*;

#[derive(Resource)]
pub struct CaseDb(pub Vec<CaseDef>);

impl CaseDb {
    pub fn get(&self, id: u8) -> Option<&CaseDef> {
        self.0.iter().find(|c| c.id == id)
    }
}

/// Current case definition + progress index.
pub fn current<'a>(game: &Game, db: &'a CaseDb) -> Option<(&'a CaseDef, usize)> {
    let pi = game.cases.iter().position(|c| c.id == game.case_idx as u8 && !c.solved)?;
    let def = db.get(game.case_idx as u8)?;
    if def.city != game.city {
        return None;
    }
    Some((def, pi))
}

pub fn resolve_place(p: Place, m: &Map, game: &Game, prog: &CaseProgress, rng: &mut Rng) -> Vec2 {
    let nth = |k: BKind, n: u8| -> Option<usize> {
        let v = m.buildings_of(k);
        if v.is_empty() {
            None
        } else {
            Some(v[n as usize % v.len()])
        }
    };
    let inside = |b: usize, rng: &mut Rng| crate::sim::agents::random_floor(m, b, rng);
    match p {
        Place::B(k, n) => nth(k, n).map(|b| inside(b, rng)).unwrap_or(m.spawn),
        Place::Near(k, n) => nth(k, n).map(|b| m.buildings[b].outside_px()).unwrap_or(m.spawn),
        Place::HomeOf(i) => prog
            .cast
            .get(i as usize)
            .and_then(|pid| game.pop.people.get(*pid as usize))
            .and_then(|p| p.home)
            .map(|b| inside(b, rng))
            .unwrap_or(m.spawn),
        Place::WorkOf(i) => {
            let p = prog.cast.get(i as usize).and_then(|pid| game.pop.people.get(*pid as usize));
            let b = p.and_then(|p| p.work.or_else(|| p.job.workplace().and_then(|k| m.find_building(k))));
            b.map(|b| inside(b, rng)).unwrap_or(m.spawn)
        }
        Place::Park | Place::Cemetery | Place::Woods => {
            let keys: &[&str] = match p {
                Place::Park => &["Praça", "Parque", "Park", "Square", "Panhandle", "Byparken"],
                Place::Cemetery => &["Cemit", "Friedhof"],
                _ => &["Bosque", "Floresta", "Colinas", "Praia"],
            };
            let d = m.districts.iter().find(|d| keys.iter().any(|k| d.name.contains(k)));
            if let Some(d) = d {
                for _ in 0..40 {
                    let x = d.x + rng.range(1, d.w - 1);
                    let y = d.y + rng.range(1, d.h - 1);
                    if !m.blocked(x, y) {
                        return tile_center(x, y);
                    }
                }
            }
            m.spawn
        }
        Place::Docks => {
            for _ in 0..200 {
                let x = rng.range(0, m.w);
                let y = rng.range(m.h - 16, m.h);
                if m.get(x, y) == Tile::Dock && !m.blocked(x, y) {
                    return tile_center(x, y);
                }
            }
            m.spawn
        }
        Place::Alley(n) => {
            let mut k = 0;
            for y in 0..m.h {
                for x in 0..m.w {
                    if m.get(x, y) == Tile::Alley && !m.blocked(x, y) && (x + y) % 17 == 0 {
                        if k == n as i32 * 7 {
                            return tile_center(x, y);
                        }
                        k += 1;
                    }
                }
            }
            m.spawn
        }
    }
}

/// Create the case cast in the population (once) and pin their homes on the
/// map. Called while loading a city, before agents are spawned.
pub fn prepare_case(game: &mut Game, m: &mut Map, db: &CaseDb) {
    let id = game.case_idx as u8;
    let Some(def) = db.get(id) else { return };
    if def.city != game.city {
        return;
    }
    let pi = match game.cases.iter().position(|c| c.id == id) {
        Some(i) => i,
        None => {
            game.cases.push(CaseProgress { id, year: game.year, timeline: game.timeline, ..Default::default() });
            game.cases.len() - 1
        }
    };
    if game.cases[pi].solved {
        return;
    }
    let mut rng = Rng::new(0xCA5E + id as u64 * 131 + game.timeline as u64);
    if game.cases[pi].cast.is_empty() {
        let year = game.year;
        let city = game.city;
        let mut pids = Vec::new();
        for c in &def.cast {
            let pid = game.pop.new_person(&mut rng, city, c.female, year - c.age, Some(c.last.to_string()), year);
            let p = game.pop.get_mut(pid);
            p.first = c.first.to_string();
            p.last = c.last.to_string();
            p.job = c.job;
            p.case_role = Some((id, pids.len() as u8));
            p.notable = true;
            p.soul = c.soul;
            p.traits.fear = c.temper.0;
            p.traits.courage = c.temper.1;
            p.traits.morality = c.temper.2;
            p.traits.greed = c.temper.3;
            p.look = era_look(&mut rng, c.female, c.age, c.job, year, city);
            if let Some(s) = c.soul {
                if game.cases.iter().any(|cp| cp.solved) || game.timeline > 1 {
                    p.elias.deja_vu = (game.timeline.min(10)) as u8 * 10 + s;
                }
            }
            if c.dead {
                p.life = Life::Dead { year, day: game.day - 3, cause: "assassinado".into(), by_elias: false };
            }
            pids.push(pid);
        }
        // homes
        for (i, c) in def.cast.iter().enumerate() {
            let b = match c.home {
                Place::B(k, n) => {
                    let v = m.buildings_of(k);
                    if v.is_empty() {
                        None
                    } else {
                        Some(v[n as usize % v.len()])
                    }
                }
                Place::HomeOf(j) => game.pop.get(pids[j as usize]).home,
                _ => None,
            };
            game.pop.get_mut(pids[i]).home = b.or_else(|| m.find_building(BKind::Apartment));
        }
        // evict previous residents of the cast homes
        let cast_homes: Vec<usize> = pids.iter().filter_map(|p| game.pop.get(*p).home).collect();
        let fallback: Vec<usize> = m.buildings_of(BKind::Apartment);
        let mut k = 0;
        for p in game.pop.people.iter_mut() {
            if p.city == city && p.case_role.is_none() && p.alive() {
                if let Some(h) = p.home {
                    if cast_homes.contains(&h) && !fallback.is_empty() {
                        p.home = Some(fallback[k % fallback.len()]);
                        k += 1;
                    }
                }
            }
        }
        // same surname + same home = married couple
        for i in 0..pids.len() {
            for j in i + 1..pids.len() {
                let (a, b) = (game.pop.get(pids[i]).clone(), game.pop.get(pids[j]).clone());
                if a.last == b.last && a.home == b.home && a.female != b.female && a.age(year) > 18 && b.age(year) > 18 {
                    game.pop.marry(pids[i], pids[j]);
                    game.pop.relate(pids[i], pids[j], RelKind::Lover, 60);
                    game.pop.relate(pids[j], pids[i], RelKind::Lover, 60);
                }
            }
        }
        // the cast knows each other a bit
        for (a, b) in def.threads {
            let (pa, pb) = (pids[*a as usize], pids[*b as usize]);
            if game.pop.get(pa).rel_to(pb).is_none() {
                game.pop.relate(pa, pb, RelKind::Friend, 10);
                game.pop.relate(pb, pa, RelKind::Friend, 10);
            }
        }
        game.cases[pi].cast = pids;
        game.cases[pi].started = true;
        game.cases[pi].year = game.year;
    }
    // resolve clue positions (after homes are known)
    if game.cases[pi].clue_pos.len() != def.clues.len() {
        let prog = game.cases[pi].clone();
        let mut pos = Vec::new();
        for c in &def.clues {
            pos.push(c.place.map(|p| {
                let v = resolve_place(p, m, game, &prog, &mut rng);
                (v.x, v.y)
            }));
        }
        game.cases[pi].clue_pos = pos;
    }
}

/// Echo positions are resolved like clues.
pub fn echo_positions(def: &CaseDef, game: &Game, prog: &CaseProgress, m: &Map) -> Vec<Vec2> {
    let mut rng = Rng::new(0xEC0 + def.id as u64);
    def.echoes
        .iter()
        .map(|e| {
            // echoes that unlock a clue with a position appear right there
            if let Some(Some((x, y))) = prog.clue_pos.get(e.unlocks as usize) {
                return Vec2::new(*x, *y);
            }
            resolve_place(e.place, m, game, prog, &mut rng)
        })
        .collect()
}

/// Is this clue visible to Elias (observation vs hidden)?
pub fn clue_visible(c: &Clue, game: &Game, red_sight: bool) -> bool {
    let obs = game.player.skill(crate::state::Skill::Observation) as u8;
    obs + if red_sight { 2 } else { 0 } >= c.hidden
}

/// Accusation check: returns (correct culprit, layers 0..3).
pub fn judge(def: &CaseDef, prog: &CaseProgress) -> (bool, u8) {
    let correct = prog.accused == Some(def.culprit);
    let mut layers = 0;
    if correct {
        layers = 1;
        if prog.motive == Some(def.motive) && prog.method == Some(def.method) {
            layers = 2;
            if prog.sphere == Some(def.anomaly) {
                layers = 3;
            }
        }
    }
    (correct, layers)
}

/// Apply the outcome of a case to the world. Returns a list of human-readable
/// alterations for the TIMELINE ALTERED screen.
pub fn apply_outcome(game: &mut Game, def: &CaseDef, pi: usize, correct: bool, layers: u8) -> Vec<String> {
    let mut out = Vec::new();
    let o = if correct { def.on_true.clone() } else { def.on_false.clone() };
    let cast = game.cases[pi].cast.clone();
    let year = game.year;
    let day = game.day;
    let before_alive = game.pop.in_city(game.city, year).len();
    // the accused, if wrong, pays for it
    if !correct {
        if let Some(a) = game.cases[pi].accused {
            if let Some(&pid) = cast.get(a as usize) {
                let p = game.pop.get_mut(pid);
                p.life = Life::Jailed;
                out.push(format!("{} foi condenad{} por um crime que talvez não cometeu.", p.name(), p.o()));
                for c in p.children.clone() {
                    game.pop.get_mut(c).destiny.push("hates_elias".into());
                }
            }
        }
    }
    for (i, fate) in &o.fates {
        let Some(&pid) = cast.get(*i as usize) else { continue };
        let name = game.pop.get(pid).name();
        match *fate {
            "jailed" => {
                game.pop.get_mut(pid).life = Life::Jailed;
                out.push(format!("{} está preso.", name));
            }
            "dead" => {
                game.pop.get_mut(pid).life = Life::Dead { year, day, cause: "desfecho do caso".into(), by_elias: false };
                out.push(format!("{} morreu.", name));
            }
            "left_city" => {
                let c = game.city;
                game.pop.get_mut(pid).life = Life::Moved(c);
                out.push(format!("{} deixou a cidade.", name));
            }
            other => {
                game.pop.get_mut(pid).destiny.push(other.to_string());
                let desc = match other {
                    "police" => "vai entrar para a polícia",
                    "criminal" => "vai continuar livre para matar",
                    "grateful" => "nunca vai esquecer o que você fez",
                    "survived" => "vai reconstruir a vida",
                    "journalist" => "vai virar jornalista",
                    "hates_elias" => "vai crescer odiando você",
                    _ => "teve o destino alterado",
                };
                out.push(format!("{} {}.", name, desc));
            }
        }
        // children inherit the consequence
        let kids = game.pop.get(pid).children.clone();
        for k in kids {
            if matches!(*fate, "jailed" | "dead") {
                let kid = game.pop.get_mut(k);
                if kid.age(year) < 18 {
                    kid.destiny.push(if kid.traits.morality > 50 { "police".into() } else { "criminal".into() });
                    kid.destiny.push("mourning".into());
                }
            }
        }
    }
    for f in &o.flags {
        game.set(f);
    }
    // closures
    for k in &o.close {
        out.push(format!("Um(a) {} fechou as portas.", k.label()));
        game.set(&format!("close:{:?}", k));
    }
    // ripple: random people connected to the cast change jobs / leave / die
    let mut rng = Rng::new(0xA17E + def.id as u64 * 7 + game.timeline as u64 + if correct { 1 } else { 0 });
    let n_ripples = if correct { 3 + layers as usize } else { 7 };
    let ids = game.pop.in_city(game.city, year);
    for _ in 0..n_ripples {
        if ids.is_empty() {
            break;
        }
        let pid = ids[rng.idx(ids.len())];
        if game.pop.get(pid).case_role.is_some() || game.pop.get(pid).elias.romance != Romance::None {
            continue;
        }
        let r = rng.f();
        let p = game.pop.get_mut(pid);
        let name = p.name();
        if r < 0.25 {
            p.life = Life::Moved(p.city);
            out.push(format!("{} nunca chegou a morar aqui.", name));
        } else if r < 0.45 {
            let j = *rng.pick(&[Job::Police, Job::Journalist, Job::Gangster, Job::Musician, Job::Merchant, Job::Doctor]);
            p.job = j;
            out.push(format!("{} agora é {}.", name, j.label(p.female).to_lowercase()));
        } else if r < 0.55 && !correct {
            p.life = Life::Dead { year, day, cause: "violência".into(), by_elias: false };
            out.push(format!("{} morreu em outra versão desta noite.", name));
        } else {
            p.destiny.push("deja_vu".into());
            p.elias.deja_vu = p.elias.deja_vu.saturating_add(10);
            out.push(format!("{} sonha com um homem que nunca conheceu.", name));
        }
    }
    let after_alive = game.pop.in_city(game.city, year).len();
    let _ = (before_alive, after_alive);
    game.alterations += out.len() as u32;
    game.timeline += 1;
    game.cases[pi].solved = true;
    game.cases[pi].correct = correct;
    game.cases[pi].layers = layers;
    game.cases[pi].timeline = game.timeline;
    // record in world memory
    game.memory.push(crate::state::MemoryRecord {
        timeline: game.timeline,
        year,
        city: game.city,
        subject: format!("CASO {:02} — {}", def.id, def.title),
        status: if correct { "RESOLVIDO".into() } else { "RESOLUÇÃO FALSA".into() },
        killed_by: None,
        witnesses: 0,
        consequences: out.clone(),
    });
    game.papers.push(crate::state::Headline {
        day,
        year,
        city: game.city,
        timeline: game.timeline,
        title: o.headline.to_string(),
        body: o.text.to_string(),
    });
    game.news_ticker.push(o.radio.to_string());
    // a false resolution shakes the timeline: Elias' own family may vanish
    if !correct {
        let family: Vec<Pid> = game.pop.people.iter().filter(|p| p.elias_child && p.alive() && p.city == game.city).map(|p| p.id).collect();
        for k in family {
            if rng.chance(0.5) {
                let name = game.pop.get(k).name();
                game.pop.get_mut(k).life = Life::Unborn;
                out.push(format!("{} deixou de existir.", name));
                game.write(format!("Procurei {} pela casa inteira. Ninguém lembra. Nem a mãe.", name), true);
            }
        }
    }
    game.player.money += def.reward * if correct { 1 } else { 0 } + layers as i32 * 15;
    out
}

// ------------------------------------------------------------------ 3D markers

#[derive(Component)]
pub struct ClueMarker {
    pub idx: u8,
}

#[derive(Component)]
pub struct EchoMarker {
    pub idx: u8,
}

#[derive(Resource, Default)]
pub struct CaseRt {
    pub spawned: Option<(u8, u32, i32)>,
    pub red_sight: bool,
    pub echo: Option<EchoRun>,
    pub echo_pos: Vec<Vec2>,
    pub caption: Option<(String, f32)>,
    pub respawn: bool,
}

pub struct EchoRun {
    pub idx: u8,
    pub t: f32,
    pub dur: f32,
    pub origin: Vec2,
    pub ghost: Option<Entity>,
    pub frame: usize,
}

pub fn clue_mesh(look: Look3d) -> crate::render::mesh::MB {
    use crate::render::mesh::{c3, MB};
    let mut m = MB::new();
    match look {
        Look3d::Blood => {
            m.floor(-0.4, -0.3, 0.4, 0.35, 0.115, c3([0.35, 0.02, 0.03]));
            m.floor(0.3, 0.2, 0.55, 0.45, 0.116, c3([0.3, 0.02, 0.03]));
        }
        Look3d::Paper => {
            m.cuboid(Vec3::new(-0.15, 0.0, -0.2), Vec3::new(0.15, 0.02, 0.2), c3([0.85, 0.82, 0.72]));
            m.cuboid(Vec3::new(-0.1, 0.02, -0.12), Vec3::new(0.08, 0.025, -0.1), c3([0.2, 0.2, 0.2]));
        }
        Look3d::Weapon => {
            m.cuboid(Vec3::new(-0.3, 0.0, -0.03), Vec3::new(0.25, 0.05, 0.03), c3([0.35, 0.22, 0.12]));
            m.cuboid(Vec3::new(0.2, 0.0, -0.12), Vec3::new(0.35, 0.05, 0.12), c3([0.5, 0.5, 0.52]));
        }
        Look3d::Object | Look3d::Photo => {
            m.cuboid(Vec3::new(-0.15, 0.0, -0.15), Vec3::new(0.15, 0.2, 0.15), c3([0.4, 0.3, 0.25]));
        }
        Look3d::Prints => {
            for i in 0..5 {
                let z = -0.8 + i as f32 * 0.4;
                let x = if i % 2 == 0 { -0.1 } else { 0.1 };
                m.floor(x - 0.06, z - 0.12, x + 0.06, z + 0.12, 0.115, c3([0.15, 0.1, 0.08]));
            }
        }
        Look3d::Glow | Look3d::None => {
            m.cuboid(Vec3::new(-0.1, 0.0, -0.1), Vec3::new(0.1, 0.12, 0.1), c3([0.5, 0.1, 0.15]));
        }
    }
    m
}

/// Consistency check of every case definition (indices, reachability of clues).
pub fn validate_all() -> Vec<String> {
    use crate::cases::defs::*;
    let mut errs = Vec::new();
    let cases = crate::cases::all_cases();
    let mut ids: Vec<u8> = Vec::new();
    for c in &cases {
        let tag = format!("caso {:02} '{}'", c.id, c.title);
        let mut e = |m: String| errs.push(format!("{}: {}", tag, m));
        if ids.contains(&c.id) {
            e("id repetido".into());
        }
        ids.push(c.id);
        let nc = c.cast.len();
        let nk = c.clues.len();
        let place_ok = |p: &Place| match p {
            Place::HomeOf(i) | Place::WorkOf(i) => (*i as usize) < nc,
            _ => true,
        };
        if !place_ok(&c.start) {
            e("start aponta para elenco inexistente".into());
        }
        if (c.culprit as usize) >= nc {
            e("culprit fora do elenco".into());
        }
        if c.method as usize >= c.methods.len() || c.motive as usize >= c.motives.len() || c.anomaly as usize >= c.anomalies.len() {
            e("method/motive/anomaly fora das opções".into());
        }
        if c.anomaly == 0 {
            e("anomalia correta não pode ser a 0".into());
        }
        for (a, b) in c.threads {
            if *a as usize >= nc || *b as usize >= nc {
                e(format!("thread ({},{}) fora do elenco", a, b));
            }
        }
        let mut revealed = vec![false; nk];
        for (ci, p) in c.cast.iter().enumerate() {
            if !place_ok(&p.home) {
                e(format!("home de {} inválida", p.first));
            }
            let keys: Vec<&str> = p.topics.iter().map(|t| t.key).collect();
            for t in &p.topics {
                for r in t.reveals {
                    if *r as usize >= nk {
                        e(format!("{}/{} revela pista {} inexistente", p.first, t.key, r));
                    } else {
                        revealed[*r as usize] = true;
                    }
                }
                if let Some(b) = t.breaker {
                    if b as usize >= nk {
                        e(format!("{}/{} breaker {} inexistente", p.first, t.key, b));
                    }
                }
                match t.req {
                    Req::Clue(i) if i as usize >= nk => e(format!("{}/{} req pista {} inexistente", p.first, t.key, i)),
                    Req::Both(a, b) if a as usize >= nk || b as usize >= nk => e(format!("{}/{} req Both inexistente", p.first, t.key)),
                    Req::Topic(k) if !keys.contains(&k) => e(format!("{}/{} req tópico '{}' inexistente", p.first, t.key, k)),
                    _ => {}
                }
            }
            if p.dead && !p.topics.is_empty() {
                e(format!("{} está morto mas tem tópicos (idx {})", p.first, ci));
            }
        }
        for ec in &c.echoes {
            if !place_ok(&ec.place) {
                e("eco com lugar inválido".into());
            }
            if ec.unlocks as usize >= nk {
                e(format!("eco desbloqueia pista {} inexistente", ec.unlocks));
            } else {
                revealed[ec.unlocks as usize] = true;
            }
        }
        for (k, cl) in c.clues.iter().enumerate() {
            for p in cl.points {
                if *p as usize >= nc {
                    e(format!("pista {} aponta para elenco {} inexistente", k, p));
                }
            }
            match cl.kind {
                ClueKind::Testimony => {
                    if !revealed[k] {
                        e(format!("depoimento {} '{}' nunca é revelado", k, cl.name));
                    }
                }
                ClueKind::Temporal => {
                    if !revealed[k] {
                        e(format!("pista temporal {} '{}' sem eco", k, cl.name));
                    }
                    if cl.place.is_none() {
                        e(format!("pista temporal {} sem lugar", k));
                    }
                }
                _ => {
                    match &cl.place {
                        None => e(format!("pista {} '{}' sem lugar", k, cl.name)),
                        Some(p) if !place_ok(p) => e(format!("pista {} lugar inválido", k)),
                        _ => {}
                    }
                }
            }
        }
        for (i, f) in c.on_true.fates.iter().chain(c.on_false.fates.iter()) {
            if *i as usize >= nc {
                e(format!("destino para elenco {} inexistente", i));
            }
            if !["jailed", "dead", "left_city", "police", "criminal", "grateful", "survived", "journalist", "hates_elias"].contains(f) {
                e(format!("destino '{}' desconhecido", f));
            }
        }
    }
    errs
}
