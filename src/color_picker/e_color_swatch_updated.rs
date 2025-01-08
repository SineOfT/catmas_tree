use bevy::prelude::*;

use super::consts::TFieldDigits;


#[derive(Event, Clone)]
pub struct EColorSwatchHueUpdated {
    pub degree: f32
}


#[derive(Event, Clone)]
pub struct EColorSwatchAlphaUpdated {
    pub t: f32
}


#[derive(Event, Clone)]
pub struct EColorSwatchWhitenessUpdated {
    pub t: f32,
    pub blackness_t: f32,
    pub digits: TFieldDigits
}


#[derive(Event, Clone)]
pub struct EColorSwatchBlacknessUpdated {
    pub t: f32,
    pub whiteness_t: f32,
    pub digits: TFieldDigits
}

#[derive(Event, Clone)]
pub struct EColorSwatchValueUpdated {
    pub whiteness_t: f32,
    pub blackness_t: f32,
    pub whiteness_digits: TFieldDigits,
    pub blackness_digits: TFieldDigits
}