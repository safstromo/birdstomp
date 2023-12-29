use crate::player::{NewPlayer, PlayerBundle};
/// Simple resource to store the ID of the connected gamepad.
/// We need to know which gamepad to use for player input.
use bevy::{
    input::gamepad::{
        GamepadAxisChangedEvent, GamepadButtonChangedEvent, GamepadButtonInput,
        GamepadConnectionEvent,
    },
    prelude::*,
};
use leafwing_input_manager::{axislike::DualAxisData, prelude::*, user_input::InputKind};
///
///
///
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
}

// TODO - add Stick support
impl PlayerBundle {
    pub fn input_map(player: NewPlayer) -> InputMap<PlayerAction> {
        let mut input_map = match player {
            NewPlayer::One => InputMap::new([
                (KeyCode::A, PlayerAction::Left),
                (KeyCode::D, PlayerAction::Right),
                (KeyCode::W, PlayerAction::Up),
                (KeyCode::S, PlayerAction::Down),
                (KeyCode::T, PlayerAction::Throw),
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
                (KeyCode::ShiftLeft, PlayerAction::Throw),
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
        ]);

        input_map
    }
}
