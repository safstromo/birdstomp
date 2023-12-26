// use crate::collisions::Collider;
use crate::resources::{DifficultyTimer, SpawnTimer};
use crate::{AppState, GameState, LEFT_WALL, RIGHT_WALL, TOP_WALL};
use bevy::prelude::*;
use rand::Rng;

use crate::sprites::{AnimationIndices, AnimationTimer};

const SPEED: f32 = 500.0;
const INITIAL_CAR_DIRECTION: Vec2 = Vec2::new(0.0, -0.5);
pub const INITIAL_CAR_SPEED: f32 = 400.0;
pub const CAR_SIZE: Vec2 = Vec2::new(20.0, 50.0);

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
            .insert_resource(DifficultyTimer(Timer::from_seconds(
                1.0,
                TimerMode::Repeating,
            )))
            .add_systems(
                FixedUpdate,
                (spawn_car, apply_velocity)
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(OnExit(AppState::InGame), despawn_cars);
    }
}

fn spawn_car(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut rng = rand::thread_rng();
    if timer.0.tick(time.delta()).just_finished() {
        let texture_handle = asset_server.load("monsters/tooth-walker/toothwalker-sheet.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 6, 5, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        // Use only the subset of sprites in the sheet that make up the run animation
        let animation_indices = AnimationIndices { first: 0, last: 5 };

        commands.spawn((
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
            Car,
            Velocity(INITIAL_CAR_DIRECTION.normalize() * SPEED),
            // Collider,
        ));
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<Time<Fixed>>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time_step.delta_seconds();
        transform.translation.y += velocity.y * time_step.delta_seconds();
    }
}
//
// fn increase_difficulty(
//     time: Res<Time>,
//     mut timer: ResMut<DifficultyTimer>,
//     mut car_speed: ResMut<CarSpeed>,
// ) {
//     timer.0.tick(time.delta());
//
//     if timer.0.just_finished() {
//         car_speed.speed += 20.0;
//     }
// }

fn despawn_cars(mut commands: Commands, cars_query: Query<Entity, With<Car>>) {
    for car in cars_query.iter() {
        commands.entity(car).despawn();
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component, Debug)]
pub struct Car;
