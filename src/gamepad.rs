/// Simple resource to store the ID of the connected gamepad.
/// We need to know which gamepad to use for player input.
use bevy::{
    input::gamepad::{
        GamepadAxisChangedEvent, GamepadButtonChangedEvent, GamepadButtonInput,
        GamepadConnectionEvent,
    },
    prelude::*,
};
use leafwing_input_manager::prelude::*;

use crate::player::{NewPlayer, PlayerBundle};
///
///
///
pub struct GamepadPlugin;

impl Plugin for GamepadPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(Update, (gamepad_events, input_test));
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

// Query for the `ActionState` component in your game logic systems!
fn input_test(query: Query<(&ActionState<PlayerAction>, &NewPlayer), With<NewPlayer>>) {
    // Each action has a button-like state of its own that you can check
    for (action_state, player) in query.iter() {
        if action_state.just_pressed(PlayerAction::Up) {
            println!("I'm up! player {:?}", player);
        }
        if action_state.just_pressed(PlayerAction::Down) {
            println!("I'm down! player {:?}", player);
        }
        if action_state.just_pressed(PlayerAction::Left) {
            println!("I'm left! player {:?}", player);
        }
        if action_state.just_pressed(PlayerAction::Right) {
            println!("I'm right! player {:?}", player);
        }
        if action_state.just_pressed(PlayerAction::Throw) {
            println!("I'm throw! player {:?}", player);
        }
    }
}

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

fn gamepad_events(
    mut connection_events: EventReader<GamepadConnectionEvent>,
    mut axis_changed_events: EventReader<GamepadAxisChangedEvent>,
    // Handles the continuous measure of how far a button has been pressed down, as measured
    // by `Axis<GamepadButton>`. Whenever that value changes, this event is emitted.
    mut button_changed_events: EventReader<GamepadButtonChangedEvent>,
    // Handles the boolean measure of whether a button is considered pressed or unpressed, as
    // defined by the thresholds in `GamepadSettings::button_settings` and measured by
    // `Input<GamepadButton>`. When the threshold is crossed and the button state changes,
    // this event is emitted.
    mut button_input_events: EventReader<GamepadButtonInput>,
) {
    // for connection_event in connection_events.read() {
    //     info!("{:?}", connection_event);
    // }
    // for axis_changed_event in axis_changed_events.read() {
    //     info!(
    //         "{:?} of {:?} is changed to {}",
    //         axis_changed_event.axis_type, axis_changed_event.gamepad, axis_changed_event.value
    //     );
    // }
    // for button_changed_event in button_changed_events.read() {
    //     info!(
    //         "{:?} of {:?} is changed to {}",
    //         button_changed_event.button_type,
    //         button_changed_event.gamepad,
    //         button_changed_event.value
    //     );
    // }
    // for button_input_event in button_input_events.read() {
    //     info!("{:?}", button_input_event);
    // }
}

// If you require in-frame relative event ordering, you can also read the `Gamepad` event
// stream directly. For standard use-cases, reading the events individually or using the
// `Input<T>` or `Axis<T>` resources is preferable.
// fn gamepad_ordered_events(mut gamepad_events: EventReader<GamepadEvent>) {
// for gamepad_event in gamepad_events.read() {
// match gamepad_event {
// GamepadEvent::Connection(connection_event) => info!("{:?}", connection_event),
// GamepadEvent::Button(button_event) => info!("{:?}", button_event),
// GamepadEvent::Axis(axis_event) => info!("{:?}", axis_event),
// }
// }
// }
