use std::{ffi::CString, path::Path, sync::{Arc, Mutex}};

use gtk4::{gdk_pixbuf::Pixbuf, gio::prelude::{ApplicationExt, ApplicationExtManual}, glib::{self}, prelude::GtkWindowExt, Application, ApplicationWindow};

use crate::context::SharedContext;

pub fn run_display(shared: Arc<Mutex<SharedContext>>) -> glib::ExitCode {
    let app = Application::builder()
        .application_id("com.hermod.main")
        .build();
    let sh_move = shared.clone();

    shared.lock().unwrap().gtk_application = Some(app.clone());
    app.connect_activate(move |app: &Application| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hermod - Main display")
            .build();

        sh_move.lock().unwrap().gtk_main_window = Some(window.clone());
        window.present();
    });
    app.run()
}
