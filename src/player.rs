use crate::enemy::Enemy;
use crate::gamepad::PlayerAction;
use crate::sprites::{AnimationIndices, AnimationTimer};
use crate::{AppState, GameState, BOTTOM_WALL, LEFT_WALL, RIGHT_WALL, TOP_WALL, WALL_THICKNESS};
use bevy::prelude::*;
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
            (move_player, collision_with_enemy)
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
                transform: Transform::from_xyz(-50.0, -250., 2.0),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(KinematicCharacterController::default())
        .insert(Collider::ball(10.0))
        .insert(ActiveEvents::COLLISION_EVENTS);
}

fn move_player(
    mut query: Query<
        (
            &ActionState<PlayerAction>,
            &mut Transform,
            &mut Velocity,
            &mut PlayerDirection,
        ),
        With<NewPlayer>,
    >,
    time_step: Res<Time<Fixed>>,
) {
    for (action_state, mut player_transform, mut velocity, mut direction) in query.iter_mut() {
        let mut horizontal = 0.0;
        let mut vertical = 0.0;
        if action_state.pressed(PlayerAction::Up) {
            vertical += 1.0;
        }
        if action_state.pressed(PlayerAction::Down) {
            vertical -= 1.0;
        }
        if action_state.pressed(PlayerAction::Left) {
            horizontal -= 1.0;
        }
        if action_state.pressed(PlayerAction::Right) {
            horizontal += 1.0;
        }

        let mut new_player_position_horizontal =
            player_transform.translation.x + horizontal * PLAYER_SPEED * time_step.delta_seconds();

        let mut new_player_position_vertical =
            player_transform.translation.y + vertical * PLAYER_SPEED * time_step.delta_seconds();

        if action_state.pressed(PlayerAction::Move) {
            // We're working with gamepads, so we want to defensively ensure that we're using the clamped values
            let axis_pair = action_state.clamped_axis_pair(PlayerAction::Move).unwrap();

            velocity.0.x = axis_pair.x();
            velocity.0.y = axis_pair.y();
            direction.direction.x = axis_pair.x();
            direction.direction.y = axis_pair.y();

            new_player_position_horizontal = player_transform.translation.x
                + velocity.0.x * PLAYER_SPEED * time_step.delta_seconds();
            new_player_position_vertical = player_transform.translation.y
                + velocity.0.y * PLAYER_SPEED * time_step.delta_seconds();
        }

        // Update the player position,
        // making sure it doesn't cause the player to leave the arena
        let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + PLAYER_SIZE.x / 2.0 + PLAYER_PADDING;
        let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - PLAYER_SIZE.x / 2.0 - PLAYER_PADDING;
        let top_bound = TOP_WALL - WALL_THICKNESS / 2.0 - PLAYER_SIZE.y / 2.0 - PLAYER_PADDING;
        let bottom_bound =
            BOTTOM_WALL + WALL_THICKNESS / 2.0 + PLAYER_SIZE.y / 2.0 + PLAYER_PADDING;

        player_transform.translation.x =
            new_player_position_horizontal.clamp(left_bound, right_bound);
        player_transform.translation.y =
            new_player_position_vertical.clamp(bottom_bound, top_bound);
    }
}

fn collision_with_enemy(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>,
    player_query: Query<(Entity, &NewPlayer), With<NewPlayer>>,
    rapier_context: Res<RapierContext>,
    mut p2_lives: ResMut<crate::Player2Lives>,
    mut p1_lives: ResMut<crate::Player1Lives>,
) {
    let enemy = enemy_query.single();

    for (player, new_player) in player_query.iter() {
        if let Some(contact_pair) = rapier_context.contact_pair(player, enemy) {
            if contact_pair.has_any_active_contacts() {
                commands.insert_resource(NextState(Some(GameState::Paused)));

                match new_player {
                    NewPlayer::One => p1_lives.lives -= 1,
                    NewPlayer::Two => p2_lives.lives -= 1,
                }
            }
        }
    }
}
