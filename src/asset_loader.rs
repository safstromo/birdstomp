use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub player: Handle<Image>,
    pub enemy: Handle<Image>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        player: asset_server.load("duckyatlas.png"),
        enemy: asset_server.load("monsters/tooth-walker/toothwalker-sheet.png"),
    }
}
