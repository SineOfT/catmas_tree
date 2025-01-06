use bevy::prelude::*;


pub use c_digit_field_entities::CDigitFieldEntities;


mod c_digit_field_entities;
mod c_digit_field_data;


pub struct DigitFieldPlugin;

impl Plugin for DigitFieldPlugin {
    fn build(&self, app: &mut App) {
    }
}