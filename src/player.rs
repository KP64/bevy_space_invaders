use bevy::{app, prelude::*, sprite::MaterialMesh2dBundle};

use crate::{
    projectile::{self, Projectile},
    window,
};

const LENGTH: u8 = 26;
const HALF_LENGTH: u8 = LENGTH / 2;

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, (player_movement, player_shooting));
    }
}

#[derive(Component)]
struct Player;

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("player.png"),
            transform: Transform::from_xyz(0.0, -300.0, 0.0),
            ..default()
        },
        Player,
        Name::new("Player"),
    ));
}

fn player_movement(keys: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Player>>) {
    let mut player_velocity = Vec3::splat(0.0);
    if keys.any_pressed([KeyCode::A, KeyCode::Left]) {
        player_velocity.x -= 1.0;
    }
    if keys.any_pressed([KeyCode::D, KeyCode::Right]) {
        player_velocity.x += 1.0;
    }

    for mut transform in &mut query {
        transform.translation += player_velocity;
        transform.translation.x = transform.translation.x.clamp(
            -f32::from(window::HALF_WIDTH - u16::from(HALF_LENGTH)),
            f32::from(window::HALF_WIDTH - u16::from(HALF_LENGTH)),
        );
    }
}

fn player_shooting(
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    query: Query<&Transform, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !keys.just_pressed(KeyCode::Space) {
        return;
    }

    let player = query.get_single().expect("Couldn't get Player");
    let mut player_translation = player.translation;
    player_translation.y += 25.0;
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(projectile::DIMENSIONS).into())
                .into(),
            material: materials.add(Color::PURPLE.into()),
            transform: Transform::from_translation(player_translation),
            ..default()
        },
        Projectile,
    ));
}
