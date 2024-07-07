use crate::{
    direction_indicator::{spawn_indicator, DirectionIndicator},
    gamepad::PlayerAction,
    player::Player,
};

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball)
            .add_systems(Update, (snap_to_player, test_bug, return_ball));
    }
}
fn test_bug(action_query: Query<(&ActionState<PlayerAction>, &Player)>) {
    // Iterate through each player to see if they jumped
    for (action_state, player) in action_query.iter() {
        if action_state.just_pressed(&PlayerAction::Throw) {
            println!("Player {} dash!", player.gamepad.id);
        }
    }
}

#[derive(Component, Debug)]
pub struct Ball {
    despawn_timer: f32,
}

// #[derive(Component)]
// pub struct BallHandler;

// TODO - add ball sprite
fn spawn_ball(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let ball_entity = commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(bevy::math::prelude::Circle::new(6.)).into(),
                material: materials.add(ColorMaterial::from(Color::BLACK)),
                transform: Transform::from_xyz(0.0, 0.0, 2.0),
                ..default()
            },
            Ball { despawn_timer: 4.0 },
        ))
        .insert(RigidBody::Dynamic)
        .insert(Ccd::enabled())
        .insert(Collider::ball(16.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(GravityScale(0.0))
        .id();

    info!("Spawned ball entity: {:?}", ball_entity);
}

//TODO: Not sure if this works correctly
fn snap_to_player(
    mut commands: Commands,
    mut event_reader: EventReader<CollisionEvent>,
    ball_query: Query<Entity, With<Ball>>,
    mut players: Query<(Entity, &mut Player), With<KinematicCharacterController>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    direction_indicator: Query<Entity, With<DirectionIndicator>>,
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
                let mut old_ballhandler = None;
                let mut new_ballhandler = false;
                for (player_entity, mut player) in players.iter_mut() {
                    if player.have_ball {
                        old_ballhandler = Some(player);
                        continue;
                    }

                    if player_entity == *collider2 || player_entity == *collider1 {
                        info!("Player entity involved in collision: {:?}", player_entity);
                        player.have_ball = true;
                        new_ballhandler = true;
                        info!("BallHandler added to player");
                        commands.entity(ball).despawn();
                        info!("Ball removed");

                        info!("Adding direction indicator to new ballhandler");
                        let direction_indicator =
                            spawn_indicator(&mut commands, &mut meshes, &mut materials);
                        commands
                            .entity(player_entity)
                            .add_child(direction_indicator);
                    }
                }
                if old_ballhandler.is_some() && new_ballhandler {
                    old_ballhandler.unwrap().have_ball = false;
                    let direction_indicator = direction_indicator.get_single().unwrap();
                    info!("Despawning direction indicator");
                    commands.entity(direction_indicator).despawn();
                }
            }
        }
    }
}

fn throw_ball(
    mut commands: Commands,
    // ballhandler: Query<
    //     (Entity, &ActionState<PlayerAction>, &Transform, &Player),
    //     With<BallHandler>,
    // >,
    players: Query<(&ActionState<PlayerAction>, &Player)>,
    indicator: Query<(&DirectionIndicator, &Transform)>,
    // mut event_reader: EventReader<CollisionEvent>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    ball_query: Query<Entity, With<Ball>>,
) {
    if indicator.is_empty() {
        return;
    }

    if !ball_query.is_empty() {
        return;
    }

    // let (ballhandler_entity, action, ballhandler_transform, player) = ballhandler.single();
    let (indicator, indicator_transform) = indicator.get_single().unwrap();

    for (action, player) in players.iter() {
        if action.just_pressed(&PlayerAction::Throw) {
            info!("player {:?} pressed throw", player);
            // // Adjust for desired throwing power
            // let impulse_strength = 500000.0;
            // // Adjust density as needed
            // let collider_mprops = ColliderMassProperties::Density(0.5);
            //
            // let move_direction = indicator.direction.normalize();
            //
            // //TODO: fix this, translation??????
            // let ball = commands
            //     .spawn((
            //         MaterialMesh2dBundle {
            //             mesh: meshes.add(shape::Circle::new(6.)).into(),
            //             material: materials.add(ColorMaterial::from(Color::BLACK)),
            //             transform: Transform::from_translation(indicator_transform.translation),
            //             ..default()
            //         },
            //         Ball { despawn_timer: 4.0 },
            //     ))
            //     .insert(RigidBody::Dynamic)
            //     .insert(Ccd::enabled())
            //     .insert(Collider::ball(16.0))
            //     .insert(collider_mprops)
            //     .insert(ActiveEvents::COLLISION_EVENTS)
            //     .insert(ExternalImpulse {
            //         impulse: move_direction * impulse_strength,
            //         torque_impulse: 0.0, // Optional: Set a torque impulse for spinning throw (default 0)
            //     })
            //     .id();
            //
            // info!("Spawned ball entity: {:?}", ball);
        }

        if action.just_pressed(&PlayerAction::Dash) {
            info!("player {:?} pressed dash", player);
        }
    }
}

fn return_ball(
    mut commands: Commands,
    players: Query<(Entity, &Player, &Transform), With<Player>>,
    mut ball_query: Query<(Entity, &mut Ball), With<Ball>>,
    time: Res<Time>,
) {
    if ball_query.is_empty() {
        return;
    }

    for (player_entity, player, player_transform) in players.iter() {
        if player.have_ball {
            let (ball_enity, mut ball) = ball_query.get_single_mut().unwrap();
            // info!("Ball timer: {:?}", ball.despawn_timer);

            ball.despawn_timer -= time.delta_seconds();

            if ball.despawn_timer <= 0.0 {
                info!("Ball despawn timer at 0, returning ball");
                commands.entity(ball_enity).despawn();
            }
        }
    }
}
