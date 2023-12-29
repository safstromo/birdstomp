use crate::enemy::Enemy;
use crate::gamepad::PlayerAction;
use crate::sprites::{AnimationIndices, AnimationTimer};
use crate::{AppState, GameState, BOTTOM_WALL, LEFT_WALL, RIGHT_WALL, TOP_WALL, WALL_THICKNESS};
use bevy::{input::gamepad::GamepadAxisChangedEvent, prelude::*};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;
const PLAYER_SPEED: f32 = 500.0;
const PLAYER_PADDING: f32 = 10.0;
const PLAYER_SIZE: Vec2 = Vec2::new(5.0, 8.0);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(
            Update,
            (move_player, move_player_with_gamepad, collision_with_enemy)
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameState::Running)),
        );
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: NewPlayer,
    direction: PlayerDirection,
    velocity: Velocity,
    sprite: SpriteSheetBundle,
    animation_indices: AnimationIndices,
    animation_timer: AnimationTimer,
    input_manager: InputManagerBundle<PlayerAction>,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            marker: NewPlayer::One,
            direction: PlayerDirection {
                direction: Vec2::new(0.0, 0.0),
            },
            velocity: Velocity(Vec2::new(0.0, 0.0)),
            input_manager: InputManagerBundle::default(),
            sprite: SpriteSheetBundle::default(),
            animation_indices: AnimationIndices {
                first: 10,
                last: 13,
            },
            animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        }
    }
}

#[derive(Component, Debug)]
pub enum NewPlayer {
    One,
    Two,
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Player {
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct Player2 {
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct PlayerDirection {
    pub direction: Vec2,
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

    commands
        .spawn(PlayerBundle {
            marker: NewPlayer::One,
            input_manager: InputManagerBundle {
                input_map: PlayerBundle::input_map(NewPlayer::One),
                ..Default::default()
            },
            sprite: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(animation_indices.first),
                transform: Transform::from_xyz(50.0, -250., 2.0),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(KinematicCharacterController::default())
        .insert(Collider::ball(10.0))
        .insert(ActiveEvents::COLLISION_EVENTS);

    commands
        .spawn(PlayerBundle {
            marker: NewPlayer::Two,
            input_manager: InputManagerBundle {
                input_map: PlayerBundle::input_map(NewPlayer::Two),
                ..Default::default()
            },
            sprite: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(animation_indices.first),
                transform: Transform::from_xyz(50.0, -250., 2.0),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(KinematicCharacterController::default())
        .insert(Collider::ball(10.0))
        .insert(ActiveEvents::COLLISION_EVENTS);

    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(animation_indices.first),
                transform: Transform::from_xyz(50.0, -250., 2.0),
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Player {
                velocity: Vec2::new(0.0, 0.0),
            }, // Collider,
            PlayerDirection {
                direction: Vec2::new(0.0, 0.0),
            },
        ))
        .insert(RigidBody::KinematicPositionBased)
        .insert(KinematicCharacterController::default())
        .insert(Collider::ball(10.0))
        .insert(ActiveEvents::COLLISION_EVENTS);

    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices2.first),
                transform: Transform::from_xyz(-50.0, -250., 2.0),
                ..default()
            },
            animation_indices2,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Player2 {
                velocity: Vec2::new(0.0, 0.0),
            }, // Collider,
            PlayerDirection {
                direction: Vec2::new(0.0, 0.0),
            },
        ))
        .insert(RigidBody::KinematicPositionBased)
        .insert(KinematicCharacterController::default())
        .insert(Collider::ball(10.0))
        .insert(ActiveEvents::COLLISION_EVENTS);

    //TODO the child collider gets different index than the parent
    // .with_children(|children| {
    //     children
    //         .spawn(Collider::ball(10.0))
    //         .insert(TransformBundle::from(Transform::from_xyz(0.0, -8.0, 0.0)))
    //         .insert(ActiveEvents::COLLISION_EVENTS);
    // });
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
    mut player: Query<(&mut Player, &mut PlayerDirection, Without<Player2>)>,
    mut player2: Query<(&mut Player2, &mut PlayerDirection, Without<Player>)>,
    gamepad_axis_changed_events: Res<Events<GamepadAxisChangedEvent>>,
    time_step: Res<Time<Fixed>>,
) {
    let (mut player1, mut player1_direction, ()) = player.single_mut();
    let (mut player2, mut player2_direction, ()) = player2.single_mut();
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
                player1_direction.direction.x = event.value;
            }
            GamepadAxisType::LeftStickY => {
                player1.velocity.y = event.value;
                player1_direction.direction.y = event.value;
            }
            GamepadAxisType::RightStickX => {
                player2.velocity.x = event.value;
                player2_direction.direction.x = event.value;
            }
            GamepadAxisType::RightStickY => {
                player2.velocity.y = event.value;
                player2_direction.direction.y = event.value;
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

fn collision_with_enemy(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>,
    player1_query: Query<Entity, With<Player>>,
    player2_query: Query<Entity, With<Player2>>,
    rapier_context: Res<RapierContext>,
    mut score: ResMut<crate::Score>,
    mut lives: ResMut<crate::Lives>,
) {
    let player1 = player1_query.single();
    let player2 = player2_query.single();
    let enemy = enemy_query.single();

    if let Some(contact_pair) = rapier_context.contact_pair(player1, enemy) {
        if contact_pair.has_any_active_contacts() {
            println!(
                "Contact  player {} with enemy {}:",
                player1.index(),
                enemy.index()
            );
            commands.insert_resource(NextState(Some(GameState::Paused)));
            lives.lives -= 1;
        }
    }
    if let Some(contact_pair2) = rapier_context.contact_pair(player2, enemy) {
        if contact_pair2.has_any_active_contacts() {
            println!(
                "Contact  player {} with enemy {}:",
                player2.index(),
                enemy.index()
            );
            commands.insert_resource(NextState(Some(GameState::Paused)));
            score.score += 1;
        }
    }
}
