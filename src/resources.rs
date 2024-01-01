use bevy::{prelude::*, utils::HashMap};

// This is used to check if a player already exists and which entity to disconnect
#[derive(Resource, Default)]
pub struct JoinedPlayers(pub HashMap<Gamepad, Entity>);

#[derive(Resource)]
pub struct Player2Lives {
    pub lives: usize,
}

#[derive(Resource)]
pub struct Player1Lives {
    pub lives: usize,
}

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
            duration: 2,
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}

impl Default for CountdownTimer {
    fn default() -> Self {
        Self::new()
    }
}
