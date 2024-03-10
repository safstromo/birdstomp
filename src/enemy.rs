use crate::ball::BallHandler;
use crate::{AppState, GameState, LEFT_WALL, RIGHT_WALL, TOP_WALL};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::sprites::{AnimationIndices, AnimationTimer};

pub const INITIAL_SPEED: f32 = 400.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemy)
            .add_systems(
                FixedUpdate,
                (move_enemy_toward_player)
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(OnExit(AppState::InGame), despawn_enemy);
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component, Debug)]
pub struct Enemy {
    pub speed: f32,
    pub current_speed: f32,
    pub health: f32,
}

fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut rng = rand::thread_rng();
    let texture = asset_server.load("monsters/tooth-walker/toothwalker-sheet.png");
    // let texture_atlas =
    // TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 6, 5, None, None);
    let layout = TextureAtlasLayout::from_grid(Vec2::new(64.0, 64.0), 6, 5, None, None);
    let texture_atlas_layout = texture_atlases_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 5 };

    commands
        .spawn((
            SpriteSheetBundle {
                // texture_atlas: texture_atlas_handle,
                // texture: texture_handle,
                // sprite: Sprite::new(animation_indices.first),
                //
                texture,
                atlas: TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
                transform: Transform {
                    translation: Vec3::new(rng.gen_range(LEFT_WALL..RIGHT_WALL), TOP_WALL, 1.),
                    scale: Vec3::splat(2.0),
                    ..default()
                },
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Enemy {
                speed: INITIAL_SPEED,
                current_speed: INITIAL_SPEED,
                health: 100.0,
            },
        ))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(6.0, 10.0));
}

fn move_enemy_toward_player(
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    ballhandler_query: Query<&Transform, (With<BallHandler>, Without<Enemy>)>,
    time: Res<Time>,
) {
    if ballhandler_query.is_empty() {
        return;
    }

    let ballhandler_transform = ballhandler_query.single();

    let (mut enemy_transform, mut enemy) = enemy_query.single_mut();

    let direction = ballhandler_transform.translation - enemy_transform.translation;
    let direction = direction.normalize();

    let distance = ballhandler_transform
        .translation
        .distance(enemy_transform.translation);

    //TODO adjust speed speeds and radius

    // Slow down when close to the player.
    let slow_down_radius = 100.0;
    if distance < slow_down_radius {
        enemy.speed = enemy.current_speed * 0.5;
    }
    if distance > slow_down_radius {
        enemy.speed = enemy.current_speed;
    }

    enemy_transform.translation += direction * enemy.speed * time.delta_seconds();
    enemy.current_speed += 0.8;
}

fn despawn_enemy(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for enemy in enemy_query.iter() {
        commands.entity(enemy).despawn_recursive();
    }
}
