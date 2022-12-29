mod helpers;

use std::time::{Duration, SystemTime};

use helpers::{
    display_block_button, display_config_or_remaining_time, get_time_elapsed, has_host_access,
    should_unblock, unblock_domains,
};

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
    has_host_access: bool, // we are going to use this to determine if we are running as root or not
}

impl Default for StudyBlocker {
    fn default() -> Self {
        Self {
            domains: String::new(),
            length_of_study: 0,
            start_time: SystemTime::now(),
            blocking: false,
            has_host_access: has_host_access(),
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
            has_host_access,
        } = self;

        let time_elapsed = get_time_elapsed(start_time);

        ctx.request_repaint_after(Duration::new(0, 100_000_000)); // 100ms

        // if we are blocking and the time elapsed is greater than the length of study, unblock
        if should_unblock(start_time, &length_of_study, &blocking) {
            unblock_domains();
            *blocking = false;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Study Blocker");

            ui.separator();

            // if we don't have access to the hosts file, show a message
            if !*has_host_access {
                ui.heading("You do not have access to the hosts file. Please run as root.");
                return;
            }

            display_config_or_remaining_time(ui, time_elapsed, length_of_study, domains, blocking);

            ui.separator();

            display_block_button(blocking, ui, domains, start_time);
        });
    }
}
