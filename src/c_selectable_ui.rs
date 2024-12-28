use bevy::prelude::*;


#[derive(Component)]
pub struct CSelectableUI {
    is_selected: bool
}

impl CSelectableUI {
    pub fn new() -> Self {
        Self { is_selected: false }
    }
}