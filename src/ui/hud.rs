use bevy::prelude::*;
use crate::{AppState, GameState};
use crate::resources::{Lives, Score};
use crate::ui::styles::{get_hud_text_style};

#[derive(Component)]
pub struct Hud;

#[derive(Component)]
pub struct LifeBox;

#[derive(Component)]
pub struct ScoreBoard;


pub fn spawn_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>, ) {
    build_hud(&mut commands, &asset_server);
}

pub fn despawn_hud(
    mut commands: Commands,
    menu_query: Query<Entity, With<Hud>>, ) {
    if let Ok(menu_entity) = menu_query.get_single() {
        commands.entity(menu_entity).despawn_recursive();
    }
}

fn build_hud(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let hud = commands.spawn(
        (NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        },
         Hud,
        ))
        //Hp Box
        .with_children(|parent| {
            parent.spawn(
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(120.0),
                        width: Val::Px(600.0),
                        height: Val::Px(120.0),
                        ..default()
                    },
                    ..default()
                }
            ).with_children(|parent| {
                parent.spawn(
                    (TextBundle::from_sections([
                        TextSection::new(
                            "Lives: ",
                            get_hud_text_style(asset_server),
                        ),
                        TextSection::from_style(
                            TextStyle {
                                font_size: 64.0,
                                color: Color::RED,
                                ..default()
                            }
                        ),
                    ]),
                     LifeBox
                    ));
                parent.spawn(
                    (TextBundle::from_sections([
                        TextSection::new(
                            "Score: ",
                            get_hud_text_style(asset_server),
                        ),
                        TextSection::from_style(
                            TextStyle {
                                font_size: 64.0,
                                color: Color::RED,
                                ..default()
                            }
                        ),
                    ]),
                     ScoreBoard
                    ));
            });
        })
        .id();
    hud
}


pub fn update_lives(
    mut commands: Commands,
    life: Res<Lives>,
    mut query: Query<&mut Text, With<LifeBox>>,
) {
    let mut text = query.single_mut();  //fails multiple entries
    if life.lives == 0 {
        commands.insert_resource(NextState(Some(GameState::Paused)));
        commands.insert_resource(NextState(Some(AppState::GameOver)));
    }
    text.sections[1].value = life.lives.to_string();
}

pub fn update_score(
    score: Res<Score>,
    mut query: Query<&mut Text, With<ScoreBoard>>,
) {
    let mut text = query.single_mut();
    text.sections[1].value = score.score.to_string();
}

