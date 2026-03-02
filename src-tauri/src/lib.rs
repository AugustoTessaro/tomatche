pub mod models;
pub mod commands;

use std::sync::Mutex; 
use models::timer::TimerState; 
use commands::timer::get_timer_state; 

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        
        .manage(Mutex::new(TimerState::new())) 
        
        
        .invoke_handler(tauri::generate_handler![
            get_timer_state
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}