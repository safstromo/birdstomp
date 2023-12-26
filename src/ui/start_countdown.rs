use bevy::prelude::*;

use crate::GameState;
use crate::resources::CountdownTimer;
use crate::ui::styles::spawn_title_box;

#[derive(Component, Debug)]
pub struct Countdown;


pub fn countdown(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<CountdownTimer>,
    asset_server: Res<AssetServer>,
    countdown_query: Query<Entity, With<Countdown>>,
) {
    timer.timer.tick(time.delta());
    if timer.timer.just_finished() {
        if timer.duration == 4 {
            spawn_countdown(&mut commands, &asset_server, "3..");
        } else if timer.duration == 3 {
            despawn_countdown(&mut commands, &countdown_query);
            spawn_countdown(&mut commands, &asset_server, "2..");
        } else if timer.duration == 2 {
            despawn_countdown(&mut commands, &countdown_query);
            spawn_countdown(&mut commands, &asset_server, "1..");
        } else if timer.duration == 1 {
            despawn_countdown(&mut commands, &countdown_query);
            spawn_countdown(&mut commands, &asset_server, "GO..");
            commands.insert_resource(NextState(Some(GameState::Running)));
        } else if timer.duration == 0 {
            despawn_countdown(&mut commands, &countdown_query);
        }
        if timer.duration != 0 {
            timer.duration -= 1;
        }
    }
}

fn spawn_countdown(commands: &mut Commands, asset_server: &Res<AssetServer>, countdown: &str) -> Entity {
    let countdown = commands
        .spawn((NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        },
                Countdown,
        ))
        .with_children(|parent| {
            spawn_title_box(asset_server, parent, countdown);
        })

        .id();
    countdown
}

//can refactor to a single despawn!!
pub fn despawn_countdown(
    commands: &mut Commands,
    countdown_query: &Query<Entity, With<Countdown>>, ) {
    if let Ok(countdown_entity) = countdown_query.get_single() {
        commands.entity(countdown_entity).despawn_recursive();
    }
}
