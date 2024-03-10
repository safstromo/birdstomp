mod buttons;
mod gameover;
mod hud;
mod menu;
mod start;
mod start_countdown;
mod styles;

use crate::resources::CountdownTimer;
use crate::ui::start::{despawn_start_menu, spawn_start_menu, start};
use crate::ui::start_countdown::countdown;
use crate::AppState;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use self::start::{add_player_boxes, despawn_player_boxes};
mod helpers;

#[derive(Component, Debug)]
struct GameBackground;

#[derive(Component, Debug)]
pub struct PlayerBox {
    pub player_id: usize,
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CountdownTimer>()
            .add_plugins(TilemapPlugin)
            .add_plugins(helpers::tiled::TiledMapPlugin)
            .add_systems(Startup, spawn_game_background)
            .add_systems(OnEnter(AppState::Menu), spawn_start_menu)
            .add_systems(OnExit(AppState::Menu), despawn_start_menu)
            // .add_systems(
            //     Update,
            //     (add_player_boxes, despawn_player_boxes).run_if(in_state(AppState::Menu)),
            // )
            // .add_systems(OnEnter(AppState::InGame), spawn_hud)
            // .add_systems(OnExit(AppState::InGame), despawn_hud)
            // .add_systems(OnEnter(AppState::GameOver), spawn_gameover)
            // .add_systems(OnExit(AppState::GameOver), despawn_gameover)
            .add_systems(Update, (toggle_appstate,))
            .add_systems(Update, (countdown).run_if(in_state(AppState::InGame)))
            // .add_systems(Update, (update_score).run_if(in_state(AppState::GameOver)))
            .add_systems(Update, (start).run_if(in_state(AppState::Menu)));

        // .add_systems(
        //     Update,
        //     (interact_with_play_button, interact_with_quit_button)
        //         .run_if(in_state(AppState::GameOver)),
        // );
    }
}

fn spawn_game_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_handle: Handle<helpers::tiled::TiledMap> = asset_server.load("map.tmx");

    commands.spawn((
        helpers::tiled::TiledMapBundle {
            tiled_map: map_handle,
            ..Default::default()
        },
        GameBackground,
    ));
}

fn _despawn_game_background(
    mut commands: Commands,
    menu_query: Query<Entity, With<GameBackground>>,
) {
    if let Ok(entity) = menu_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

fn toggle_appstate(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) && game_state.as_ref() == &AppState::InGame {
        println!("Appstate set to: Menu");
        commands.insert_resource(NextState(Some(AppState::Menu)));
    }
    if keyboard_input.just_pressed(KeyCode::KeyM) && game_state.as_ref() == &AppState::Menu {
        println!("Appstate set to: InGame");
        commands.insert_resource(NextState(Some(AppState::InGame)));
    }
}
