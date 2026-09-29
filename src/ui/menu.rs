//! Title screen: an investigative case file, typewriter, tape noise.

use super::*;
use crate::save::SaveMeta;

pub fn build_title(saves: &[SaveMeta], blink: bool, secret: bool) -> El {
    let has_save = !saves.is_empty();
    let line = "────────────────────────────";
    let mut kids = vec![
        mono(line, 18.0, FAINT),
        space(18.0),
        txt("RED THREAD", 64.0, INK, Fnt::MonoB),
        txt("F I O   V E R M E L H O", 16.0, RED, Fnt::Mono),
        space(6.0),
        txt("Todo mistério deixa uma marca.", 18.0, DIM, Fnt::SerifI),
        space(26.0),
    ];
    let item = |id: &str, label: &str, enabled: bool| -> El {
        El::Btn {
            id: id.into(),
            sty: Sty { pad: UiRect::axes(Val::Px(18.0), Val::Px(5.0)), ..default() },
            kids: vec![txt(label, 21.0, if enabled { INK } else { FAINT }, Fnt::Mono)],
            active: false,
        }
    };
    if has_save {
        kids.push(item("t_continue", "CONTINUAR", true));
    }
    kids.push(item("t_new", "NOVA INVESTIGAÇÃO", true));
    kids.push(item("t_load", "ARQUIVOS DE CASO", has_save));
    kids.push(item("t_settings", "AJUSTES", true));
    kids.push(item("t_controls", "CONTROLES", true));
    if secret {
        kids.push(item("t_break", "QUEBRAR O FIO", true));
    }
    kids.push(item("t_quit", "SAIR", true));
    kids.push(space(26.0));
    kids.push(mono(line, 18.0, FAINT));
    kids.push(space(8.0));
    kids.push(mono(if blink { "1920 _" } else { "1920  " }, 18.0, RED));
    El::Col(
        Sty { w: Val::Percent(100.0), h: Val::Percent(100.0), align: AlignItems::Center, justify: JustifyContent::Center, bg: Some(Color::srgba(0.0, 0.0, 0.0, 0.35)), ..default() },
        vec![col(kids).align(AlignItems::Center).gap(4.0)],
    )
}
