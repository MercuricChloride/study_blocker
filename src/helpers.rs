use std::{
    fs::{copy, read_to_string, write},
    time::SystemTime,
};

use eframe::egui;

pub fn block_domains(domains: String, blocking: &bool) {
    // general plan here is to split domains into a vector of strings
    let split_domains = domains
        .split("\n")
        .map(|item| item.replace("www.", "").trim().to_owned())
        .collect::<Vec<String>>();

    // if the user is blocking, we don't want to copy the hosts file, we just want to write to it
    if !blocking {
        copy("/etc/hosts", "/etc/hosts.bak").expect("Failed to backup hosts file");
    }

    let mut hosts = read_to_string("/etc/hosts").expect("Failed to read hosts file");

    // then for each domain we will format the string to be added to the hosts file
    for domain in split_domains {
        let formatted_domain = format!("\n127.0.0.1 {} www.{}\n:: {}", domain, domain, domain);

        hosts.push_str(&formatted_domain)
    }

    // write to the hosts file
    write("/etc/hosts", hosts).expect("Failed to write to hosts file");
}

pub fn unblock_domains() {
    copy("/etc/hosts.bak", "/etc/hosts").expect("Failed to restore hosts file");
}

pub fn has_host_access() -> bool {
    copy("/etc/hosts", "/etc/hosts").is_ok()
}

pub fn get_time_elapsed(start_time: &SystemTime) -> u64 {
    start_time.elapsed().unwrap().as_secs()
}

pub fn should_unblock(start_time: &SystemTime, length_of_study: &u64, is_blocking: &bool) -> bool {
    let elapsed = get_time_elapsed(start_time);

    // should unblock if the time elapsed is greater than the length of the study in seconds and we are blocking
    (elapsed >= (length_of_study * 3600)) && *is_blocking
}

fn format_time(time: u64) -> String {
    if time < 60 {
        format!("Time Left: {} Seconds", time)
    } else if time < 3600 {
        format!("Time Left: {} Minutes", time / 60)
    } else {
        format!("Time Left: {} Hours", time / 3600)
    }
}

pub fn display_config_or_remaining_time(
    ui: &mut egui::Ui,
    time_elapsed: u64,
    length_of_study: &mut u64,
    domains: &mut String,
    blocking: &bool,
) {
    if *blocking {
        let time_left = (*length_of_study * 3600) - time_elapsed;
        ui.label(format_time(time_left));
    } else {
        // length of study input
        ui.label("Duration of Study");
        ui.add(egui::Slider::new(length_of_study, 0..=24).text("Hours"));

        // domain input
        ui.label("Domains to Block");
        ui.text_edit_multiline(domains);
    }
}

pub fn display_block_button(
    blocking: &mut bool,
    ui: &mut egui::Ui,
    domains: &mut String,
    start_time: &mut SystemTime,
) {
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
