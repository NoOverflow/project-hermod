use std::sync::{Arc, Mutex};
use std::thread;

use api::run_api;
use gui::run_display;

mod context;
mod api;
mod gui;

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

fn main() {
    let shared = Arc::new(Mutex::new(context::SharedContext::default()));
    let api_context = shared.clone();

    if setup_logger().is_err() {
        eprintln!("Failed to setup logger, exiting.");
    }
    thread::spawn(move || run_api(api_context));
    run_display(shared);
}
