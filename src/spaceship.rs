use bevy::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    collision_detection::Collider,
    movement::{Acceleration, MovingObjectBundle, Velocity},
    schedule::InGameSet,
};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const MOVEMENT_SPEED: f32 = 25.0;
const ROTATION_SPEED: f32 = 2.5;
const ROLL_SPEED: f32 = 2.5;
const COLLIDER_RADIUS: f32 = 2.5;

const MISSILE_SPEED: f32 = 50.0;
const MISSILE_FORWARD_SPAWN_SCALAR: f32 = 7.5;
const MISSILE_COLLIDER_RADIUS: f32 = 1.0;

#[derive(Component, Debug)]
pub struct Spaceship;

#[derive(Component, Debug)]
pub struct SpaceshipMissile;

#[derive(Component, Debug)]
pub struct SpaceshipShield;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship).add_systems(
            Update,
            (
                spaceship_movement_controls,
                spaceship_weapon_controls,
                spaceship_shield_controls,
            )
                .chain()
                .in_set(InGameSet::UserInput),
        );
    }
}

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity { value: Vec3::ZERO },
            acceleration: Acceleration::new(Vec3::ZERO),
            collider: Collider::new(COLLIDER_RADIUS),
            model: SceneBundle {
                scene: scene_assets.spaceship.clone(),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
        },
        Spaceship,
    ));
}

fn spaceship_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok((mut transform, mut velocity)) = query.get_single_mut() else {
        return;
    };

    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    if keyboard_input.pressed(KeyCode::KeyD) {
        rotation = -ROTATION_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::KeyA) {
        rotation = ROTATION_SPEED * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        movement = -MOVEMENT_SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyW) {
        movement = MOVEMENT_SPEED
    }

    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        roll = -ROLL_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::ControlLeft) {
        roll = ROLL_SPEED * time.delta_seconds();
    }

    transform.rotate_y(rotation);

    transform.rotate_local_z(roll);

    velocity.value = -transform.forward() * movement;
}

fn spaceship_weapon_controls(
    mut commands: Commands,
    query: Query<&Transform, With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    scene_assets: Res<SceneAssets>,
) {
    let Ok(transform) = query.get_single() else {
        return;
    };

    if keyboard_input.pressed(KeyCode::Space) {
        commands.spawn((
            MovingObjectBundle {
                velocity: Velocity::new(-transform.forward() * MISSILE_SPEED),
                acceleration: Acceleration::new(Vec3::ZERO),
                collider: Collider::new(MISSILE_COLLIDER_RADIUS),
                model: SceneBundle {
                    scene: scene_assets.missiles.clone(),
                    transform: Transform::from_translation(
                        transform.translation + -transform.forward() * MISSILE_FORWARD_SPAWN_SCALAR,
                    ),
                    ..default()
                },
            },
            SpaceshipMissile,
        ));
    }
}

fn spaceship_shield_controls(
    mut commands: Commands,
    query: Query<Entity, With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(spaceship) = query.get_single() else {
        return;
    };
    if keyboard_input.pressed(KeyCode::Tab) {
        commands.entity(spaceship).insert(SpaceshipShield);
    }
}
