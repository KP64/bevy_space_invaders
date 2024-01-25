use std::ops::RangeInclusive;

use bevy::{app, prelude::*};
use bevy_rapier2d::prelude::*;

mod explosion;
pub mod invader;
mod ufo;

use crate::{asset_loader::TextureAssets, projectile::Projectile, score::Score};

use self::explosion::Explosion;

const COLUMNS: RangeInclusive<u8> = 1..=10;
const ROWS: RangeInclusive<u8> = 1..=6;

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((invader::Plugin, ufo::Plugin, explosion::Plugin))
            .add_systems(Update, check_hit);
    }
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Points(u16);

fn check_hit(
    mut commands: Commands,
    (mut score, texture_assets, rapier_context): (
        ResMut<Score>,
        Res<TextureAssets>,
        Res<RapierContext>,
    ),
    (enemy_query, projectile_query): (
        Query<(Entity, &Transform, &Points), With<Enemy>>,
        Query<(Entity, &Projectile)>,
    ),
) {
    for (e_entity, e_transform, points_worth) in &enemy_query {
        for (p_entity, _) in projectile_query
            .iter()
            .filter(|(_, p)| p.direction.is_upwards())
        {
            let Some(will_collide) = rapier_context.intersection_pair(e_entity, p_entity) else {
                continue;
            };
            if !will_collide {
                continue;
            }

            score.0 += points_worth.0 as usize;

            commands.entity(e_entity).despawn();
            commands.entity(p_entity).despawn();
            commands.spawn((
                SpriteBundle {
                    texture: texture_assets.explosions.enemy.clone(),
                    transform: Transform::from_translation(e_transform.translation),
                    ..default()
                },
                Explosion::default(),
            ));
        }
    }
}
