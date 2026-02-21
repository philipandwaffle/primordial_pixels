use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stats {
    pub metronome_beat: f32,
    pub incubation_period: f32,
}
impl Stats {
    pub fn new(metronome_beat: f32, incubation_period: f32) -> Self {
        return Self {
            metronome_beat,
            incubation_period,
        };
    }
}

pub struct VariableStats {
    pub time_alive: f32,
}
impl VariableStats {
    pub fn new() -> Self {
        return Self { time_alive: 0.0 };
    }
}
