use super::{
    enemy::{self, PointsWorth},
    player,
};
use crate::{game, window};
use bevy::{
    app,
    prelude::*,
    render::color,
    sprite::{Material2d, MaterialMesh2dBundle},
};
use bevy_rapier2d::prelude::*;
use std::ops::RangeInclusive;

pub struct Plugin;

impl app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Spawn>()
            .add_event::<Collision>()
            .add_systems(OnEnter(game::State::Paused), freeze)
            .add_systems(OnExit(game::State::Paused), unfreeze)
            .add_systems(
                Update,
                (despawn_out_of_window, spawn, check_collisions, on_hit)
                    .run_if(in_state(game::State::Playing)),
            )
            .add_systems(OnEnter(game::State::GameOver), cleanup);
    }
}

#[derive(Component)]
pub(super) struct Projectile {
    pub(super) initial_velocity: Velocity,
}

impl Projectile {
    pub(super) const fn new(initial_velocity: Velocity) -> Self {
        Self { initial_velocity }
    }
}

#[derive(Bundle)]
struct Bundle<M>
where
    M: Material2d,
{
    projectile: Projectile,
    material_mesh: MaterialMesh2dBundle<M>,
    rigidbody: RigidBody,
    sensor: Sensor,
    active_collision_types: ActiveCollisionTypes,
    active_events: ActiveEvents,
    collider: Collider,
    collision_groups: CollisionGroups,
    velocity: Velocity,
}

impl<M> Bundle<M>
where
    M: Material2d,
{
    fn new(
        velocity: Velocity,
        collision_groups: CollisionGroups,
        material_mesh: MaterialMesh2dBundle<M>,
        dimensions: Vec2,
    ) -> Self {
        Self {
            material_mesh,
            collision_groups,
            velocity,
            projectile: Projectile::new(velocity),
            rigidbody: RigidBody::KinematicVelocityBased,
            sensor: Sensor,
            active_collision_types: ActiveCollisionTypes::KINEMATIC_KINEMATIC
                | ActiveCollisionTypes::KINEMATIC_STATIC,
            active_events: ActiveEvents::COLLISION_EVENTS,
            collider: Collider::cuboid(dimensions.x / 2.0, dimensions.y / 2.0),
        }
    }
}

#[derive(Component, Clone, Copy, Deref, DerefMut)]
pub(super) struct Color(pub(super) color::Color);

#[derive(Event)]
struct Collision;

#[derive(Event)]
pub(super) struct Spawn {
    pub(super) velocity: Velocity,
    pub(super) transform: Transform,
    pub(super) collision_target_groups: CollisionGroups,
    pub(super) dimensions: Vec2,
    pub(super) color: Color,
}

fn spawn(
    mut commands: Commands,
    (mut event, mut meshes, mut materials): (
        EventReader<Spawn>,
        ResMut<Assets<Mesh>>,
        ResMut<Assets<ColorMaterial>>,
    ),
) {
    for &Spawn {
        velocity,
        transform,
        collision_target_groups,
        dimensions,
        color,
    } in event.read()
    {
        commands.spawn((
            Name::new("Projectile"),
            Bundle::new(
                velocity,
                collision_target_groups,
                MaterialMesh2dBundle {
                    mesh: meshes.add(Rectangle::from_size(dimensions)).into(),
                    material: materials.add(color.0),
                    transform,
                    ..default()
                },
                dimensions,
            ),
        ));
    }
}

fn freeze(mut velocities: Query<&mut Velocity, With<Projectile>>) {
    for mut rb_velocity in &mut velocities {
        *rb_velocity = Velocity::zero();
    }
}

fn unfreeze(mut query: Query<(&mut Velocity, &Projectile)>) {
    for (mut rb_velocity, projectile) in &mut query {
        *rb_velocity = projectile.initial_velocity;
    }
}

fn check_collisions(
    mut commands: Commands,
    (rapier_context, mut player_death, mut enemy_death, mut projectile_collision_event): (
        Res<RapierContext>,
        EventWriter<player::Death>,
        EventWriter<enemy::Death>,
        EventWriter<Collision>,
    ),
    mut query: Query<(Entity, Option<&PointsWorth>, &CollisionGroups)>,
) {
    for [(entity_1, point1, coll_group_1), (entity_2, point2, coll_group_2)] in
        query.iter_combinations_mut()
    {
        if !rapier_context
            .intersection_pair(entity_1, entity_2)
            .is_some_and(|will_collide| will_collide)
        {
            continue;
        }

        match (coll_group_1.memberships, coll_group_2.memberships) {
            (Group::GROUP_1, _) | (_, Group::GROUP_1) => {
                player_death.send(player::Death);
            }
            (Group::GROUP_2, _) | (_, Group::GROUP_2) => {
                let score = match (point1, point2) {
                    (None, Some(p)) | (Some(p), None) => *p,
                    (None, None) => PointsWorth::default(),
                    (Some(_), Some(_)) => unreachable!("Two Enemies Collided with each other"),
                };
                enemy_death.send(enemy::Death(score));
            }
            (Group::GROUP_3 | Group::GROUP_4, Group::GROUP_3 | Group::GROUP_4) => {
                projectile_collision_event.send(Collision);
            }
            e => unreachable!("Collision of Projectile with Unknown Entity Collision: {e:#?}"),
        };

        if let Some(mut entity_1) = commands.get_entity(entity_1) {
            entity_1.despawn();
        }
        if let Some(mut entity_2) = commands.get_entity(entity_2) {
            entity_2.despawn();
        }
    }
}

fn on_hit(mut collisions: EventReader<Collision>) {
    for _ in collisions.read() {
        info!("Projectile Projectile Hit");
    }
}

fn despawn_out_of_window(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Projectile>>,
) {
    /// Arbitrarily chosen
    const OFFSET: f32 = 20.0;

    const GROUND_CEILING: f32 = window::DIMENSIONS.y / 2.0 + OFFSET;
    const WINDOW_RANGE: RangeInclusive<f32> = -GROUND_CEILING..=GROUND_CEILING;

    for (projectile, _) in query
        .iter()
        .map(|(projectile, transform)| (projectile, transform.translation.y))
        .filter(|(_, y_pos)| !WINDOW_RANGE.contains(y_pos))
    {
        if let Some(mut projectile) = commands.get_entity(projectile) {
            projectile.despawn();
        }
    }
}

fn cleanup(mut commands: Commands, projectiles: Query<Entity, With<Projectile>>) {
    for projectile in &projectiles {
        if let Some(mut projectile) = commands.get_entity(projectile) {
            projectile.despawn();
        }
    }
}
