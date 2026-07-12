#![deny(unsafe_code)]

use std::{fs, time::Duration};

use dev_recall_application::{BackgroundOperationManager, DEFAULT_MAX_BACKGROUND_OPERATIONS};
use dev_recall_observability::LocalLogger;
use tauri::{Manager, RunEvent};

mod ipc;
pub mod localization;

pub const APP_NAME: &str = "Dev Recall";
const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(5);

pub fn run() -> Result<(), tauri::Error> {
    let application = tauri::Builder::default()
        .setup(|application| {
            let application_data_directory = application.path().app_data_dir()?;
            fs::create_dir_all(&application_data_directory)?;

            let logger =
                LocalLogger::open_for_application_data_directory(&application_data_directory)?;
            application.manage(logger);

            let background_operations =
                BackgroundOperationManager::new(DEFAULT_MAX_BACKGROUND_OPERATIONS)?;
            application.manage(background_operations);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![ipc::get_application_health])
        .build(tauri::generate_context!())?;

    application.run(|application_handle, event| {
        if matches!(event, RunEvent::ExitRequested { .. }) {
            let background_operations = application_handle.state::<BackgroundOperationManager>();
            let _ = background_operations.shutdown(SHUTDOWN_TIMEOUT);
        }
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::APP_NAME;

    #[test]
    fn application_name_is_stable() {
        assert_eq!(APP_NAME, "Dev Recall");
    }
}
