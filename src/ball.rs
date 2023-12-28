use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball);
    }
}

#[derive(Component)]
pub struct Ball {
    velocity: Vec2,
}

// TODO - add ball sprite
fn spawn_ball(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(6.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_xyz(0.0, 0.0, 2.0),
                ..default()
            },
            Ball {
                velocity: Vec2::new(0.0, 0.0),
            },
        ))
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(6.0));
}
