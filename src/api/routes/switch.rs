use std::{fs, path::Path};

use gtk4::{gdk_pixbuf::Pixbuf, glib::{self, ControlFlow}, prelude::{GtkApplicationExt, GtkWindowExt}};
use middlewares::authentication::ApiKey;
use rocket::{post, http::Status, State};
use rocket_okapi::openapi;
use tower_sanitize_path::SanitizePathLayer;

use crate::api::middlewares;

fn load_image(file_path: &Path, window: &gtk4::ApplicationWindow) {
    let pixbuf: Result<Pixbuf, glib::Error> = Pixbuf::from_file(file_path);

    match pixbuf {
        Ok(pixbuf) => {
            println!("Loaded image: {:?}", pixbuf);
            window.set_child(Some(
                &gtk4::Picture::for_pixbuf(&pixbuf)
            ));
        },
        Err(err) => {
            eprintln!("Error loading image: {}", err);
            return;
        }
    }
}

/// # Switch the current image
///
/// This route is used to switch the current image displayed
#[openapi(tag = "Image")]
#[post("/switch?<path>")]
pub async fn route_image_switch(_context: &State<crate::context::ApiContext>, _key: ApiKey, path: &str) -> Status {
    if path.contains("..") || path.contains("/") || path.contains("\\") {
        return Status::Forbidden;
    }

    let mut built_path = std::path::PathBuf::from("./res/");
    built_path.push(std::path::PathBuf::from(path));
    let mov_path = Box::leak(Box::new(built_path.clone()));
    let mov_ctx = _context.shared.clone();

    if !built_path.clone().exists() {
        log::error!("Path does not exist: {}", built_path.display());
        return Status::NotFound;
    }
    glib::idle_add(move || {
        match mov_ctx.lock() {
            Ok(mov_ctx) => {
                if !mov_ctx.gtk_main_window.is_none() {
                    load_image(mov_path, &(mov_ctx.gtk_main_window.clone().unwrap()));
                    mov_path.clear();
                    return ControlFlow::Break;
                }
                return ControlFlow::Continue;
            },
            Err(_) => {
                log::error!("Failed to lock context");
                return ControlFlow::Continue;
            }
        }
    });
    return Status::Ok;
}
