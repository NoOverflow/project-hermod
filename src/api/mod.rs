use anyhow::Result;
use network_interface::{NetworkInterface, NetworkInterfaceConfig};

use crate::context;

use super::context::{Config, SharedContext};
use log::{error, info, warn};
use rocket::tokio;
use rocket_okapi::{openapi_get_routes, swagger_ui::*};
use routes::reboot::*;
use routes::switch::*;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::env;


pub mod middlewares;
pub mod routes;

fn build_config() -> Result<Config, ()> {
    let env_api_key = env::var("PROJECT_HERMOD_API_KEY");

    if env_api_key.is_err() || env_api_key.clone().unwrap() == "" {
        error!("Invalid API key, did you set the PROJECT_HERMOD_API_KEY environment variable to a non-empty string ?");
        return Err(());
    }
    Ok(Config {
        default_position: false,
        api_key: env_api_key.unwrap(),
    })
}

fn get_listen_address(interface_name: &str, ipv6: bool) -> IpAddr {
    let network_interfaces = NetworkInterface::show().unwrap();

    match network_interfaces.into_iter().find(|itf| itf.name == interface_name) {
        Some(int) => {
            let address = int.addr.get(if ipv6 {1} else {0});

            if address.is_none() {
                warn!("Couldn't find the address for the requested protocol on the interface. Using localhost instead.");
                return IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1))
            }
            address.unwrap().ip()
        },
        None => {
            warn!("Couldn't find interface {}, using localhost instead.", interface_name);
            IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1))
        },
    }
}

#[tokio::main]
pub async fn run_api(shared: Arc<Mutex<SharedContext>>) -> Result<(), ()> {
    let config = build_config();

    if config.is_err() {
        error!("Couldn't build configuration, check logs for error.");
        return Err(());
    }
    let context = context::ApiContext {
        config: config.unwrap(),
        shared,
    };

    info!("Starting moonscale server with context:");
    let rocket_config = rocket::Config {
        port: 8000,
        address: get_listen_address("tailscale0", true),
        ..Default::default()
    };

    let launch_result = rocket::build()
        .mount("/api", openapi_get_routes![route_reboot, route_image_switch])
        .mount(
            "/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "/api/openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .configure(rocket_config)
        .manage(context)
        .launch()
        .await;
    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    };
    Ok(())
}
