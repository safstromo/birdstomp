use bevy::prelude::*;

#[derive(Resource)]
pub struct Score {
    pub score: usize,
}

#[derive(Resource)]
pub struct Lives {
    pub lives: usize,
}

// #[derive(Resource)]
// pub struct CarSpeed {
//     pub speed: f32,
// }

#[derive(Resource)]
pub struct SpawnTimer(pub Timer);

#[derive(Resource)]
pub struct DifficultyTimer(pub Timer);

#[derive(Resource)]
pub struct CountdownTimer {
    pub(crate) timer: Timer,
    pub(crate) duration: u8,
}

impl CountdownTimer {
    pub fn new() -> Self {
        Self {
            duration: 4,
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}

impl Default for CountdownTimer {
    fn default() -> Self {
        Self::new()
    }
}
