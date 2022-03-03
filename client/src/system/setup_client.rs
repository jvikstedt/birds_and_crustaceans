use bevy::prelude::{info, ResMut};
use bevy_networking_turbulence::NetworkResource;
use shared::SERVER_PORT;
use std::net::SocketAddr;

pub fn setup_client(mut net: ResMut<NetworkResource>) {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            let mut server_address: SocketAddr = if cfg!(debug_assertions) {
                "192.168.1.105:0".parse().unwrap()
            } else {
                "104.131.91.47:0".parse().unwrap()
            };

            server_address.set_port(SERVER_PORT);
        } else {
            let ip_address =
                bevy_networking_turbulence::find_my_ip_address().expect("can't find ip address");
            let server_address = SocketAddr::new(ip_address, SERVER_PORT);
        }
    }

    info!("Starting client {:?}", server_address);
    net.connect(server_address);
}
