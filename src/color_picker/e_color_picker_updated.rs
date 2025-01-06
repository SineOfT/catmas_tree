use bevy::prelude::*;

use super::consts::{TFieldDigits, DegFieldDigits};


#[derive(Event)]
pub struct EHueUpdated {
    pub degree: f32,
    pub t: f32,
    pub digits: DegFieldDigits
}


#[derive(Event)]
pub struct EAlphaUpdated {
    pub t: f32,
    pub digits: TFieldDigits
}


#[derive(Event)]
pub struct EWhitenessUpdated {
    pub t: f32,
    pub digits: TFieldDigits
}


#[derive(Event)]
pub struct EBlacknessUpdated {
    pub t: f32,
    pub digits: TFieldDigits
}