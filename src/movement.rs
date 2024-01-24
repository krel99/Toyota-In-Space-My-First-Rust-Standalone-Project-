use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

#[derive(Component, Debug)]
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position);
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        //time to make sure it is the same speed on all computers
        transform.translation.x += velocity.value.x * time.delta_seconds();
        transform.translation.y += velocity.value.y * time.delta_seconds();
        transform.translation.z += velocity.value.z * time.delta_seconds();
        // can use translation and value without xyz instead for more efficient
        // transform.translation += velocity.value
    }
}