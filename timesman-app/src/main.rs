#[macro_use]
mod log;
mod app;
mod config;
mod pane;
mod req;

use std::process::ExitCode;
use std::sync::Arc;
use std::sync::Mutex;

use crate::config::Config;
use eframe::Error;

fn main() -> ExitCode {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    let logs = Arc::new(Mutex::new(vec![]));
    log::register(logs.clone());
    info!("Starting");

    let config = match Config::load_config() {
        Ok(c) => c,
        Err(e) => {
            error!(format!("{}", e));
            return ExitCode::from(1);
        }
    };

    match eframe::run_native(
        "TimesMan",
        options,
        Box::new(|cc| Ok(Box::<app::App>::new(app::App::new(cc, logs)))),
    ) {
        Ok(_) => {}
        Err(e) => {
            error!(format!("{}", e));
            return ExitCode::from(1);
        }
    };

    info!("Closing");

    ExitCode::from(0)
}
