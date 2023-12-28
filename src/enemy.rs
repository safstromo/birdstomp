use crate::player::Player;
// use crate::resources::SpawnTimer;
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

fn spawn_enemy(
    mut commands: Commands,
    // time: Res<Time>,
    // mut timer: ResMut<SpawnTimer>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut rng = rand::thread_rng();
    // if timer.0.tick(time.delta()).just_finished() {
    let texture_handle = asset_server.load("monsters/tooth-walker/toothwalker-sheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 6, 5, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 5 };

    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices.first),
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
    // .with_children(|children| {
    //     children
    //         .spawn(Collider::cuboid(6.0, 10.0))
    //         .insert(TransformBundle::from(Transform::from_xyz(4.0, 0.0, 0.0)));
    //
    // });
}

//TODO: move towatds closest player

fn move_enemy_toward_player(
    mut query: Query<(&mut Transform, &mut Enemy)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    for (mut transform, mut enemy) in query.iter_mut() {
        let player_transform = player_query.single();
        let direction = player_transform.translation - transform.translation;
        let direction = direction.normalize();

        let distance = player_transform.translation.distance(transform.translation);

        // println!("distance to player: {}", distance);
        // println!("enemy speed: {}", enemy.speed);

        //TODO adjust speed speeds and radius

        // Slow down when close to the player.
        let slow_down_radius = 100.0;
        if distance < slow_down_radius {
            enemy.speed = enemy.current_speed * 0.5;
        }
        if distance > slow_down_radius {
            enemy.speed = enemy.current_speed;
        }

        transform.translation += direction * enemy.speed * time.delta_seconds();
        enemy.current_speed += 0.8;
    }
}

fn despawn_enemy(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for enemy in enemy_query.iter() {
        commands.entity(enemy).despawn();
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
