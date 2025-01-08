use bevy::prelude::*;


#[derive(Event, Clone)]
pub struct ECursorUpdated{
    pub cursor: usize
}