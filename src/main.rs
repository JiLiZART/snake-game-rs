mod snake;

use crate::snake::{SNAKE_HEAD_COLOR, SnakeHead};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup_camera(mut commmands: Commands) {
    commmands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_snake(mut commmands: Commands) {
    commmands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(SnakeHead);
}
