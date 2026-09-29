//! Esc menu: continue, save, load, settings, controls (rebinding), quit.

use super::*;
use crate::keys::{key_name, Action, Bindings, Settings};
use crate::save::SaveMeta;

pub fn build_pause(ui: &UiState, settings: &Settings, binds: &Bindings, saves: &[SaveMeta], title_screen: bool) -> El {
    match ui.sub {
        1 | 2 => {
            let saving = ui.sub == 1;
            let mut kids = vec![title(if saving { "SALVAR — ARQUIVOS DE CASO" } else { "CARREGAR — ARQUIVOS DE CASO" })];
            kids.push(hr());
            let mut slots: Vec<u32> = if saving { (1..=8).collect() } else { saves.iter().map(|s| s.slot).collect() };
            slots.sort();
            if slots.is_empty() {
                kids.push(t("Nenhum arquivo encontrado."));
            }
            for s in slots {
                let meta = saves.iter().find(|m| m.slot == s);
                let label = match meta {
                    Some(m) => format!(
                        "CASE FILE {:03}{}   {} {}   {}   {}   linha {}   mortes {}   alterações {}",
                        s,
                        if s == 0 { " (automático)" } else { "" },
                        m.city,
                        m.year,
                        m.case_title,
                        m.status,
                        m.timeline,
                        m.casualties,
                        m.alterations
                    ),
                    None => format!("CASE FILE {:03}   — vazio —", s),
                };
                kids.push(opt(format!("{}:{}", if saving { "save" } else { "load" }, s), label, if meta.is_some() { INK } else { FAINT }));
            }
            kids.push(space(8.0));
            kids.push(btn("back", "Voltar"));
            modal(panel(kids).w(980.0))
        }
        3 => {
            let pct = |v: f32| format!("{:>3.0}%", v * 100.0);
            let line = |label: &str, val: String, dec: &str, inc: &str| row(vec![txt(label.to_string(), 18.0, INK, Fnt::Serif).sty(|_| {}), grow(), btn(dec.to_string(), "−"), space(8.0), mono(val, 17.0, AMBER), space(8.0), btn(inc.to_string(), "+")]).w(620.0);
            let toggle = |label: &str, on: bool, id: &str| row(vec![txt(label.to_string(), 18.0, INK, Fnt::Serif), grow(), btn(id.to_string(), if on { "LIGADO" } else { "DESLIGADO" }).active(on)]).w(620.0);
            modal(
                panel(vec![
                    title("AJUSTES"),
                    hr(),
                    line("Volume geral", pct(settings.master), "master-", "master+"),
                    line("Música", pct(settings.music), "music-", "music+"),
                    line("Efeitos e vozes", pct(settings.sfx), "sfx-", "sfx+"),
                    line("Velocidade da câmera", format!("{:.1}x", settings.cam_speed), "cam-", "cam+"),
                    toggle("Tela cheia", settings.fullscreen, "fullscreen"),
                    toggle("Sombras", settings.shadows, "shadows"),
                    toggle("Neblina", settings.fog, "fog"),
                    toggle("Granulação de filme e vinheta", settings.grain, "grain"),
                    toggle("Gráficos bonitos (sombreamento suave, bordas limpas)", settings.quality >= 1, "quality"),
                    space(8.0),
                    btn("back", "Voltar"),
                ])
                .w(680.0),
            )
        }
        4 => {
            let mut rows = vec![title("CONTROLES"), mono("Clique numa ação e aperte a nova tecla.", 14.0, DIM), hr()];
            let mut left = vec![];
            let mut right = vec![];
            for (i, a) in Action::ALL.iter().enumerate() {
                let waiting = ui.rebinding == Some(*a);
                let el = row(vec![
                    txt(a.label(), 16.0, INK, Fnt::Serif).sty(|_| {}),
                    grow(),
                    btn_w(format!("bind:{}", i), if waiting { "...aperte uma tecla".to_string() } else { key_name(binds.key(*a)) }, 190.0).active(waiting),
                ])
                .w(470.0);
                if i < 13 {
                    left.push(el);
                } else {
                    right.push(el);
                }
            }
            rows.push(row(vec![col(left).gap(3.0), space(20.0), col(right).gap(3.0)]).align(AlignItems::FlexStart));
            rows.push(mono("Mouse: mirar e atirar (com a arma sacada) · Roda: zoom · 1-9: escolher falas", 13.0, DIM));
            rows.push(row(vec![btn("binds_reset", "Restaurar padrão"), space(10.0), btn("back", "Voltar")]));
            modal(panel(rows).w(1020.0))
        }
        _ => {
            let mut kids = vec![txt("RED THREAD", 40.0, INK, Fnt::MonoB), mono("— pausa —", 14.0, RED), hr()];
            if !title_screen {
                kids.push(btn_w("resume", "Continuar", 320.0));
                kids.push(btn_w("to_save", "Salvar", 320.0));
            }
            kids.push(btn_w("to_load", "Carregar", 320.0));
            kids.push(btn_w("to_settings", "Ajustes", 320.0));
            kids.push(btn_w("to_controls", "Controles", 320.0));
            if !title_screen {
                kids.push(btn_w("to_title", "Menu principal", 320.0));
            }
            kids.push(btn_w("quit", "Sair do jogo", 320.0));
            if let Some(m) = &ui.message {
                kids.push(mono(m.clone(), 14.0, AMBER));
            }
            modal(panel(kids).gap(8.0).align(AlignItems::Center).w(420.0))
        }
    }
}
