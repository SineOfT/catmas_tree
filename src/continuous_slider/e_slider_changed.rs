use bevy::prelude::*;


#[derive(Component)]
pub struct ESliderChanged;

impl Event for ESliderChanged {
    type Traversal = &'static Parent;

    const AUTO_PROPAGATE: bool = true;
}