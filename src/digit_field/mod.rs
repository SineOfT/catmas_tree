use bevy::prelude::*;


pub mod s_digit_field_entities;
pub mod c_digit_field_data;
pub mod e_digit_changed;


pub struct DigitFieldPlugin;

impl Plugin for DigitFieldPlugin {
    fn build(&self, app: &mut App) {
    }
}