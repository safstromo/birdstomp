use crate::sprites::{AnimationIndices, AnimationTimer};
use crate::{AppState, GameState, BOTTOM_WALL, LEFT_WALL, RIGHT_WALL, TOP_WALL, WALL_THICKNESS};
use bevy::{input::gamepad::GamepadAxisChangedEvent, prelude::*};
const PLAYER_SPEED: f32 = 500.0;
const PLAYER_PADDING: f32 = 10.0;
const PLAYER_SIZE: Vec2 = Vec2::new(5.0, 8.0);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(
            Update,
            (move_player, move_player_with_gamepad)
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameState::Running)),
        );
    }
}

#[derive(Component)]
pub struct Player {
    pub health: f32,
    velocity: Vec2,
}

#[derive(Component)]
pub struct Player2 {
    pub health: f32,
    velocity: Vec2,
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("duckyatlas.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 5, 3, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices {
        first: 10,
        last: 13,
    };
    let animation_indices2 = AnimationIndices { first: 0, last: 4 };

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_xyz(50.0, -250., 2.0),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player {
            health: 100.0,
            velocity: Vec2::new(0.0, 0.0),
        }, // Collider,
    ));

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices2.first),
            transform: Transform::from_xyz(-50.0, -250., 2.0),
            ..default()
        },
        animation_indices2,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player2 {
            health: 100.0,
            velocity: Vec2::new(0.0, 0.0),
        }, // Collider,
    ));
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time_step: Res<Time<Fixed>>,
) {
    let mut player_transform = query.single_mut();
    let mut horizontal = 0.0;
    let mut vertical = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        horizontal -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        horizontal += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        vertical += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        vertical -= 1.0;
    }

    let new_player_position_horizontal =
        player_transform.translation.x + horizontal * PLAYER_SPEED * time_step.delta_seconds();

    let new_player_position_vertical =
        player_transform.translation.y + vertical * PLAYER_SPEED * time_step.delta_seconds();
    // Update the player position,
    // making sure it doesn't cause the player to leave the arena
    let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + PLAYER_SIZE.x / 2.0 + PLAYER_PADDING;
    let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - PLAYER_SIZE.x / 2.0 - PLAYER_PADDING;
    let top_bound = TOP_WALL - WALL_THICKNESS / 2.0 - PLAYER_SIZE.y / 2.0 - PLAYER_PADDING;
    let bottom_bound = BOTTOM_WALL + WALL_THICKNESS / 2.0 + PLAYER_SIZE.y / 2.0 + PLAYER_PADDING;

    player_transform.translation.x = new_player_position_horizontal.clamp(left_bound, right_bound);
    player_transform.translation.y = new_player_position_vertical.clamp(bottom_bound, top_bound);
}

fn move_player_with_gamepad(
    mut query: Query<&mut Transform, (With<Player>, Without<Player2>)>,
    mut query2: Query<&mut Transform, (With<Player2>, Without<Player>)>,
    mut player: Query<&mut Player>,
    mut player2: Query<&mut Player2>,
    gamepad_axis_changed_events: Res<Events<GamepadAxisChangedEvent>>,
    time_step: Res<Time<Fixed>>,
) {
    let mut player1 = player.single_mut();
    let mut player2 = player2.single_mut();
    let mut player1_transform = query.single_mut();
    let mut player2_transform = query2.single_mut();
    //Stick movement
    for event in gamepad_axis_changed_events
        .get_reader()
        .read(&gamepad_axis_changed_events)
    {
        match event.axis_type {
            GamepadAxisType::LeftStickX => {
                player1.velocity.x = event.value;
            }
            GamepadAxisType::LeftStickY => {
                player1.velocity.y = event.value;
            }
            GamepadAxisType::RightStickX => {
                player2.velocity.x = event.value;
            }
            GamepadAxisType::RightStickY => {
                player2.velocity.y = event.value;
            }
            _ => {}
        }
    }

    let new_player1_position_horizontal = player1_transform.translation.x
        + player1.velocity.x * PLAYER_SPEED * time_step.delta_seconds();
    let new_player1_position_vertical = player1_transform.translation.y
        + player1.velocity.y * PLAYER_SPEED * time_step.delta_seconds();

    let new_player2_position_horizontal = player2_transform.translation.x
        + player2.velocity.x * PLAYER_SPEED * time_step.delta_seconds();
    let new_player2_position_vertical = player2_transform.translation.y
        + player2.velocity.y * PLAYER_SPEED * time_step.delta_seconds();

    // Update the player position,
    // making sure it doesn't cause the player to leave the arena
    let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + PLAYER_SIZE.x / 2.0 + PLAYER_PADDING;
    let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - PLAYER_SIZE.x / 2.0 - PLAYER_PADDING;
    let top_bound = TOP_WALL - WALL_THICKNESS / 2.0 - PLAYER_SIZE.y / 2.0 - PLAYER_PADDING;
    let bottom_bound = BOTTOM_WALL + WALL_THICKNESS / 2.0 + PLAYER_SIZE.y / 2.0 + PLAYER_PADDING;

    player1_transform.translation.x =
        new_player1_position_horizontal.clamp(left_bound, right_bound);
    player1_transform.translation.y = new_player1_position_vertical.clamp(bottom_bound, top_bound);
    player2_transform.translation.x =
        new_player2_position_horizontal.clamp(left_bound, right_bound);
    player2_transform.translation.y = new_player2_position_vertical.clamp(bottom_bound, top_bound);
}
