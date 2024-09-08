use crate::utils::app_dir;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub(crate) fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub(crate) fn get_app_dir() -> String {
    app_dir().display().to_string()
}

/*
* The download command is a placeholder for the download functionality.
* It only demos the ipc from the front end to the tauri backend. There
* is no actual download functionality implemented in this command.
*/
#[tauri::command]
pub(crate) fn download(content: String) -> String {
    format!("Save by tauri {}", content)
}
