#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]
//! RED THREAD — Fio Vermelho. Todo mistério deixa uma marca.

mod audio;
mod camera;
mod cases;
mod city;
mod crime;
mod debug;
mod economy;
mod env;
mod glue;
mod interact;
mod items;
mod keys;
mod narrative;
mod player;
mod pursuit;
mod render;
mod save;
mod sim;
mod state;
mod ui;
mod util;
mod vehicles;
mod voice;
mod world;

use bevy::input::InputSystem;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode};

fn setup(mut c: Commands, mut images: ResMut<Assets<Image>>, mut mats: ResMut<Assets<StandardMaterial>>) {
    let tex = render::textures::make(&mut images);
    let m = render::city3d::make_mats(&mut mats, &tex);
    c.insert_resource(m);
    c.insert_resource(cases::run::CaseDb(cases::all_cases()));
}

/// Start at the title screen (with the limbo and the sphere as backdrop),
/// or jump straight into the game for automated tests (RT_START).
fn boot(mut ui: ResMut<ui::UiState>, mut flow: ResMut<narrative::Flow>, mut game: ResMut<state::Game>, mut music: ResMut<audio::MusicState>) {
    game.phase = state::Phase::Prologue;
    game.set("limbo");
    game.minute = 3.0 * 60.0 + 17.0;
    match std::env::var("RT_START").ok().as_deref() {
        Some("new") => flow.actions.push("new_game".into()),
        Some(s) if s.starts_with("case") => {
            let n: u8 = s[4..].parse().unwrap_or(1);
            flow.actions.push(format!("start_case:{}", n));
        }
        Some("limbo") => flow.actions.push("limbo".into()),
        _ => {
            ui.open(ui::Mode::Title);
            music.want = audio::Track::Menu;
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--dump-map") {
        let city = args.iter().position(|a| a == "--city").and_then(|i| args.get(i + 1)).cloned().unwrap_or_default();
        dump(&city);
        return;
    }
    if args.iter().any(|a| a == "--check-cases") {
        let errs = cases::run::validate_all();
        for e in &errs {
            println!("{}", e);
        }
        println!("{} casos, {} problemas", cases::all_cases().len(), errs.len());
        std::process::exit(if errs.is_empty() { 0 } else { 1 });
    }
    if args.iter().any(|a| a == "--dump-kinds") {
        dump_kinds();
        return;
    }
    if let Some(i) = args.iter().position(|a| a == "--render-audio") {
        let dir = args.get(i + 1).cloned().unwrap_or("audio_out".into());
        audio::render_to_dir(&dir);
        voice::render_samples(&dir);
        let v = audio::render_venue(1920, 7);
        let _ = std::fs::write(format!("{}/venue_1920.wav", dir), audio::wav(&v, audio::SR, false));
        return;
    }
    let settings = keys::Settings::load();
    let fullscreen = settings.fullscreen && std::env::var("RT_SCRIPT").is_err();
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "RED THREAD — Fio Vermelho".into(),
                        resolution: (1280., 720.).into(),
                        present_mode: PresentMode::AutoVsync,
                        mode: if fullscreen { WindowMode::BorderlessFullscreen(MonitorSelection::Current) } else { WindowMode::Windowed },
                        ..default()
                    }),
                    ..default()
                })
                .set(bevy::log::LogPlugin { level: bevy::log::Level::WARN, ..default() }),
        )
        .insert_resource(ClearColor(Color::srgb(0.02, 0.02, 0.04)))
        .insert_resource(AmbientLight { color: Color::srgb(0.5, 0.5, 0.75), brightness: 80.0, ..default() })
        .insert_resource(keys::Script::from_env())
        .insert_resource(settings.clone())
        .insert_resource(settings.bindings.clone())
        .insert_resource(world::LoadCity(true))
        .insert_resource(state::Game::new())
        .init_resource::<keys::Act>()
        .init_resource::<camera::CamState>()
        .init_resource::<camera::Cursor>()
        .init_resource::<render::city3d::CityVis>()
        .init_resource::<sim::agents::Sim>()
        .init_resource::<sim::update::SimEvents>()
        .init_resource::<env::EnvState>()
        .init_resource::<player::PlayerRt>()
        .init_resource::<ui::UiState>()
        .init_resource::<ui::hud::Toasts>()
        .init_resource::<ui::hud::Prompt>()
        .init_resource::<ui::hud::HudMem>()
        .init_resource::<ui::dialogue::Dlg>()
        .init_resource::<ui::board::BoardSel>()
        .init_resource::<ui::screens::Popups>()
        .init_resource::<ui::screens::Choices>()
        .init_resource::<ui::screens::Lockpick>()
        .init_resource::<ui::screens::Shop>()
        .init_resource::<ui::screens::MapImage>()
        .init_resource::<ui::overlay::OverlayMem>()
        .init_resource::<cases::run::CaseRt>()
        .init_resource::<interact::Interact>()
        .init_resource::<crime::CrimeRt>()
        .init_resource::<vehicles::Cars>()
        .init_resource::<narrative::Flow>()
        .init_resource::<narrative::Cutscene>()
        .init_resource::<save::SaveReq>()
        .init_resource::<audio::AudioBank>()
        .init_resource::<audio::MusicState>()
        .init_resource::<pursuit::Hunt>()
        .init_resource::<world::SpawnAgents>()
        .init_resource::<glue::VenueState>()
        .add_event::<audio::Sfx>()
        .add_event::<crime::CrimeEv>()
        .add_event::<crime::SpawnDecal>()
        .add_systems(
            Startup,
            (
                setup,
                ui::load_fonts,
                camera::spawn_camera,
                env::setup_env,
                audio::setup_audio,
                audio::start_venue_gen,
                voice::start_engine,
                ui::hud::setup_hud,
                narrative::setup_cine,
                boot,
            )
                .chain(),
        )
        .add_systems(PreUpdate, (keys::run_script, keys::update_act, ui::ui_scale).chain().after(InputSystem))
        .add_systems(
            Update,
            (
                narrative::flow_system,
                save::save_load_system,
                world::load_city_system,
                world::spawn_agents_system,
                debug::debug_cmds,
                vehicles::spawn_cars,
                env::clock,
                sim::update::sim_update,
                player::player_move,
                crime::player_combat,
                crime::hostile_ai,
                crime::process_crimes,
                crime::police_system,
                crime::flush_rumors,
                pursuit::police_hunt,
                pursuit::hide_self,
                pursuit::fugitive_system,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                interact::find_target,
                interact::do_interact,
                interact::handle_choices,
                interact::lockpick_system,
                vehicles::car_actions,
                vehicles::drive_cars,
                glue::dialogue_effects,
                glue::case_markers,
                glue::echo_system,
                glue::red_sight,
                glue::prologue_interact,
                glue::aim_reactions,
                glue::desperate_voices,
                narrative::fatigue_system,
                crime::player_death,
            )
                .chain()
                .after(pursuit::fugitive_system),
        )
        .add_systems(
            Update,
            (
                camera::camera_follow,
                camera::update_cursor,
                camera::cutaway,
                env::update_env,
                env::update_rain,
                env::player_fill,
                world::sync_agents,
                player::spawn_player_vis,
                player::sync_player,
                render::character::animate_rigs,
                world::animate_doors,
                glue::spawn_sphere,
                glue::animate_sphere,
                crime::spawn_decals,
                crime::fade_fx,
            )
                .chain()
                .after(crime::player_death),
        )
        .add_systems(
            Update,
            (
                ui::overlay::ui_toggles,
                ui::overlay::popup_system,
                ui::overlay::choice_input,
                ui::overlay::pause_input,
                ui::overlay::board_input,
                ui::overlay::misc_input,
                ui::dialogue::dialogue_input,
                ui::button_system,
                ui::overlay::overlay_system,
                ui::hud::update_hud,
                ui::hud::update_bubbles,
                narrative::run_cutscene,
                audio::receive_music,
                audio::music_system,
                audio::play_sfx,
                glue::venue_music,
                voice::voice_watch,
                voice::voice_play,
                keys::script_effects,
            )
                .chain()
                .after(crime::fade_fx),
        )
        .run();
}

/// Lists building kinds and districts of every city (used when writing cases).
fn dump_kinds() {
    use city::gen::CityId::*;
    for (c, y) in [(NewOrleans, 1920), (Chicago, 1924), (Bavaria, 1934), (London, 1941), (Adelaide, 1950), (Berlin, 1961), (Bergen, 1968), (SanFrancisco, 1972), (Portland, 1978), (NewYork, 1986), (LosAngeles, 1994), (NewOrleans, 2001)] {
        let m = city::gen::generate(c, y);
        let mut k: std::collections::BTreeMap<String, usize> = Default::default();
        for b in &m.buildings {
            *k.entry(format!("{:?}", b.kind)).or_default() += 1;
        }
        let d: Vec<&str> = m.districts.iter().map(|d| d.name.as_str()).collect();
        println!("{:?} {}: {:?}\n  distritos: {:?}", c, y, k, d);
    }
}

fn dump(city: &str) {
    use city::gen::CityId;
    use city::map::*;
    let id = match city {
        "chicago" => CityId::Chicago,
        "bavaria" => CityId::Bavaria,
        "london" => CityId::London,
        "adelaide" => CityId::Adelaide,
        "berlin" => CityId::Berlin,
        "bergen" => CityId::Bergen,
        "sf" => CityId::SanFrancisco,
        "portland" => CityId::Portland,
        "ny" => CityId::NewYork,
        "la" => CityId::LosAngeles,
        _ => CityId::NewOrleans,
    };
    let m = city::gen::generate(id, 1930);
    for y in 0..m.h {
        let mut l = String::new();
        for x in 0..m.w {
            let c = if let Some(p) = m.prop_at_tile(x, y) {
                match m.props[p].kind {
                    PKind::Tree => 'T',
                    PKind::LampPost => 'i',
                    PKind::Bed => 'b',
                    PKind::Grave => '+',
                    PKind::Rubble => '%',
                    _ => 'o',
                }
            } else {
                match m.get(x, y) {
                    Tile::Water => '~',
                    Tile::Road => ' ',
                    Tile::Sidewalk => '.',
                    Tile::Alley => ',',
                    Tile::Grass => '"',
                    Tile::Dirt => ':',
                    Tile::Dock => '=',
                    Tile::Rail => 'H',
                    Tile::Field => 'f',
                    Tile::Floor => '_',
                    Tile::Wall => '#',
                    Tile::Window => 'w',
                    Tile::Door => 'D',
                    Tile::Fence => '|',
                    Tile::Gravel => 'g',
                    Tile::Plaza => 'p',
                    Tile::Sand => 's',
                    Tile::Void => 'X',
                }
            };
            l.push(c);
        }
        println!("{}", l);
    }
    println!("{} buildings, {} props, {} lamps", m.buildings.len(), m.props.len(), m.lamps.len());
}
