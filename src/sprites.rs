use crate::{AppState, GameState};
use bevy::prelude::*;

pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            animate_sprite
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameState::Running)),
        );
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut texture_atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            texture_atlas.index = if texture_atlas.index == indices.last {
                indices.first
            } else {
                texture_atlas.index + 1
            };
        }
    }
}

#[derive(Component)]
pub struct AnimationIndices {
    pub(crate) first: usize,
    pub(crate) last: usize,
}
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
