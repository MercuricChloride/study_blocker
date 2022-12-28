use std::fs::{copy, read_to_string, write};

pub fn block_domains(domains: String) {
    // general plan here is to split domains into a vector of strings
    let split_domains = domains
        .split("\n")
        .map(|item| item.trim().to_owned())
        .collect::<Vec<String>>();
    // backup the hosts file to /etc/hosts.bak
    copy("/etc/hosts", "/etc/hosts.bak").expect("Failed to backup hosts file");

    let mut hosts = read_to_string("/etc/hosts").expect("Failed to read hosts file");

    // then for each domain we will format the string to be added to the hosts file
    for domain in split_domains {
        let formatted_domain = format!("\n127.0.0.1 {}\n:: {}", domain, domain);
        hosts.push_str(&formatted_domain)
    }
    // write to the hosts file
    write("/etc/hosts", hosts).expect("Failed to write to hosts file");
}

pub fn unblock_domains() {
    copy("/etc/hosts.bak", "/etc/hosts").expect("Failed to restore hosts file");
}
