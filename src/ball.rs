use crate::{
    direction_indicator::DirectionIndicator,
    gamepad::PlayerAction,
    player::{Player, PlayerDirection},
};
use bevy::{ecs::schedule::common_conditions, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball)
            // .add_systems(Update, (snap_to_player, move_ball, throw_ball));
            .add_systems(Update, (snap_to_player, throw_ball, return_ball));
    }
}
const BALL_SPEED: f32 = 500.0;

#[derive(Component, Debug)]
pub struct Ball {
    velocity: Vec2,
    despawn_timer: f32,
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
                despawn_timer: 4.0,
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
                    if !ballhandler.is_empty() && player == ballhandler.get_single().unwrap() {
                        break;
                    }
                    if player == *collider2 || player == *collider1 {
                        info!("Player entity involved in collision: {:?}", player);
                        commands.entity(player).insert(BallHandler);
                        info!("BallHandler component added to player");
                        commands.entity(ball).despawn();
                        info!("ball removed");
                        if !ballhandler.is_empty() {
                            commands
                                .entity(ballhandler.get_single().unwrap())
                                .remove::<BallHandler>();
                        }
                    }
                }
            }
        }
    }
}

//TODO: Refactor spawning ball
fn throw_ball(
    mut commands: Commands,
    ballhandler: Query<(Entity, &ActionState<PlayerAction>, &Transform), With<BallHandler>>,
    indicator: Query<&DirectionIndicator>,
    mut event_reader: EventReader<CollisionEvent>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    ball_query: Query<Entity, With<Ball>>,
) {
    if ballhandler.is_empty() {
        return;
    }

    if !ball_query.is_empty() {
        return;
    }

    let (ballhandler_entity, action, ballhandler_transform) = ballhandler.single();
    let indicator = indicator.get_single().unwrap();

    if action.just_pressed(&PlayerAction::Throw) {
        let impulse_strength = 500000.0; // Adjust for desired throwing power
        let collider_mprops = ColliderMassProperties::Density(0.5); // Adjust density as needed

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
                    despawn_timer: 4.0,
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
    }
}

fn return_ball(
    mut commands: Commands,
    ballhandler: Query<(Entity, &ActionState<PlayerAction>, &Transform), With<BallHandler>>,
    mut ball_query: Query<(Entity, &mut Ball), With<Ball>>,
    time: Res<Time>,
) {
    if ball_query.is_empty() || ballhandler.is_empty() {
        return;
    }

    let (ball_enity, mut ball) = ball_query.get_single_mut().unwrap();
    info!("Ball velocity: {:?}", ball.despawn_timer);

    ball.despawn_timer -= time.delta_seconds();

    if ball.despawn_timer <= 0.0 {
        commands.entity(ball_enity).despawn();
    }
}
