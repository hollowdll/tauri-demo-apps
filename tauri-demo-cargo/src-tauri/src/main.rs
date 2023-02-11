#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello from Rust, {}!", name)
}

#[tauri::command]
fn say_hi() -> String {
    format!("Hi from Rust!")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, say_hi])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
