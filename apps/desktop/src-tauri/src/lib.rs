#![deny(unsafe_code)]

mod ipc;
pub mod localization;

pub const APP_NAME: &str = "Dev Recall";

pub fn run() -> Result<(), tauri::Error> {
    tauri::Builder::default()
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
