use bevy::{log::LogPlugin, prelude::*};
use bevy_networking_turbulence::{LinkConditionerConfig, NetworkResource, NetworkingPlugin};
use resource::Opt;
use server_stage::{ServerStage, SERVER_UPDATE};
use shared::SERVER_PORT;
use std::net::SocketAddr;
use structopt::StructOpt;

mod component;
mod event;
mod resource;
mod server_stage;
mod system;

fn main() {
    let opt = Opt::from_args();

    let mut app = App::new();

    app.add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin)
        .add_plugin(NetworkingPlugin {
            link_conditioner: if cfg!(debug_assertions) {
                Some(LinkConditionerConfig::good_condition())
            } else {
                None
            },
            ..Default::default()
        })
        .insert_resource(opt)
        .init_resource::<resource::Players>()
        .insert_resource(resource::Frames::new(1))
        .add_stage_before(CoreStage::Update, SERVER_UPDATE, ServerStage::new())
        .insert_resource(resource::PlayerHandleProvider::default())
        .add_startup_system(shared::system::setup_network)
        .add_startup_system(setup)
        .add_system(
            system::handle_server_events
                .system()
                .label("handle_server_events"),
        );

    app.run();
}

fn setup(mut net: ResMut<NetworkResource>) {
    let ip_address =
        bevy_networking_turbulence::find_my_ip_address().expect("can't find ip address");
    let server_address = SocketAddr::new(ip_address, SERVER_PORT);

    info!("Starting server {:?}", server_address);
    net.listen(server_address, None, None);
}
