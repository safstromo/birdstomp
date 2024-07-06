use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use leafwing_input_manager::prelude::*;

use crate::{gamepad::PlayerAction, player::Player, GameState};

pub struct DirectionIndicatorPlugin;

impl Plugin for DirectionIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (move_indicator).run_if(in_state(GameState::Running)),
        );
    }
}

#[derive(Component)]
pub struct DirectionIndicator {
    pub direction: Vec2,
}

pub fn move_indicator(
    player_query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut indicator: Query<
        (&mut Transform, &mut DirectionIndicator),
        (With<DirectionIndicator>, Without<Player>),
    >,
) {
    if indicator.is_empty() {
        return;
    }

    // let player_action = player_query.single();
    let (mut indicator_transform, mut indicator) = indicator.single_mut();

    for player_action in player_query.into_iter() {
        if player_action.pressed(&PlayerAction::Aim) {
            let axis_pair = player_action.clamped_axis_pair(&PlayerAction::Aim).unwrap();

            let direction = Vec2::new(axis_pair.x(), axis_pair.y()).normalize();

            // Set the indicator's direction
            indicator.direction = direction;

            // Scale the normalized direction by the desired offset
            let offset_direction = direction * 30.0;

            // Set the indicator's position with the adjusted direction and offset
            indicator_transform.translation =
                Vec3::new(offset_direction.x, offset_direction.y, 0.0);
        }
    }
}

pub fn spawn_indicator(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    let circle = Mesh2dHandle(meshes.add(Circle { radius: 4.0 }));
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: circle,
            material: materials.add(Color::TOMATO),
            transform: Transform::from_xyz(0.0, 30.0, 0.0),
            ..Default::default()
        })
        .insert(DirectionIndicator {
            direction: Vec2::new(0.0, 0.0),
        })
        .id()
}
