use crate::{
    player::{spawn_player, Player},
    resources::JoinedPlayers,
};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct GamepadPlugin;

impl Plugin for GamepadPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(Update, (join, disconnect));
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
    Disconnect,
}

fn join(
    mut commands: Commands,
    mut joined_players: ResMut<JoinedPlayers>,
    gamepads: Res<Gamepads>,
    button_inputs: Res<Input<GamepadButton>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for gamepad in gamepads.iter() {
        // Join the game when both bumpers (L+R) on the controller are pressed
        // We drop down the Bevy's input to get the input from each gamepad
        if button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger))
            && button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::RightTrigger))
        {
            // Make sure a player can not join twice
            if !joined_players.0.contains_key(&gamepad) {
                println!("Player {} has joined the game!", gamepad.id);

                let input_map = InputMap::default()
                    .insert_multiple([
                        (GamepadButtonType::DPadLeft, PlayerAction::Left),
                        (GamepadButtonType::DPadRight, PlayerAction::Right),
                        (GamepadButtonType::DPadUp, PlayerAction::Up),
                        (GamepadButtonType::DPadDown, PlayerAction::Down),
                        (GamepadButtonType::South, PlayerAction::Throw),
                        (GamepadButtonType::West, PlayerAction::Dash),
                        (GamepadButtonType::Start, PlayerAction::Start),
                        (GamepadButtonType::Select, PlayerAction::Disconnect),
                    ])
                    .insert(DualAxis::left_stick(), PlayerAction::Move)
                    // Make sure to set the gamepad or all gamepads will be used!
                    .set_gamepad(gamepad)
                    .build();

                let player = spawn_player(
                    &mut commands,
                    &asset_server,
                    &mut texture_atlases,
                    input_map,
                    gamepad,
                );
                // Insert the created player and its gamepad to the hashmap of joined players
                // Since uniqueness was already checked above, we can insert here unchecked
                joined_players.0.insert_unique_unchecked(gamepad, player);
            }
        }
    }
}

fn disconnect(
    mut commands: Commands,
    action_query: Query<(&ActionState<PlayerAction>, &Player)>,
    mut joined_players: ResMut<JoinedPlayers>,
) {
    for (action_state, player) in action_query.iter() {
        if action_state.pressed(PlayerAction::Disconnect) {
            let player_entity = *joined_players.0.get(&player.gamepad).unwrap();

            // Despawn the disconnected player and remove them from the joined player list
            commands.entity(player_entity).despawn();
            joined_players.0.remove(&player.gamepad);

            println!("Player {} has disconnected!", player.gamepad.id);
        }
    }
}
