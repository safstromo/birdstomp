use crate::gamepad::PlayerAction;
use crate::resources::{CountdownTimer, Player1Lives, Player2Lives};
use crate::ui::styles::*;
use crate::AppState;
use bevy::app::AppExit;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct QuitButton;

pub fn interact_with_play_button(
    mut life: ResMut<Player1Lives>,
    mut countdown: ResMut<CountdownTimer>,
    mut score: ResMut<Player2Lives>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                life.lives = 5;
                score.lives = 5;
                countdown.duration = 4;
                app_state_next_state.set(AppState::InGame);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn start(
    mut life: ResMut<Player1Lives>,
    mut countdown: ResMut<CountdownTimer>,
    mut score: ResMut<Player2Lives>,
    mut button_query: Query<(&mut BackgroundColor, With<PlayButton>)>,
    start_action: Query<&ActionState<PlayerAction>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    let (mut background_color, _) = button_query.single_mut();

    for start in start_action.iter() {
        if start.pressed(PlayerAction::Start) {
            *background_color = PRESSED_BUTTON_COLOR.into();
            life.lives = 5;
            score.lives = 5;
            countdown.duration = 4;
            app_state_next_state.set(AppState::InGame);
        }
    }
}

pub fn interact_with_quit_button(
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_exit_event_writer.send(AppExit);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}
