mod camera;
mod spaceship;
mod movement;
mod debug;

use bevy::prelude::*;
use camera::CameraPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;


fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
    .insert_resource(AmbientLight {
        color: Color::default(),
        brightness: 0.75,
    })
    .add_plugins(CameraPlugin)
    .add_plugins(DefaultPlugins)
    .add_plugins(SpaceshipPlugin)
    .add_plugins(MovementPlugin)
    .add_plugins(DebugPlugin)
    .run();
}


