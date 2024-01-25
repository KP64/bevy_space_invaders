use bevy::{app, prelude::*};

#[derive(Resource, Debug, Default)]
pub struct TextureAssets {
    pub invaders: Vec<Handle<Image>>,
    pub ufo: Handle<Image>,
    pub player: Handle<Image>,
    pub explosions: Explosions,
    pub shots: Shots,
}

#[derive(Debug, Default)]
pub struct Explosions {
    pub player: Handle<Image>,
    pub enemy: Handle<Image>,
}

#[derive(Debug, Default)]
pub struct Shots {
    pub enemy: [Handle<Image>; 2],
}

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TextureAssets>()
            .add_systems(PreStartup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<TextureAssets>, asset_server: Res<AssetServer>) {
    let enemy_types = ["A1", "A2", "B1", "B2", "C1", "C2"];
    let invaders = enemy_types
        .iter()
        .map(|enemy_type| format!("invader_{enemy_type}.png"))
        .map(|enemy| asset_server.load(enemy))
        .collect::<Vec<_>>();

    *scene_assets = TextureAssets {
        invaders,
        ufo: asset_server.load("UFO.png"),
        player: asset_server.load("Player.png"),
        explosions: Explosions {
            player: asset_server.load("PlayerExplosion.png"),
            enemy: asset_server.load("EnemyExplosion.png"),
        },
        shots: Shots {
            enemy: [
                asset_server.load("EnemyShot_1.png"),
                asset_server.load("EnemyShot_2.png"),
            ],
        },
    }
}
