use crate::player::{NewPlayer, PlayerBundle};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct GamepadPlugin;

impl Plugin for GamepadPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default());
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PlayerAction {
    Up,
    Down,
    Left,
    Right,
    Throw,
    Dash,
    Move,
    Start,
}

//TODO: Set gamepads dynamically to players
impl PlayerBundle {
    pub fn input_map(player: NewPlayer) -> InputMap<PlayerAction> {
        let mut input_map = match player {
            NewPlayer::One => InputMap::new([
                (KeyCode::A, PlayerAction::Left),
                (KeyCode::D, PlayerAction::Right),
                (KeyCode::W, PlayerAction::Up),
                (KeyCode::S, PlayerAction::Down),
                (KeyCode::ShiftLeft, PlayerAction::Throw),
                (KeyCode::ControlLeft, PlayerAction::Dash),
            ])
            // This is a quick and hacky solution:
            // you should coordinate with the `Gamepads` resource to determine the correct gamepad for each player
            // and gracefully handle disconnects
            // Note that this step is not required:
            // if it is skipped all input maps will read from all connected gamepads
            .set_gamepad(Gamepad { id: 0 })
            .build(),
            NewPlayer::Two => InputMap::new([
                (KeyCode::Left, PlayerAction::Left),
                (KeyCode::Right, PlayerAction::Right),
                (KeyCode::Up, PlayerAction::Up),
                (KeyCode::Down, PlayerAction::Down),
                (KeyCode::ShiftRight, PlayerAction::Throw),
                (KeyCode::ControlRight, PlayerAction::Dash),
            ])
            .set_gamepad(Gamepad { id: 1 })
            .build(),
        };

        // Each player will use the same gamepad controls, but on separate gamepads.
        input_map.insert_multiple([
            (GamepadButtonType::DPadLeft, PlayerAction::Left),
            (GamepadButtonType::DPadRight, PlayerAction::Right),
            (GamepadButtonType::DPadUp, PlayerAction::Up),
            (GamepadButtonType::DPadDown, PlayerAction::Down),
            (GamepadButtonType::South, PlayerAction::Throw),
            (GamepadButtonType::West, PlayerAction::Dash),
            (GamepadButtonType::Start, PlayerAction::Start),
        ]);

        input_map.insert(DualAxis::left_stick(), PlayerAction::Move);

        input_map
    }
}
