#![deny(unsafe_code)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    if dev_recall_lib::run().is_err() {
        eprintln!("Dev Recall could not start.");
        std::process::exit(1);
    }
}
