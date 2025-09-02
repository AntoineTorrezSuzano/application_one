#![windows_subsystem = "windows"]
mod app;

use app::MyApp;

#[tokio::main]



async fn main() {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Gensokyo Realm",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyApp::new()))),
    )
    .expect("Immortal Killer Move failed !");
}
