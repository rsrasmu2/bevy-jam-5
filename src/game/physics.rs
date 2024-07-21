use avian2d::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default());
    app.insert_resource(Gravity(Vec2::ZERO));
}

#[derive(PhysicsLayer)]
pub enum CollisionLayer {
    Player,
    Building,
    Beam,
    BeamBlocker,
}
