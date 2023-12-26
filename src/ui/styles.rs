use bevy::prelude::*;
use crate::ui::hud::ScoreBoard;

pub const NORMAL_BUTTON_COLOR: Color = Color::BLACK;
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);
pub const BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(200.0);
    style.height = Val::Px(80.0);
    style
};

pub const CENTER_ROW: Style = {
    let mut style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(300.0);
    style.height = Val::Px(120.0);
    style
};
pub const FULL_CENTER_COL: Style = {
    let mut style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.row_gap = Val::Px(20.0);
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style
};

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
    }
}

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 64.0,
        color: Color::BLACK,
    }
}

pub fn get_chicken_image_bundle(asset_server: &Res<AssetServer>) -> ImageBundle {
    ImageBundle {
        style: Style {
            width: Val::Px(32.0),
            height: Val::Px(32.0),
            margin: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0)),
            ..default()
        },
        image: asset_server.load("chicken1.png").into(),
        ..default()
    }
}

pub fn spawn_button_text_box(asset_server: &Res<AssetServer>, parent: &mut ChildBuilder, text: &str) {
    parent.spawn(
        TextBundle {
            text: Text {
                sections: vec![
                    TextSection::new(
                        text,
                        get_button_text_style(asset_server),
                    )
                ],
                alignment: TextAlignment::Center,
                ..default()
            },
            ..default()
        }
    );
}

pub fn spawn_title_box(asset_server: &Res<AssetServer>, parent: &mut ChildBuilder, text: &str) {
    parent.spawn(
        TextBundle {
            text: Text {
                sections: vec![
                    TextSection::new(
                        text,
                        get_title_text_style(asset_server),
                    )
                ],
                alignment: TextAlignment::Center,
                ..default()
            },
            ..default()
        });
}

pub fn spawn_end_score(asset_server: &Res<AssetServer>, parent: &mut ChildBuilder) {
    parent.spawn(
        (TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 64.0,
                    color: Color::BLACK,
                },
            ),
            TextSection::from_style(
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 64.0,
                    color: Color::BLACK,
                    ..default()
                }
            ),
        ]),
         ScoreBoard
        ));
}

pub fn get_hud_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 64.0,
        color: Color::BLACK,
    }
}
