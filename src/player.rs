use crate::{
    asset_loader::TextureAssets,
    get_single, get_single_mut,
    projectile::{self, Projectile},
    window, AppState,
};
use bevy::{app, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

const LENGTH: u8 = 26;
const HEIGHT: u8 = 16;
const HALF_LENGTH: u8 = LENGTH / 2;
const HALF_HEIGHT: u8 = HEIGHT / 2;

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                (movement, shoot).run_if(in_state(AppState::InGame)),
                check_hit,
                correct_out_of_bounds,
            ),
        );
    }
}

#[derive(Component)]
pub struct Player;

fn setup(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    commands.spawn((
        Name::new("Player"),
        Player,
        SpriteBundle {
            texture: texture_assets.player.clone(),
            transform: Transform::from_xyz(0.0, -300.0, 0.0),
            ..default()
        },
        RigidBody::KinematicVelocityBased,
        Sensor,
        CollisionGroups::new(Group::GROUP_1, Group::GROUP_4),
        ActiveCollisionTypes::KINEMATIC_KINEMATIC,
        ActiveEvents::COLLISION_EVENTS,
        Velocity::zero(),
        Collider::cuboid(f32::from(HALF_LENGTH), f32::from(HALF_HEIGHT)),
    ));
}

fn movement(keys: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>) {
    const VELOCITY: f32 = 400.0;

    let mut move_delta = Vec2::ZERO;
    if keys.any_pressed([KeyCode::A, KeyCode::Left]) {
        move_delta.x -= VELOCITY;
    }
    if keys.any_pressed([KeyCode::D, KeyCode::Right]) {
        move_delta.x += VELOCITY;
    }

    for mut rb_velocity in &mut query {
        rb_velocity.linvel = move_delta;
    }
}

fn correct_out_of_bounds(mut query: Query<&mut Transform, With<Player>>) {
    let mut player = get_single_mut!(query);
    player.translation.x = player.translation.x.clamp(
        -f32::from(window::HALF_WIDTH - u16::from(HALF_LENGTH)),
        f32::from(window::HALF_WIDTH - u16::from(HALF_LENGTH)),
    );
}

fn shoot(
    mut commands: Commands,
    (mut meshes, mut materials, keys): (
        ResMut<Assets<Mesh>>,
        ResMut<Assets<ColorMaterial>>,
        Res<Input<KeyCode>>,
    ),
    query: Query<&Transform, With<Player>>,
) {
    const PROJECTILE_VELOCITY: Vec2 = Vec2::new(0.0, 500.0);

    if !keys.just_pressed(KeyCode::Space) {
        return;
    }

    let player = get_single!(query);
    let mut player_translation = player.translation;
    player_translation.y += 25.0;

    commands.spawn((
        Projectile::new(projectile::Direction::Up),
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(projectile::DIMENSIONS).into())
                .into(),
            material: materials.add(Color::PURPLE.into()),
            transform: Transform::from_translation(player_translation),
            ..default()
        },
        RigidBody::KinematicVelocityBased,
        Sensor,
        ActiveCollisionTypes::KINEMATIC_KINEMATIC | ActiveCollisionTypes::KINEMATIC_STATIC,
        ActiveEvents::COLLISION_EVENTS,
        Collider::cuboid(projectile::HALF_LENGTH, projectile::HALF_HEIGHT),
        CollisionGroups::new(Group::GROUP_3, Group::GROUP_2),
        Velocity::linear(PROJECTILE_VELOCITY),
    ));
}

fn check_hit(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    (p_query, b_query): (Query<Entity, With<Player>>, Query<(Entity, &Projectile)>),
) {
    let player = get_single!(p_query);
    for (projectile_entity, _) in b_query.iter().filter(|(_, p)| p.direction.is_downwards()) {
        let Some(will_collide) = rapier_context.intersection_pair(player, projectile_entity) else {
            continue;
        };
        if will_collide {
            commands.entity(player).despawn();
            commands.entity(projectile_entity).despawn();
        }
    }
}
