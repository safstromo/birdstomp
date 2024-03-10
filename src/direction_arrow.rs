use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct DirectionArrow;

impl Plugin for DirectionArrow {
    fn build(&self, app: &mut App) {
        // app.add_systems(
        //     Update,
        //     (move_arrow)
        //         .run_if(in_state(AppState::InGame))
        //         .run_if(in_state(GameState::Running)),
        // );
        // app.add_systems(Startup, spawn_arrow);
    }
}

pub fn move_arrow() {}

pub fn spawn_arrow(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let triangle = Mesh2dHandle(meshes.add(Triangle2d::new(
        Vec2::Y * 8.0,
        Vec2::new(-8.0, -8.0),
        Vec2::new(8.0, -8.0),
    )));

    commands.spawn(MaterialMesh2dBundle {
        mesh: triangle,
        material: materials.add(Color::TOMATO),
        transform: Transform::from_xyz(10.0, 10.0, 0.0),
        ..Default::default()
    });
}
