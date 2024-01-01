use crate::gamepad::PlayerAction;
use crate::player::Player;
use crate::resources::{CountdownTimer, JoinedPlayers};
use crate::ui::styles::*;
use crate::AppState;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use super::PlayerBox;

#[derive(Component)]
pub struct StartMenu;

pub fn spawn_start_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_start_menu(&mut commands, &asset_server);
}

pub fn despawn_start_menu(
    mut commands: Commands,
    start_menu_query: Query<Entity, With<StartMenu>>,
    player_box_query: Query<Entity, With<PlayerBox>>,
) {
    if let Ok(start_menu_entity) = start_menu_query.get_single() {
        commands.entity(start_menu_entity).despawn_recursive();
    }
    for entity in player_box_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn start(
    mut countdown: ResMut<CountdownTimer>,
    start_action: Query<&ActionState<PlayerAction>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    for start in start_action.iter() {
        if start.pressed(PlayerAction::Start) {
            countdown.duration = 4;
            app_state_next_state.set(AppState::InGame);
        }
    }
}

pub fn add_player_boxes(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    players: Query<&Player>,
) {
    if players.is_empty() {
        return;
    }

    for player in players.iter() {
        let playerid = player.player_id + 1;
        commands
            .spawn((
                NodeBundle {
                    style: BUTTON_STYLE,
                    ..default()
                },
                PlayerBox {
                    player_id: player.player_id,
                },
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            format!("Player {}", playerid.to_string()),
                            get_button_text_style(&asset_server),
                        )],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                });
            });
    }
}

pub fn despawn_player_boxes(
    mut commands: Commands,
    player_box_query: Query<(Entity, &PlayerBox)>,
    player_query: Query<&Player>,
) {
    if player_box_query.is_empty() {
        return;
    }
    for (entity, player_box) in player_box_query.iter() {
        if player_query
            .iter()
            .any(|player| player.player_id == player_box.player_id)
        {
            continue;
        }
        commands.entity(entity).despawn_recursive();
    }
}

fn build_start_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let start_menu_entity = commands
        .spawn((
            NodeBundle {
                style: FLEX_FULL_CENTER_COL,
                ..default()
            },
            StartMenu,
        ))
        .with_children(|parent| {
            //title
            parent
                .spawn(NodeBundle {
                    style: CENTER_ROW,
                    ..default()
                })
                .with_children(|parent| {
                    //image
                    parent.spawn(get_chicken_image_bundle(asset_server));
                    //Title text
                    spawn_title_box(asset_server, parent, "Stomp");
                    //image
                    parent.spawn(get_chicken_image_bundle(asset_server));
                });
            //Title
            spawn_title_box(asset_server, parent, "Press L1 + R1 to to join");
        })
        .id();
    start_menu_entity
}
