#![deny(unsafe_code)]

use std::fs;

use dev_recall_observability::LocalLogger;
use tauri::Manager;

mod ipc;
pub mod localization;

pub const APP_NAME: &str = "Dev Recall";

pub fn run() -> Result<(), tauri::Error> {
    tauri::Builder::default()
        .setup(|application| {
            let application_data_directory = application.path().app_data_dir()?;
            fs::create_dir_all(&application_data_directory)?;

            let logger =
                LocalLogger::open_for_application_data_directory(&application_data_directory)?;
            application.manage(logger);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![ipc::get_application_health])
        .run(tauri::generate_context!())
}

#[cfg(test)]
mod tests {
    use super::APP_NAME;

    #[test]
    fn application_name_is_stable() {
        assert_eq!(APP_NAME, "Dev Recall");
    }
}
