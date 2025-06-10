use std::{fs, path::Path};

use gtk4::{gdk_pixbuf::Pixbuf, glib::{self, ControlFlow}, prelude::{GtkApplicationExt, GtkWindowExt}};
use middlewares::authentication::ApiKey;
use rocket::{post, http::Status, State};
use rocket_okapi::openapi;
use tower_sanitize_path::SanitizePathLayer;
use file_type::FileType;

use crate::api::middlewares;

fn load_image(file_path: &Path, window: &gtk4::ApplicationWindow) {
    let pixbuf: Result<Pixbuf, glib::Error> = Pixbuf::from_file(file_path);

    match pixbuf {
        Ok(pixbuf) => {
            log::info!("Loaded image: {:?}", pixbuf);
            window.set_child(Some(
                &gtk4::Picture::for_pixbuf(&pixbuf)
            ));
        },
        Err(err) => {
            log::error!("Error loading image: {}", err);
            return;
        }
    }
}

fn load_video(file_path: &Path, window: &gtk4::ApplicationWindow) {
    window.set_child(Some(
        &gtk4::Video::for_filename(Some(file_path))
    ));
}

fn get_file_type(file_path: &Path) -> Option<FileType> {
    let file_type: Result<&'static FileType, file_type::Error> = FileType::try_from_file(file_path);

    match file_type {
        Ok(ft) => Some(ft.clone()),
        Err(_) => {
            None
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
                    match get_file_type(&mov_path) {
                        Some(file_type) => {
                            let extension = file_type.extensions().first().unwrap_or(&"unknown");

                            log::info!("Switching display to: {}, type: {}", mov_path.display(), file_type.name());
                            if ["jpg", "bmp"].contains(extension) {
                                load_image(mov_path, &(mov_ctx.gtk_main_window.clone().unwrap()));
                            } else if ["mp4", "m4v", "m4a", "f4v"].contains(extension) {
                                load_video(mov_path, &(mov_ctx.gtk_main_window.clone().unwrap()));
                            } else {
                                log::error!("Unsupported file type: {}", extension);
                                mov_path.clear();
                                return ControlFlow::Break;
                            }
                            mov_path.clear();
                            return ControlFlow::Break;
                        },
                        None => {
                            log::error!("Unsupported file type for: {}", mov_path.display());
                            return ControlFlow::Break;
                        }
                    }
                }
                mov_path.clear();
                return ControlFlow::Continue;
            },
            Err(_) => {
                log::error!("Failed to lock context");
                mov_path.clear();
                return ControlFlow::Continue;
            }
        }
    });
    return Status::Ok;
}
