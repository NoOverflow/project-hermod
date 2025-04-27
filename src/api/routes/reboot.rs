use middlewares::authentication::ApiKey;
use gtk4::prelude::{GtkApplicationExt, GtkWindowExt};
use rocket::{get, http::Status, State};
use rocket_okapi::openapi;
use system_shutdown::reboot;

use crate::api::middlewares;

/// # Reboot the controller
///
/// This route is used to trigger a reboot of the controller.
#[openapi(tag = "Power")]
#[get("/reboot")]
pub async fn route_reboot(_context: &State<crate::context::ApiContext>, _key: ApiKey) -> Status {
    match reboot() {
        Ok(_) => {
            log::info!("Rebooting the controller");
            return Status::Ok;
        }
        Err(e) => {
            log::error!("Failed to reboot the controller: {}", e);
            return Status::InternalServerError;
        }
    }
}
