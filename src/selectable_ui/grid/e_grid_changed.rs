use bevy::prelude::*;


#[derive(Component)]
pub struct EGridChanged<const N: usize, T: Sync + Send + Copy + 'static = ()> {
    pub t: [f32; N],
    pub tag: T
}

impl<const N: usize, T: Sync + Send + Copy + 'static> Event for EGridChanged<N, T> {
    type Traversal = &'static Parent;

    const AUTO_PROPAGATE: bool = true;
}


#[derive(Event)]
pub struct EGridUpdated<const N: usize> {
    pub t: [f32; N]
}