use std::fs;
use eframe::egui::{self};
// use eframe::egui::{self, response};
// use reqwest::Request;
use rfd::FileDialog;
// use std::collections::HashMap;
use std::sync::{Arc, Mutex};


pub struct MyApp{
    file_content: Option<String>,
    has_uploaded_file: bool,
    hosted_website_url: Arc<Mutex<Option<String>>>,
    pub server_ip: String,
    // hosted_website_url: String
}

impl MyApp {
    pub fn new() -> Self {
        Self {
            file_content: None,
            has_uploaded_file: false,
            hosted_website_url: Arc::new(Mutex::new(None)),
            server_ip: "http://127.0.0.1:5999/deploy".to_string(),
        }
    } 

    fn ui_file_controls(&mut self, ui: &mut egui::Ui) {
        if ui.button("Upload a file").clicked() {
            if let Some(path) = FileDialog::new()
                .add_filter("html", &["html"])
                .set_directory("/")
                .pick_file()
            {
                match fs::read_to_string(path) {
                    Ok(content) => {self.file_content = Some(content); self.has_uploaded_file = true},
                    Err(e) => self.file_content = Some(format!("Failed to read file:\n{}", e))
                }
            }       
        }
    }
    
    fn ui_viewer(&mut self, ui: &mut egui::Ui) {
        if let Some(content) = &self.file_content {
            ui.label("File Content:");
            egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                ui.label(content);
            });
        } else {
            ui.label("No file loaded.");
        }
    }
    fn ui_server_textedit(&mut self, ui: &mut egui::Ui) {
        ui.label("Server's full Url:");
        let response = ui.add(egui::TextEdit::singleline(&mut self.server_ip));
        if response.changed() {
            println!("Text changed to: {}", self.server_ip);
        }
    }
    fn ui_website_hyperlink(&mut self, ui: &mut egui::Ui) {
        let hosted_url = self.hosted_website_url.lock().unwrap();
        if let Some(url) = &*hosted_url {
            ui.label("Website url:");
            ui.hyperlink(url);
        }
    }
    fn ui_host_button(&mut self, ui: &mut egui::Ui) {
        ui.add_enabled_ui(self.has_uploaded_file, |ui| {
            if ui.button("Host Website").clicked() {
                if let Some(content_to_send) = self.file_content.clone() {
                    let url_arc = self.hosted_website_url.clone();
                    let uri = self.server_ip.clone();
                    tokio::spawn(async move {
                        let emissary_html = serde_json::json!({
                            "html_content": content_to_send
                        });
                        let client = reqwest::Client::new();
                        println!("Phantom dispatched. Sending will to the Formation Spirit...");

                        match client.post(uri).json(&emissary_html).send().await {
                            Ok(response) => {
                                if response.status().is_success() {
                                    match response.json::<serde_json::Value>().await {
                                        Ok(report) => {
                                            println!("Phantom reports success: {:?}", report);
                                            if let Some(url) = report["url"].as_str() {
                                                let mut hosted_url = url_arc.lock().unwrap();
                                                *hosted_url = Some(url.to_string());
                                            }
                                            
                                        }
                                        Err(_) => println!("Phantom reports success, but the reply was garbled."),
                                    }
                                } else {
                                    let error_report = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                                    println!("Phantom reports a tribulation: {}", error_report);
                                }
                            },
                            Err(e) => {
                                println!("The phantom was lost in the chaotic void: {}", e);
                            }
                        }
                    });
                    println!("Will has been dispatched. The main body continues its duties.");
                }

                
                
                println!("Hosting in : idk");
            }
        });
    }

}



impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Immortal Formation Gensokyo");
            ui.separator();
            self.ui_file_controls(ui);

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            self.ui_viewer(ui);

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            self.ui_server_textedit(ui);

            ui.add_space(5.0);
            ui.separator();
            ui.add_space(5.0);

            self.ui_host_button(ui);

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            self.ui_website_hyperlink(ui);
        });
    }
}
