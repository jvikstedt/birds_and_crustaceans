use bevy::{
    app::Events, core::FixedTimestep, diagnostic::FrameTimeDiagnosticsPlugin,
    ecs::schedule::ShouldRun, prelude::*,
};
use bevy_kira_audio::AudioPlugin;
use bevy_networking_turbulence::{LinkConditionerConfig, NetworkingPlugin};
use bevy_prototype_lyon::plugin::ShapePlugin;
use component::{Checksum, Collider, Health, Player, PositionHistory};
use event::CollisionEvent;
use menu::MenuPlugin;
use resource::{AudioHandles, FrameInfo, MouseInfo, NetworkIdProvider, Opt, RemoteFrames, Scores};
use shared::message::{ClientState, PlayerInput};
use structopt::StructOpt;

use rollback::{RollbackApp, RollbackPlugin};

#[cfg(feature = "inspectable")]
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorParams, WorldInspectorPlugin};

mod component;
mod event;
mod menu;
mod resource;
mod rollback;
mod system;

const STAGE_ROLLBACK_INIT: &str = "rollback_init";
const STAGE_ROLLBACK_ACTIONS: &str = "rollback_actions";
const STAGE_ROLLBACK_COLLISION: &str = "rollback_collision";
const STAGE_ROLLBACK_CLEANUP: &str = "rollback_cleanup";
const STAGE_EVERY_DEFAULT: &str = "every_default";

const SYSTEM_LABEL_COLLISION: &str = "collision";
const SYSTEM_LABEL_SPAWN: &str = "spawn";
const SYSTEM_LABEL_SPAWN_PLAYERS: &str = "spawn_players";
const SYSTEM_LABEL_DESPAWN_PLAYERS: &str = "despawn_players";
const SYSTEM_LABEL_BUILD_MAP: &str = "build_map";

const SYSTEM_LABEL_UPDATE_INPUTS: &str = "update_inputs";
const SYSTEM_LABEL_APPLY_INPUTS: &str = "apply_inputs";
const SYSTEM_LABEL_UPDATE_MOUSE_INFO: &str = "update_mouse_info";

const TIMESTEP_1_PER_SECOND: f64 = 1.;

fn run_if_confirmed(frame_info: ResMut<FrameInfo>) -> ShouldRun {
    if frame_info.confirmed {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn main() {
    let opt = Opt::from_args();

    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: "Birds And Crustaceans - Client".to_string(),
        width: 1024.,
        height: 768.,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(RollbackPlugin::new(opt.input_lag))
    .add_plugin(MenuPlugin::new())
    .add_plugin(AudioPlugin)
    .add_plugin(ShapePlugin)
    .register_rollback_type::<PlayerInput>()
    .register_rollback_type::<Collider>()
    .register_rollback_type::<PositionHistory>()
    .register_rollback_type::<Health>()
    .register_rollback_type::<Checksum>()
    .register_rollback_type::<Player>()
    .init_resource::<Events<CollisionEvent>>()
    .insert_resource(opt)
    .insert_resource(RemoteFrames::new(1))
    .insert_resource(Scores::new())
    .insert_resource(MouseInfo::default())
    .insert_resource(AudioHandles::default())
    .insert_rollback_resource(NetworkIdProvider::default())
    .add_plugin(NetworkingPlugin {
        link_conditioner: if cfg!(debug_assertions) {
            Some(LinkConditionerConfig::good_condition())
        } else {
            None
        },
        idle_timeout_ms: Some(10000),
        ..Default::default()
    })
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_state(ClientState::MainMenu)
    .add_startup_system(system::setup_debug_window)
    .add_startup_system(shared::system::setup_network)
    .add_system_set(SystemSet::on_enter(ClientState::Connecting).with_system(system::setup_client))
    .add_system_set(
        SystemSet::on_enter(ClientState::Loading)
            .with_system(system::audio::setup_audio_handles)
            .with_system(system::build_map.label(SYSTEM_LABEL_BUILD_MAP)),
    )
    .add_system_set(SystemSet::on_update(ClientState::Loading).with_system(system::loading))
    .add_system_set(
        SystemSet::on_enter(ClientState::InGame)
            .with_system(system::audio::start_background_audio)
            .with_system(system::setup_score_window)
            .with_system(system::setup_stats_window)
            .with_system(system::setup_cursor),
    )
    .add_system_set(
        SystemSet::on_update(ClientState::InGame)
            .with_system(system::update_mouse_info.label(SYSTEM_LABEL_UPDATE_MOUSE_INFO))
            .with_system(system::move_cursor.after(SYSTEM_LABEL_UPDATE_MOUSE_INFO))
            .with_system(system::update_score_window)
            .with_system(system::update_stats_window),
    )
    .with_input_system(system::input)
    .with_rollback_schedule(
        Schedule::default()
            .with_stage(
                STAGE_ROLLBACK_INIT,
                SystemStage::parallel().with_system_set(
                    SystemSet::new()
                        .label(SYSTEM_LABEL_SPAWN)
                        .with_run_criteria(run_if_confirmed)
                        .with_system(system::spawn_players.label(SYSTEM_LABEL_SPAWN_PLAYERS))
                        .with_system(
                            system::despawn_players
                                .label(SYSTEM_LABEL_DESPAWN_PLAYERS)
                                .after(SYSTEM_LABEL_SPAWN_PLAYERS),
                        )
                        .with_system(system::ai::spawn_enemy.after(SYSTEM_LABEL_DESPAWN_PLAYERS))
                        .with_system(
                            system::update_player_size.after(SYSTEM_LABEL_DESPAWN_PLAYERS),
                        ),
                ),
            )
            .with_stage(
                STAGE_ROLLBACK_ACTIONS,
                SystemStage::parallel()
                    .with_system_set(SystemSet::new().with_system(system::ai::actions::wander))
                    .with_system_set(
                        SystemSet::new()
                            .with_system(system::update_inputs.label(SYSTEM_LABEL_UPDATE_INPUTS))
                            .with_system(
                                system::apply_inputs
                                    .label(SYSTEM_LABEL_APPLY_INPUTS)
                                    .after(SYSTEM_LABEL_UPDATE_INPUTS),
                            )
                            .with_system(system::hit.after(SYSTEM_LABEL_APPLY_INPUTS)),
                    ),
            )
            .with_stage(
                STAGE_ROLLBACK_COLLISION,
                SystemStage::parallel()
                    .with_system_set(
                        SystemSet::new()
                            .label(SYSTEM_LABEL_COLLISION)
                            .with_system(system::check_collision),
                    )
                    .with_system_set(
                        SystemSet::new()
                            .after(SYSTEM_LABEL_COLLISION)
                            .with_system(system::handle_hits)
                            .with_system(system::update_position_history),
                    ),
            )
            .with_stage(
                STAGE_ROLLBACK_CLEANUP,
                SystemStage::parallel()
                    .with_system(system::clear_events)
                    .with_system(system::calculate_checksum)
                    .with_system(system::death)
                    .with_system(system::mark_to_delete),
            ),
    )
    .with_every(Schedule::default().with_stage(
        STAGE_EVERY_DEFAULT,
        SystemStage::parallel().with_system(system::move_players),
    ))
    .add_system(system::handle_client_events)
    .add_system(system::debug_input)
    .add_system(system::update_health_bar)
    .add_system_set(
        SystemSet::new()
            .with_run_criteria(FixedTimestep::step(TIMESTEP_1_PER_SECOND))
            .with_system(system::update_debug_window),
    );

    #[cfg(feature = "inspectable")]
    app.add_plugin(WorldInspectorPlugin::new())
        .insert_resource(WorldInspectorParams {
            despawnable_entities: true,
            highlight_changes: true,
            ..Default::default()
        })
        .register_inspectable::<component::EntityType>()
        .register_inspectable::<component::NetworkEntity>()
        .register_inspectable::<component::Player>()
        .register_inspectable::<component::Collider>();

    app.run();
}
