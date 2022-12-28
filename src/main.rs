mod blocking;

use blocking::{block_domains, unblock_domains};

use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Study Blocker",
        native_options,
        Box::new(|cc| Box::new(StudyBlocker::new(cc))),
    );
}

/// Data Structure for the App
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
struct StudyBlocker {
    domains: String,
    length_of_study: u32,
    start_time: u32,
    blocking: bool, // we are going to use this to determine if we are blocking or not, so we can not copy a blocked hosts file
}

impl Default for StudyBlocker {
    fn default() -> Self {
        Self {
            domains: String::new(),
            length_of_study: 0,
            start_time: 0,
            blocking: false,
        }
    }
}

impl StudyBlocker {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Self::default()
    }
}

impl eframe::App for StudyBlocker {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self {
            domains,
            length_of_study,
            start_time,
            blocking,
        } = self;

        let split_domains: Vec<String> = domains.split("\n").map(|item| item.to_owned()).collect();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Study Blocker");

            ui.separator();

            // domain input
            ui.label("Domains to Block");
            ui.text_edit_multiline(domains);

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Block Domains").clicked() {
                    block_domains(domains.to_owned(), blocking);
                    *blocking = true;
                }

                if ui.button("Unblock Domains").clicked() {
                    unblock_domains();
                    *blocking = false;
                }
            });

            // DEBUG: Displaying the split domains
            for domain in split_domains {
                ui.label(domain);
            }
        });
    }
}
