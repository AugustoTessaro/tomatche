use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, TS)]
#[ts(export, export_to = "../../src/types/Phase.ts")] 
pub enum Phase {
    Work,
    ShortBreak,
    LongBreak,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../../src/types/TimerState.ts")]
pub struct TimerState {
    pub work_interval: u32,
    pub short_interval: u32,
    pub long_interval: u32,
    pub cycles: u32,
    pub work_sessions_completed: u32,
    pub time_remaining: u32,
    pub current_phase: Phase,
    pub is_running: bool,
}

impl TimerState {
    pub fn new() -> Self {
        let default_work_min = 25;
        
        Self {
            work_interval: default_work_min,
            short_interval: 5,
            long_interval: 15,
            cycles: 0,
            work_sessions_completed: 0,
            time_remaining: default_work_min * 60, 
            current_phase: Phase::Work,
            is_running: false,
        }
    }
}