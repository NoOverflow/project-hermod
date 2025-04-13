use crate::middlewares::authentication::ApiKey;
use rocket::{get, http::Status, State};
use rocket_okapi::openapi;
use system_shutdown::reboot;

/// # Reboot the controller
///
/// This route is used to trigger a reboot of the controller.
#[openapi(tag = "Power")]
#[get("/reboot")]
pub async fn route_reboot(_context: &State<crate::context::Context>, _key: ApiKey) -> Status {
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
    Status::InternalServerError
}
