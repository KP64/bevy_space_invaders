use bevy::{app, prelude::*, sprite::MaterialMesh2dBundle};

use crate::{
    asset_loader::TextureAssets,
    projectile::{self, Direction, Projectile},
    window,
};

const LENGTH: u8 = 26;
const HALF_LENGTH: u8 = LENGTH / 2;

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup)
            .add_systems(Update, (movement, shooting, check_hit));
    }
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands, scene_assets: Res<TextureAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: scene_assets.player.clone(),
            transform: Transform::from_xyz(0.0, -300.0, 0.0),
            ..default()
        },
        Player,
        Name::new("Player"),
    ));
}

fn movement(
    (keys, time): (Res<Input<KeyCode>>, Res<Time>),
    mut query: Query<&mut Transform, With<Player>>,
) {
    const VELOCITY: f32 = 400.0;
    let mut player_velocity = Vec3::splat(0.0);
    if keys.any_pressed([KeyCode::A, KeyCode::Left]) {
        player_velocity.x -= VELOCITY;
    }
    if keys.any_pressed([KeyCode::D, KeyCode::Right]) {
        player_velocity.x += VELOCITY;
    }

    if player_velocity == Vec3::ZERO {
        return;
    }

    let Ok(mut player) = query.get_single_mut() else {
        error!("Couldn't get Player to Move. Maybe Already Dead?");
        return;
    };

    player.translation += player_velocity * time.delta_seconds();
    player.translation.x = player.translation.x.clamp(
        -f32::from(window::HALF_WIDTH - u16::from(HALF_LENGTH)),
        f32::from(window::HALF_WIDTH - u16::from(HALF_LENGTH)),
    );
}

fn shooting(
    mut commands: Commands,
    (mut meshes, mut materials, keys): (
        ResMut<Assets<Mesh>>,
        ResMut<Assets<ColorMaterial>>,
        Res<Input<KeyCode>>,
    ),
    query: Query<&Transform, With<Player>>,
) {
    const PROJECTILE_SPEED: f32 = 500.0;
    if !keys.just_pressed(KeyCode::Space) {
        return;
    }

    let Ok(player) = query.get_single() else {
        error!("Couldn't get Player to Shoot. Maybe Already Dead?");
        return;
    };
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
        Projectile::new(Direction::Up, PROJECTILE_SPEED),
    ));
}

/* TODO: Unify this function with the enemy one for less duplication? */
fn check_hit(
    mut commands: Commands,
    (enemy_query, projectile_query): (
        Query<(Entity, &Transform), With<Player>>,
        Query<(Entity, &Transform, &Projectile)>,
    ),
) {
    const LENGTH: u8 = 26;
    const HALF_LENGTH: f32 = (LENGTH / 2) as f32;
    const HEIGHT: u8 = 16;
    const HALF_HEIGHT: f32 = (HEIGHT / 2) as f32;

    for (player_entity, player_translation) in enemy_query
        .iter()
        .map(|(ent, trans)| (ent, trans.translation.xy()))
    {
        for (proj_entity, proj_translation, _) in projectile_query
            .iter()
            .map(|(ent, trans, proj)| (ent, trans.translation.xy(), proj))
            .filter(|(_, _, proj)| proj.direction != Direction::Up)
        {
            let player_range = (
                (player_translation.x - HALF_LENGTH)..=(player_translation.x + HALF_LENGTH),
                player_translation.y - HALF_HEIGHT..=(player_translation.y + HALF_HEIGHT),
            );

            let negative_pos = proj_translation - projectile::HALF_DIMENSIONS;
            let positive_pos = proj_translation + projectile::HALF_DIMENSIONS;

            if (player_range.0.contains(&negative_pos.x)
                || player_range.0.contains(&positive_pos.x))
                && (player_range.1.contains(&negative_pos.y)
                    || player_range.1.contains(&positive_pos.y))
            {
                commands.entity(player_entity).despawn();
                commands.entity(proj_entity).despawn();
            }
        }
    }
}
