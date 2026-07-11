#![deny(unsafe_code)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    if dev_recall_lib::run().is_err() {
        let locale = dev_recall_lib::localization::system_locale();
        eprintln!(
            "{}",
            dev_recall_lib::localization::startup_failure_message(locale)
        );
        std::process::exit(1);
    }
}
