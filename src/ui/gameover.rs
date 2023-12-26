use bevy::prelude::*;
use crate::ui::buttons::{PlayButton, QuitButton};
use crate::ui::styles::{BUTTON_STYLE, CENTER_ROW, FULL_CENTER_COL, get_chicken_image_bundle, NORMAL_BUTTON_COLOR, spawn_button_text_box, spawn_end_score, spawn_title_box};

#[derive(Component)]
pub struct GameOver;

pub fn spawn_gameover(
    mut commands: Commands,
    asset_server: Res<AssetServer>, ) {
    build_gameover(&mut commands, &asset_server);
}

pub fn despawn_gameover(
    mut commands: Commands,
    menu_query: Query<Entity, With<GameOver>>, ) {
    if let Ok(gameover_entity) = menu_query.get_single() {
        commands.entity(gameover_entity).despawn_recursive();
    }
}

fn build_gameover(commands: &mut Commands,
                  asset_server: &Res<AssetServer>,
) -> Entity {
    let gameover_entity = commands
        .spawn((NodeBundle {
            style: FULL_CENTER_COL,
            ..default()
        },
                GameOver,
        ))
        .with_children(|parent| {
            //Final score
            parent.spawn(
                NodeBundle {
                    style: CENTER_ROW,
                    ..default()
                }
            ).with_children(|parent| {
                spawn_end_score(asset_server, parent);
            });
        }).with_children(|parent| {
        //title
        parent.spawn(
            NodeBundle {
                style: CENTER_ROW,
                ..default()
            }).with_children(|parent| {
            //image
            parent.spawn(
                get_chicken_image_bundle(asset_server));
            //Title text
            spawn_title_box(asset_server, parent, "Game Over");
            //image
            parent.spawn(
                get_chicken_image_bundle(asset_server));
        }
        );
        //restartbutton
        parent.spawn(
            (
                ButtonBundle {
                    style: BUTTON_STYLE,
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                PlayButton,
            )
        ).with_children(|parent| {
            spawn_button_text_box(asset_server, parent, "Restart")
        });
        //quitbutton
        parent.spawn(
            (
                ButtonBundle {
                    style: BUTTON_STYLE,
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                QuitButton
            )
        )
            .with_children(|parent| {
                spawn_button_text_box(asset_server, parent, "Quit")
            });
    })
        .id();
    gameover_entity
}






