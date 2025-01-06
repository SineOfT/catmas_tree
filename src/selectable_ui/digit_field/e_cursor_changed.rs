use bevy::prelude::*;


#[derive(Component)]
pub struct ECursorChanged<T: Sync + Send + Copy + 'static = ()> {
    pub cursor: usize,
    pub tag: T
}

impl<T: Sync + Send + Copy + 'static> Event for ECursorChanged<T> {
    type Traversal = &'static Parent;

    const AUTO_PROPAGATE: bool = true;
}


#[derive(Event, Clone)]
pub struct ECursorUpdated{
    pub cursor: usize
}