use crate::{
    direction_indicator::DirectionIndicator,
    gamepad::PlayerAction,
    player::{Player, PlayerDirection},
};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball)
            // .add_systems(Update, (snap_to_player, move_ball, throw_ball));
            .add_systems(Update, (snap_to_player, throw_ball));
    }
}
const BALL_SPEED: f32 = 500.0;

#[derive(Component)]
pub struct Ball {
    velocity: Vec2,
}

#[derive(Component)]
pub struct BallHandler;

// TODO - add ball sprite
fn spawn_ball(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let ball_entity = commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(6.)).into(),
                material: materials.add(ColorMaterial::from(Color::BLACK)),
                transform: Transform::from_xyz(0.0, 0.0, 2.0),
                ..default()
            },
            Ball {
                velocity: Vec2::new(0.0, 0.0),
            },
        ))
        .insert(RigidBody::Dynamic)
        .insert(Ccd::enabled())
        .insert(Collider::ball(16.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .id();

    info!("Spawned ball entity: {:?}", ball_entity);
}

fn snap_to_player(
    mut commands: Commands,
    mut event_reader: EventReader<CollisionEvent>,
    ball_query: Query<Entity, With<Ball>>,
    players: Query<Entity, With<KinematicCharacterController>>,
    ballhandler: Query<Entity, With<BallHandler>>,
) {
    if !ballhandler.is_empty() {
        return;
    }

    if ball_query.is_empty() {
        return;
    }

    let ball = ball_query.single();

    for event in event_reader.read() {
        if let CollisionEvent::Started(collider1, collider2, _event) = event {
            info!(
                "Collision detected between: {:?} and {:?}",
                collider1, collider2
            );
            if ball == *collider1 || ball == *collider2 {
                for player in players.iter() {
                    if player == *collider2 || player == *collider1 {
                        info!("Player entity involved in collision: {:?}", player);
                        commands.entity(player).insert(BallHandler);
                        info!("BallHandler component added to player");
                        // commands.entity(ball).remove::<RigidBody>();
                        commands.entity(ball).despawn();
                        info!("ball removed")
                    }
                }
            }
        }
    }
}

// //TODO: Fix ball movement/velocity
// fn move_ball(
//     mut ball_query: Query<(&mut Ball, &mut Transform, Has<Ball>)>,
//     ballhandler: Query<&mut Transform, (With<BallHandler>, Without<Ball>)>,
//     indicator: Query<&DirectionIndicator>,
// ) {
//     let (_, mut ball_transform, _) = ball_query.single_mut();
//
//     if ballhandler.is_empty() {
//         return;
//     }
//     let indicator = indicator.single();
//     let ballhandler = ballhandler.single();
//
//     //TODO: this is not centered
//     let offset_direction = indicator.direction * 30.0;
//
//     let new_x = offset_direction.x + ballhandler.translation.x;
//     let new_y = offset_direction.y + ballhandler.translation.y;
//     ball_transform.translation = Vec3::new(new_x, new_y, 0.0);
// }

//TODO: Refactor spawning ball

fn throw_ball(
    mut commands: Commands,
    ballhandler: Query<(Entity, &ActionState<PlayerAction>, &Transform), With<BallHandler>>,
    indicator: Query<&DirectionIndicator>,
    mut event_reader: EventReader<CollisionEvent>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if ballhandler.is_empty() {
        return;
    }

    let (ballhandler_entity, action, ballhandler_transform) = ballhandler.single();
    let indicator = indicator.get_single().unwrap();

    if action.just_pressed(&PlayerAction::Throw) {
        let impulse_strength = 500000.0; // Adjust for desired throwing power
        let collider_mprops = ColliderMassProperties::Density(0.5); // Adjust density as needed

        // Spawn ball in the middle of the screen and set move direction straight up
        let screen_center = Vec3::new(0.0, 0.0, 2.0);
        let move_direction = indicator.direction.normalize();

        let ball = commands
            .spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(6.)).into(),
                    material: materials.add(ColorMaterial::from(Color::BLACK)),
                    transform: Transform::from_translation(ballhandler_transform.translation),
                    ..default()
                },
                Ball {
                    velocity: move_direction * BALL_SPEED,
                },
            ))
            .insert(RigidBody::Dynamic)
            .insert(Ccd::enabled())
            .insert(Collider::ball(16.0))
            .insert(collider_mprops)
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(ExternalImpulse {
                impulse: move_direction * impulse_strength,
                torque_impulse: 0.0, // Optional: Set a torque impulse for spinning throw (default 0)
            })
            .id();

        info!("Spawned ball entity: {:?}", ball);
        commands.entity(ballhandler_entity).remove::<BallHandler>();
    }
}
