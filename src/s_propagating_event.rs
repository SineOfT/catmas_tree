use bevy::prelude::*;


pub trait PropagatingEvent {
    type AwaitingEvent: Event;
}

// impl<E: PropagatingEvent> Event for E {
//     type Traversal = &'static Parent;
//     const AUTO_PROPAGATE: bool = true;
// }