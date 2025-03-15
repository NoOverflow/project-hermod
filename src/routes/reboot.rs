use crate::middlewares::authentication::ApiKey;
use rocket::{get, http::Status, State};
use rocket_okapi::openapi;

/// # Reboot the controller
///
/// This route is used to trigger a reboot of the controller.
#[openapi(tag = "Power")]
#[get("/reboot")]
pub async fn route_reboot(
    _context: &State<crate::context::Context>,
    _key: ApiKey,
) -> Status {
    Status::Ok
}
