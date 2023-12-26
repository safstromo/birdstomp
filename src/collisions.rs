// use crate::car::{Car, CAR_SIZE};
// use crate::events::CollisionEvent;
// use crate::player::Player;
// use crate::resources::{Lives, Score};
// use crate::{AppState, GameState, BOTTOM_WALL};
// use bevy::prelude::*;
// use bevy::sprite::collide_aabb::collide;
//
// pub struct CollisionPlugin;
//
// impl Plugin for CollisionPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(
//             FixedUpdate,
//             check_for_collisions
//                 .run_if(in_state(AppState::InGame))
//                 .run_if(in_state(GameState::Running)),
//         );
//     }
// }
//
// fn check_for_collisions(
//     mut commands: Commands,
//     mut scoreboard: ResMut<Score>,
//     mut lives: ResMut<Lives>,
//     mut player_query: Query<(Entity, &Transform), With<Player>>,
//     collider_query: Query<(Entity, &Transform), With<Car>>,
//     mut collision_events: EventWriter<CollisionEvent>,
//     mut player_sprite_query: Query<(&Transform, &Handle<Image>), With<Player>>,
//     mut car_sprite_query: Query<(&Transform, &Handle<Image>), With<Car>>,
//     assets: Res<Assets<Image>>,
// ) {
//     let (_player_entity, player_transform) = player_query.single_mut();
//
//     // let player_size = get_player_size(&mut player_sprite_query, &assets);
//     // let car_size = get_car_size(&mut car_sprite_query, assets);
//
//     let player_size = Vec2::new(0.0, 0.0);
//     let car_size = Vec2::new(0.0, 0.0);
//
//     for (car_entity, car_transform) in collider_query.iter() {
//         if collide(
//             player_transform.translation,
//             player_size,
//             car_transform.translation,
//             car_size,
//         )
//         .is_some()
//         {
//             lives.lives -= 1;
//             collision_events.send_default();
//             commands.entity(car_entity).despawn();
//         }
//
//         if car_transform.translation.y <= BOTTOM_WALL - CAR_SIZE.y {
//             scoreboard.score += 1;
//             commands.entity(car_entity).despawn();
//         }
//     }
// }
//
// // fn get_player_size(
// //     sprite_query: &mut Query<(&Transform, &Handle<Image>), With<Player>>,
// //     assets: &Res<Assets<Image>>,
// // ) -> Vec2 {
// //     let mut player_size = Vec2::new(0.0, 0.0);
// //     for (transform, image_handle) in sprite_query.iter_mut() {
// //         let image_dimensions = assets.get(image_handle).unwrap().size();
// //
// //         let scaled_image_dimension = image_dimensions * transform.scale.truncate();
// //
// //         let bounding_box =
// //             Rect::from_center_size(transform.translation.truncate(), scaled_image_dimension);
// //         player_size = bounding_box.size();
// //     }
// //     player_size
// // }
// //
// // fn get_car_size(
// //     sprite_query: &mut Query<(&Transform, &Handle<Image>), With<Car>>,
// //     assets: Res<Assets<Image>>,
// // ) -> Vec2 {
// //     let mut car_size = Vec2::new(0.0, 0.0);
// //     for (transform, image_handle) in sprite_query.iter_mut() {
// //         if assets.get(image_handle).is_some() {
// //             let image_dimensions = assets.get(image_handle).unwrap().size();
// //
// //             let scaled_image_dimension = image_dimensions * transform..scale.truncate();
// //
// //             let bounding_box =
// //                 Rect::from_center_size(transform.translation.truncate(), scaled_image_dimension);
// //             car_size = bounding_box.size();
// //         }
// //     }
// //     car_size
// // }
// //
// #[derive(Component)]
// pub struct Collider;
