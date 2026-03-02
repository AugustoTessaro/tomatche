use std::sync::Mutex;
use tauri::State;
use crate::models::timer::TimerState;

#[tauri::command]
pub fn start_timer() -> String {
    
    "Timer iniciado!".to_string()
}

#[tauri::command]
pub fn get_timer_status() -> String {
    "Status do timer".to_string()
}

#[tauri::command]
pub fn get_timer_state(state: State<'_, Mutex<TimerState>>) -> TimerState {
    let timer = state.lock().unwrap(); 
    timer.clone() 
}