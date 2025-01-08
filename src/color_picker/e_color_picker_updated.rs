use bevy::prelude::*;

use super::consts::{TFieldDigits, DegFieldDigits};


#[derive(Event, Clone)]
pub struct EColorPickerHueUpdated {
    pub color: Color,
    pub degree: f32,
    pub t: f32,
    pub digits: DegFieldDigits
}


#[derive(Event, Clone)]
pub struct EColorPickerAlphaUpdated {
    pub color: Color,
    pub t: f32,
    pub digits: TFieldDigits
}


#[derive(Event, Clone)]
pub struct EColorPickerWhitenessUpdated {
    pub color: Color,
    pub t: f32,
    pub blackness_t: f32,
    pub digits: TFieldDigits
}


#[derive(Event, Clone)]
pub struct EColorPickerBlacknessUpdated {
    pub color: Color,
    pub t: f32,
    pub whiteness_t: f32,
    pub digits: TFieldDigits
}


#[derive(Event, Clone)]
pub struct EColorPickerValueUpdated {
    pub color: Color,
    pub whiteness_t: f32,
    pub blackness_t: f32,
    pub whiteness_digits: TFieldDigits,
    pub blackness_digits: TFieldDigits
}