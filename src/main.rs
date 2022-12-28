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
struct StudyBlocker {
    domains: String,
    length_of_study: u32,
    start_time: u32,
}

impl Default for StudyBlocker {
    fn default() -> Self {
        Self {
            domains: String::new(),
            length_of_study: 0,
            start_time: 0,
        }
    }
}

impl StudyBlocker {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for StudyBlocker {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self {
            domains,
            length_of_study,
            start_time,
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
                    block_domains(domains.to_owned());
                }

                if ui.button("Unblock Domains").clicked() {
                    unblock_domains();
                }
            });

            // DEBUG: Displaying the split domains
            for domain in split_domains {
                ui.label(domain);
            }
        });
    }
}
