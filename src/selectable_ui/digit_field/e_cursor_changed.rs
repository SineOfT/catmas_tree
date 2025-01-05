use bevy::prelude::*;


#[derive(Component)]
pub struct ECursorChanged(pub usize);

impl Event for ECursorChanged {
    type Traversal = &'static Parent;

    const AUTO_PROPAGATE: bool = true;
}


#[derive(Event)]
pub struct ECursorUpdated(pub usize);