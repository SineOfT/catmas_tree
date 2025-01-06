use bevy::prelude::*;

use super::super::s_field_digit::FieldDigits;


#[derive(Event)]
pub struct ESliderFieldUpdated<const N: usize, const D: usize> {
    pub t: f32,
    pub digits: FieldDigits<N, D>
}