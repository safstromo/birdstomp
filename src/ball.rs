use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use crate::player::Player;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball)
            .add_systems(Update, (snap_to_player, move_ball, throw_ball));
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
    commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(6.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::BLACK)),
                transform: Transform::from_xyz(0.0, 0.0, 2.0),
                ..default()
            },
            Ball {
                velocity: Vec2::new(0.0, 0.0),
            },
        ))
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(30.0))
        .insert(ActiveEvents::COLLISION_EVENTS);
}

// TODO - Snap to player

fn snap_to_player(
    mut commands: Commands,
    mut event_reader: EventReader<CollisionEvent>,
    ball_query: Query<Entity, With<Ball>>,
    players: Query<Entity, With<KinematicCharacterController>>,
    ballhandler: Query<Entity, With<BallHandler>>,
    mut score: ResMut<crate::Score>, // TODO - remove this
) {
    let ball = ball_query.single();

    if !ballhandler.is_empty() {
        return;
    }

    for event in event_reader.read() {
        match event {
            CollisionEvent::Started(collider1, collider2, _event) => {
                if ball.index() == collider1.index() {
                    for player in players.iter() {
                        if player.index() == collider2.index() {
                            score.score += 1;
                            commands.entity(player).insert(BallHandler);
                            println! {"ball hit {:?}", player};
                        }
                    }
                }
                if ball.index() == collider2.index() {
                    for player in players.iter() {
                        if player.index() == collider2.index() {
                            score.score += 1;
                            println! {"ball hit {:?}", player};
                            commands.entity(player).insert(BallHandler);
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

//TODO: Fix ball movement/velocity
fn move_ball(
    mut ball_query: Query<(&mut Ball, &mut Transform, With<Ball>)>,
    ballhandler: Query<&mut Transform, (With<BallHandler>, Without<Ball>)>,
) {
    let (ball, mut ball_transform, ()) = ball_query.single_mut();

    if ballhandler.is_empty() {
        ball_transform.translation =
            ball_transform.translation + Vec3::new(ball.velocity.x, ball.velocity.y, 0.0) * 0.1;
        return;
    }

    let ballhandler = ballhandler.single();

    ball_transform.translation = ballhandler.translation;
}

//TODO: Fix input key and controller
fn throw_ball(
    mut commands: Commands,
    ballhandler: Query<(Entity, &mut Player, With<BallHandler>)>,
    mut ball_query: Query<&mut Ball>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if ballhandler.is_empty() {
        return;
    }

    let (entity, ballhandler, _) = ballhandler.single();
    let mut ball = ball_query.single_mut();

    if keyboard_input.pressed(KeyCode::T) {
        ball.velocity = ballhandler.direction * BALL_SPEED;
        commands.entity(entity).remove::<BallHandler>();
    }
}
