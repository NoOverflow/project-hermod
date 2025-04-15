use std::sync::{Arc, Mutex};
use std::{env, thread};

use anyhow::Result;
use context::{Config, SharedContext};
use log::{error, info};
use rocket::{futures, tokio};
use rocket_okapi::{openapi_get_routes, swagger_ui::*};
use routes::reboot::*;

use futures::executor::block_on;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use gtk4 as gtk;

mod context;
mod middlewares;
mod routes;

fn setup_logger() -> Result<(), log::SetLoggerError> {
    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339(std::time::SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(if cfg!(debug_assertions) {
            // TODO: Add a flag to enable debug logging
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Debug
        })
        .chain(std::io::stdout())
        .apply()
}

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

#[tokio::main]
async fn run_api(shared: Arc<Mutex<SharedContext>>) -> Result<(), ()> {
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

    let launch_result = rocket::build()
        .mount("/api", openapi_get_routes![route_reboot])
        .mount(
            "/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "/api/openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .manage(context)
        .launch()
        .await;
    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    };
    Ok(())
}

fn run_display(shared: Arc<Mutex<SharedContext>>) -> glib::ExitCode {
    let app = Application::builder()
        .application_id("com.hermod.main")
        .build();

    shared.lock().unwrap().gtk_application = Some(app.clone());
    app.connect_activate(|app: &Application| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hermod - Main display")
            .build();

        window.present();
    });
    app.run()
}

fn main() {
    let shared = Arc::new(Mutex::new(context::SharedContext::default()));
    let api_context = shared.clone();

    if setup_logger().is_err() {
        eprintln!("Failed to setup logger, exiting.");
    }
    thread::spawn(move || run_api(api_context));
    run_display(shared.clone());
}
