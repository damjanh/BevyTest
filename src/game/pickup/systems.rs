use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use super::components::*;
use super::resources::*;
use super::NUM_OF_PICKUPS;

pub fn spawn_pickups(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUM_OF_PICKUPS {
        let rand_x = random::<f32>() * window.width();
        let rand_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(rand_x, rand_y, 0.0)
                    .with_scale(Vec3::new(2.0, 2.0, 1.0)),
                texture: asset_server.load("sprites/energy_can.png"),
                ..default()
            },
            Pickup {},
        ));
    }
}

pub fn despawn_pickups(mut commands: Commands, pickup_query: Query<Entity, With<Pickup>>) {
    for pickup_entity in pickup_query.iter() {
        commands.entity(pickup_entity).despawn();
    }
}

pub fn tick_pickup_spawn_timer(mut pickup_spawn_timer: ResMut<PickupSpawnTimer>, time: Res<Time>) {
    pickup_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_pickups_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    pickup_spawn_timer: Res<PickupSpawnTimer>,
) {
    if pickup_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0)
                    .with_scale(Vec3::new(2.0, 2.0, 1.0)),
                texture: asset_server.load("sprites/energy_can.png"),
                ..default()
            },
            Pickup {},
        ));
    }
}
