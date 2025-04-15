use std::sync::{Arc, Mutex};

pub struct Config {
    pub default_position: bool,
    pub api_key: String,
}

pub struct ApiContext {
    pub config: Config,
    pub shared: Arc<Mutex<SharedContext>>,
}

#[derive(Default)]
pub struct SharedContext {
    pub gtk_application: Option<gtk4::Application>,
    pub gtk_main_window: Option<gtk4::ApplicationWindow>,
}
unsafe impl Send for SharedContext {}
