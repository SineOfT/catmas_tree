use bevy::prelude::*;


#[derive(Component)]
pub struct EGridChanged<const N: usize>(pub [f32; N]);

impl<const N: usize> Event for EGridChanged<N> {
    type Traversal = &'static Parent;

    const AUTO_PROPAGATE: bool = true;
}


#[derive(Event)]
pub struct EGridUpdated<const N: usize>(pub [f32; N]);