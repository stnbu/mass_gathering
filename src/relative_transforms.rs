use bevy::prelude::{Component, Entity, Query, Transform, Without};

#[derive(Component)]
pub struct RelativeTransform {
    pub entity: Entity,
    pub transform: Transform,
}

pub fn update_relative_transforms(
    mut followers: Query<(&mut Transform, &RelativeTransform)>,
    reference_frames: Query<&Transform, Without<RelativeTransform>>,
) {
    for (mut follower, relative_transform) in followers.iter_mut() {
        if let Ok(frame) = reference_frames.get(relative_transform.entity) {
            *follower = frame.mul_transform(relative_transform.transform);
        }
    }
}
