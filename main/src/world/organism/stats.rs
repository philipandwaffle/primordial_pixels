use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StaticStats {
    pub metronome_beat: f32,
}
impl StaticStats {
    pub fn new(metronome_beat: f32) -> Self {
        return Self { metronome_beat };
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
