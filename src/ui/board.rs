//! The case board: victims, suspects, evidence cards, red threads that the
//! player ties between evidence and people, and the deduction panel.
//! The game never says "wrong".

use super::*;
use crate::cases::defs::Role;
use crate::cases::run::{current, CaseDb};
use crate::state::{Game, Skill};

#[derive(Resource, Default)]
pub struct BoardSel {
    pub clue: Option<u8>,
    /// set when the board was opened to formally accuse
    pub accusing: bool,
    pub confront_pid: Option<u32>,
}

pub fn build_board(game: &Game, db: &CaseDb, ui: &UiState, sel: &BoardSel) -> El {
    let Some((def, pi)) = current(game, db) else {
        return modal(panel(vec![title("QUADRO DO CASO"), t("Nenhuma investigação ativa."), mono("[Tab] fechar", 13.0, FAINT)]));
    };
    let prog = &game.cases[pi];
    let tabs = ["QUADRO", "DEDUÇÃO"];
    let mut tab_row = vec![];
    for (i, tt) in tabs.iter().enumerate() {
        tab_row.push(btn(format!("tab:{}", i), *tt).active(ui.tab == i));
    }
    let header = row(vec![
        col(vec![txt(def.title.to_uppercase(), 26.0, PAPER_INK, Fnt::MonoB), mono(format!("{} — {}", def.city.upper(), prog.year), 14.0, Color::srgb(0.35, 0.1, 0.1))]),
        grow(),
        row(tab_row).gap(6.0),
    ]);
    let cork = Color::srgb(0.36, 0.26, 0.17);
    let card = |kids: Vec<El>, tint: f32, id: String, active: bool| -> El {
        El::Btn {
            id,
            sty: Sty {
                w: Val::Px(200.0),
                pad: UiRect::all(Val::Px(8.0)),
                bg: Some(Color::srgb(0.86 * tint, 0.82 * tint, 0.72 * tint)),
                border: Some(if active { RED } else { Color::srgb(0.3, 0.25, 0.2) }),
                border_w: if active { 3.0 } else { 1.0 },
                margin: UiRect::all(Val::Px(4.0)),
                ..default()
            },
            kids: vec![col(kids).gap(2.0)],
            active,
        }
    };
    if ui.tab == 0 {
        // people column
        let mut people = vec![];
        for (i, c) in def.cast.iter().enumerate() {
            let pid = prog.cast[i];
            let p = game.pop.get(pid);
            let met = p.elias.met || c.role == Role::Victim;
            if !met && !prog.topics.iter().any(|(ci, _)| *ci as usize == i) && c.role != Role::Victim && !def.clues.iter().enumerate().any(|(k, cl)| prog.has(k as u8) && cl.points.contains(&(i as u8))) {
                continue;
            }
            let role = match c.role {
                Role::Victim => "VÍTIMA",
                Role::Suspect => "SUSPEITO",
                Role::Witness => "TESTEMUNHA",
                Role::Contact => "CONTATO",
            };
            // the red threads the player tied to this person
            let links: Vec<String> = prog.links.iter().filter(|(_, ci)| *ci as usize == i).map(|(cl, _)| format!("● {}", def.clues[*cl as usize].name)).collect();
            let mut k = vec![
                mono(role, 11.0, Color::srgb(0.5, 0.1, 0.1)),
                txt(p.name(), 16.0, PAPER_INK, Fnt::SerifB),
                wrap(c.desc, 12.0, Color::srgb(0.3, 0.26, 0.22), Fnt::Serif, 180.0),
            ];
            let lies = prog.lies_heard.iter().filter(|(ci, _)| *ci as usize == i).count();
            let broken = prog.lies_broken.iter().filter(|(ci, _)| *ci as usize == i).count();
            if lies > broken {
                k.push(mono("mentiu para você", 11.0, RED));
            }
            for l in links.iter().take(5) {
                k.push(mono(l.clone(), 11.0, Color::srgb(0.7, 0.05, 0.08)));
            }
            let dead = !p.alive();
            people.push(card(k, if dead { 0.8 } else { 1.0 }, format!("person:{}", i), false));
        }
        // evidence column
        let mut ev = vec![];
        for c in prog.found.iter() {
            let cl = &def.clues[*c as usize];
            let active = sel.clue == Some(*c);
            let mut k = vec![mono(format!("{} #{:02}", cl.kind.label(), c + 1), 11.0, Color::srgb(0.5, 0.1, 0.1)), txt(cl.name, 15.0, PAPER_INK, Fnt::SerifB)];
            if active {
                k.push(wrap(cl.desc, 12.0, Color::srgb(0.25, 0.22, 0.2), Fnt::Serif, 180.0));
                if game.player.skill(Skill::Forensics) >= 2 {
                    if let Some(f) = cl.forensic {
                        k.push(wrap(format!("Perícia: {}", f), 12.0, Color::srgb(0.35, 0.08, 0.08), Fnt::SerifI, 180.0));
                    }
                }
            }
            ev.push(card(k, 0.95, format!("clue:{}", c), active));
        }
        if ev.is_empty() {
            ev.push(wrap("Nenhuma evidência ainda. Examine a cena do crime e converse com as pessoas.", 15.0, PAPER, Fnt::SerifI, 600.0));
        }
        let hint = if sel.clue.is_some() {
            "Agora clique numa pessoa para amarrar o fio vermelho (ou na mesma evidência para soltar)."
        } else {
            "Clique numa evidência e depois numa pessoa para ligá-las com o fio vermelho. O quadro não diz se você está certo."
        };
        let body = row(vec![
            col(vec![mono("PESSOAS", 13.0, PAPER), El::Row(Sty { wrap: true, w: Val::Px(440.0), ..default() }, people)]).gap(4.0),
            col(vec![mono(format!("EVIDÊNCIAS ({})", prog.found.len()), 13.0, PAPER), El::Row(Sty { wrap: true, w: Val::Px(640.0), ..default() }, ev)]).gap(4.0),
        ])
        .gap(10.0)
        .align(AlignItems::FlexStart);
        return modal(
            col(vec![header.bg(PAPER).pad(10.0), col(vec![body]).h(520.0).scroll(), mono(hint, 13.0, PAPER), mono("[Tab] fechar", 12.0, Color::srgb(0.7, 0.6, 0.5))])
                .bg(cork)
                .pad(14.0)
                .gap(8.0)
                .w(1180.0),
        );
    }
    // deduction
    let mut sus = vec![];
    for (i, c) in def.cast.iter().enumerate() {
        if c.role == Role::Victim {
            continue;
        }
        let p = game.pop.get(prog.cast[i]);
        sus.push(opt(format!("acc:{}", i), p.name(), if prog.accused == Some(i as u8) { RED } else { PAPER_INK }).active(prog.accused == Some(i as u8)));
    }
    let mut meth = vec![];
    for (i, m) in def.methods.iter().enumerate() {
        meth.push(opt(format!("meth:{}", i), *m, if prog.method == Some(i as u8) { RED } else { PAPER_INK }).active(prog.method == Some(i as u8)));
    }
    let mut mot = vec![];
    for (i, m) in def.motives.iter().enumerate() {
        mot.push(opt(format!("mot:{}", i), *m, if prog.motive == Some(i as u8) { RED } else { PAPER_INK }).active(prog.motive == Some(i as u8)));
    }
    let temporal = prog.found.iter().any(|c| matches!(def.clues[*c as usize].kind, crate::cases::defs::ClueKind::Temporal) || def.clues[*c as usize].look == crate::cases::defs::Look3d::Glow);
    let mut an = vec![];
    if temporal {
        for (i, m) in def.anomalies.iter().enumerate() {
            an.push(opt(format!("anom:{}", i), *m, if prog.sphere == Some(i as u8) { RED } else { PAPER_INK }).active(prog.sphere == Some(i as u8)));
        }
    } else {
        an.push(wrap("Você ainda não encontrou nada que pareça... fora do tempo.", 14.0, Color::srgb(0.35, 0.3, 0.25), Fnt::SerifI, 480.0));
    }
    let section = |t: &str, kids: Vec<El>| col(vec![mono(t, 13.0, Color::srgb(0.5, 0.1, 0.1)), col(kids).gap(1.0)]).bg(PAPER).pad(10.0).gap(4.0);
    let mut foot = vec![];
    if sel.accusing {
        foot.push(btn("accuse_final", "ACUSAR FORMALMENTE").border(RED));
        foot.push(wrap("Depois da acusação, não há volta. A linha do tempo vai mudar.", 14.0, PAPER, Fnt::SerifI, 600.0));
    } else {
        foot.push(wrap("Para acusar: fale com o detetive na delegacia, ou confronte o suspeito cara a cara (opção \"Eu sei o que você fez\").", 14.0, PAPER, Fnt::SerifI, 900.0));
    }
    modal(
        col(vec![
            header.bg(PAPER).pad(10.0),
            row(vec![
                col(vec![section("QUEM?", sus), section("CONEXÃO COM A ESFERA", an)]).gap(8.0).w(520.0),
                col(vec![section("COMO?", meth), section("POR QUÊ?", mot)]).gap(8.0).w(620.0),
            ])
            .gap(10.0)
            .align(AlignItems::FlexStart),
            row(foot).gap(12.0),
            mono("[Tab] fechar", 12.0, Color::srgb(0.7, 0.6, 0.5)),
        ])
        .bg(cork)
        .pad(14.0)
        .gap(8.0)
        .w(1180.0),
    )
}
