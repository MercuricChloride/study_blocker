mod blocking;

use std::time::{Duration, SystemTime};

use blocking::{block_domains, has_host_access, unblock_domains};

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
    length_of_study: u64,
    start_time: SystemTime,
    blocking: bool, // we are going to use this to determine if we are blocking or not, so we can not copy a blocked hosts file
    #[serde(skip)] // we don't want to save this field
    root: bool, // we are going to use this to determine if we are running as root or not
}

impl Default for StudyBlocker {
    fn default() -> Self {
        Self {
            domains: String::new(),
            length_of_study: 0,
            start_time: SystemTime::now(),
            blocking: false,
            root: has_host_access(),
        }
    }
}

impl StudyBlocker {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
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

    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let Self {
            domains,
            length_of_study,
            start_time,
            blocking,
            root,
        } = self;

        let time_elapsed = SystemTime::now()
            .duration_since(*start_time)
            .unwrap()
            .as_secs();

        ctx.request_repaint_after(Duration::new(0, 100_000_000)); // 100ms

        // if we are blocking and the time elapsed is greater than the length of study, unblock
        if *blocking && *length_of_study != 0 && time_elapsed > *length_of_study {
            unblock_domains();
            *blocking = false;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Study Blocker");

            ui.separator();

            // if we don't have access to the hosts file, show a message
            if !*root {
                ui.heading("You do not have access to the hosts file. Please run as root.");
            } else {
                // Length of study slider & domain input field
                // only show if we are not blocking
                if !*blocking {
                    // length of study input
                    ui.label("Duration of Study");
                    ui.add(egui::Slider::new(length_of_study, 0..=24).text("Hours"));

                    // domain input
                    ui.label("Domains to Block");
                    ui.text_edit_multiline(domains);
                } else {
                    // otherwise show the time left
                    let time_left = *length_of_study - time_elapsed;
                    ui.label(format!("Time Left: {} minutes", time_left));
                }

                ui.separator();

                if *blocking {
                    if ui.button("Unblock Domains").clicked() {
                        unblock_domains();
                        *blocking = false;
                    }
                } else {
                    if ui.button("Block Domains").clicked() {
                        block_domains(domains.to_owned(), blocking);
                        *start_time = SystemTime::now();
                        *blocking = true;
                    }
                }
            }
        });
    }
}
