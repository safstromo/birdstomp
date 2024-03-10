use crate::direction_indicator::spawn_indicator;
use crate::enemy::Enemy;
use crate::gamepad::PlayerAction;
use crate::resources::CountdownTimer;
use crate::sprites::{AnimationIndices, AnimationTimer};
use crate::{AppState, GameState, BOTTOM_WALL, LEFT_WALL, RIGHT_WALL, TOP_WALL, WALL_THICKNESS};
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

const PLAYER_SPEED: f32 = 500.0;
const PLAYER_PADDING: f32 = 10.0;
const PLAYER_SIZE: Vec2 = Vec2::new(5.0, 8.0);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (move_player, collision_with_enemy)
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameState::Running)),
        );
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
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
            player: Player {
                player_id: 0,
                lives: 3,
                gamepad: Gamepad { id: 0 },
            },
            direction: PlayerDirection {
                direction: Vec2::new(0.0, 0.0),
            },
            velocity: Velocity(Vec2::new(0.0, 0.0)),
            input_manager: InputManagerBundle::default(),
            sprite: SpriteSheetBundle::default(),
            //Idle animation
            animation_indices: AnimationIndices {
                first: 10,
                last: 13,
            },
            animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        }
    }
}

#[derive(Component, Debug)]
pub struct Player {
    pub player_id: usize,
    pub lives: u32,
    pub gamepad: Gamepad,
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct PlayerDirection {
    pub direction: Vec2,
}

pub fn spawn_player(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    input_map: InputMap<PlayerAction>,
    gamepad: Gamepad,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    let texture = asset_server.load("duckyatlas.png");
    // let texture_atlas =
    //     TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 5, 3, None, None);
    let layout = TextureAtlasLayout::from_grid(Vec2::new(64.0, 64.0), 5, 3, None, None);
    let texture_atlas_layout = texture_atlases_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices {
        first: 10,
        last: 13,
    };

    let triangle = Mesh2dHandle(meshes.add(Triangle2d::new(
        Vec2::Y * 8.0,
        Vec2::new(-8.0, -8.0),
        Vec2::new(8.0, -8.0),
    )));
    let arrow = spawn_indicator(commands, meshes, materials);
    let player = commands
        .spawn(PlayerBundle {
            player: Player {
                player_id: gamepad.id, //make this dynamic
                lives: 3,
                gamepad,
            },
            input_manager: InputManagerBundle {
                input_map,
                ..Default::default()
            },
            sprite: SpriteSheetBundle {
                // texture_atlas: texture_atlas_handle,
                // sprite: Sprite::new(animation_indices.first),
                texture,
                atlas: TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
                transform: Transform::from_xyz(50.0, -250., 2.0),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(KinematicCharacterController::default())
        .insert(Collider::ball(10.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .id();

    commands.entity(player).add_child(arrow);

    return player;
}

fn move_player(
    mut query: Query<
        (
            &ActionState<PlayerAction>,
            &mut Transform,
            &mut Velocity,
            &mut PlayerDirection,
            &mut Sprite,
            &mut TextureAtlas,
        ),
        With<Player>,
    >,
    time_step: Res<Time<Fixed>>,
) {
    for (
        action_state,
        mut player_transform,
        mut velocity,
        mut direction,
        mut sprite,
        mut texture_atlas,
    ) in query.iter_mut()
    {
        let mut horizontal = 0.0;
        let mut vertical = 0.0;
        if action_state.pressed(&PlayerAction::Up) {
            vertical += 1.0;
        }
        if action_state.pressed(&PlayerAction::Down) {
            vertical -= 1.0;
        }
        if action_state.pressed(&PlayerAction::Left) {
            horizontal -= 1.0;
            sprite.flip_x = true;
        }
        if action_state.pressed(&PlayerAction::Right) {
            horizontal += 1.0;
            sprite.flip_x = false;
        }

        let mut new_player_position_horizontal =
            player_transform.translation.x + horizontal * PLAYER_SPEED * time_step.delta_seconds();

        let mut new_player_position_vertical =
            player_transform.translation.y + vertical * PLAYER_SPEED * time_step.delta_seconds();

        if action_state.pressed(&PlayerAction::Move) {
            // We're working with gamepads, so we want to defensively ensure that we're using the clamped values
            let axis_pair = action_state.clamped_axis_pair(&PlayerAction::Move).unwrap();

            velocity.0.x = axis_pair.x();
            velocity.0.y = axis_pair.y();
            direction.direction.x = axis_pair.x();
            direction.direction.y = axis_pair.y();

            new_player_position_horizontal = player_transform.translation.x
                + velocity.0.x * PLAYER_SPEED * time_step.delta_seconds();
            new_player_position_vertical = player_transform.translation.y
                + velocity.0.y * PLAYER_SPEED * time_step.delta_seconds();

            // if moved left or right flip sprite
            if velocity.0.x != 0.0 {
                sprite.flip_x = velocity.0.x < 0.0;
            }

            // idle animation or run animation
            if velocity.0.x != 0.0 || velocity.0.y != 0.0 {
                if texture_atlas.index < 4 || texture_atlas.index > 7 {
                    texture_atlas.index = 4;
                }
            } else {
                if texture_atlas.index < 10 || texture_atlas.index > 13 {
                    texture_atlas.index = 10;
                }
            }
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

// fn move_arrow(
//     mut parent: Query<(&ActionState<PlayerAction>, &mut Transform), With<Player>>,
//
//     time_step: Res<Time<Fixed>>,
// ) {
// }

fn collision_with_enemy(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>,
    mut player_query: Query<(Entity, &Player), With<Player>>,
    rapier_context: Res<RapierContext>,
    mut p2_lives: ResMut<crate::Player2Lives>,
    mut p1_lives: ResMut<crate::Player1Lives>,
    mut countdown: ResMut<CountdownTimer>,
) {
    let enemy = enemy_query.single();

    for (entity, player) in player_query.iter_mut() {
        if let Some(contact_pair) = rapier_context.contact_pair(entity, enemy) {
            if contact_pair.has_any_active_contacts() {
                commands.insert_resource(NextState(Some(GameState::Paused)));

                if player.player_id == 0 {
                    p1_lives.lives -= 1;
                    println!("Player 1 has {} lives left", p1_lives.lives);
                } else {
                    p2_lives.lives -= 1;
                    println!("Player 2 has {} lives left", p2_lives.lives);
                }
                countdown.duration = 4;
            }
        }
    }
}
