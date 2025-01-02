use bevy::prelude::*;

use super::c_selectable_ui::SelectableUIMarker;


#[derive(Event)]
pub struct ESelectedChanged<M: SelectableUIMarker> { pub entity: Entity, pub data: M::Data }