use eframe::egui;

struct MyApp;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.add_space(ui.available_height() / 2.0 - 15.0);

                if ui.button("Click here !").clicked() {
                    println!("Immortal Killer Move, Hakurei Barrier !");
                }
            });
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Gensokyo Realm",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyApp))),
    )
    .expect("Immortal Killer Move failed !");
}
