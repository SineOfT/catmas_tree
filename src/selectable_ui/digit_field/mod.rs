use bevy::prelude::*;

pub use c_digit_field::CDigitField;
pub use e_cursor_updated::ECursorUpdated;
pub use e_digits_changed::{EDigitsChanged, EDigitsUpdated};


mod c_key_input_field;
mod e_cursor_updated;
mod e_digits_changed;
mod c_digit_field;


pub struct DigitFieldPlugin;

impl Plugin for DigitFieldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update,
                (
                    c_key_input_field::update::update_selected_key_input_field::<1, 0, ()>,
                    c_key_input_field::update::update_selected_key_input_field::<2, 0, ()>,
                    c_key_input_field::update::update_selected_key_input_field::<3, 0, ()>,
                    c_key_input_field::update::update_selected_key_input_field::<4, 0, ()>,
                    c_key_input_field::update::update_selected_key_input_field::<5, 0, ()>,
                    c_key_input_field::update::update_selected_key_input_field::<6, 0, ()>,
                    c_key_input_field::update::update_selected_key_input_field::<1, 0, super::super::color_picker::ColorPickerTag>,
                    c_key_input_field::update::update_selected_key_input_field::<2, 0, super::super::color_picker::ColorPickerTag>,
                    c_key_input_field::update::update_selected_key_input_field::<3, 0, super::super::color_picker::ColorPickerTag>,
                    c_key_input_field::update::update_selected_key_input_field::<4, 0, super::super::color_picker::ColorPickerTag>,
                    c_key_input_field::update::update_selected_key_input_field::<5, 0, super::super::color_picker::ColorPickerTag>,
                    c_key_input_field::update::update_selected_key_input_field::<6, 0, super::super::color_picker::ColorPickerTag>,
                    c_key_input_field::update::update_selected_key_input_field::<1, {usize::MAX}, super::super::color_picker::ColorPickerTag>,
                    c_key_input_field::update::update_selected_key_input_field::<2, {usize::MAX}, super::super::color_picker::ColorPickerTag>,
                    c_key_input_field::update::update_selected_key_input_field::<3, {usize::MAX}, super::super::color_picker::ColorPickerTag>,
                    c_key_input_field::update::update_selected_key_input_field::<4, {usize::MAX}, super::super::color_picker::ColorPickerTag>,
                    c_key_input_field::update::update_selected_key_input_field::<5, {usize::MAX}, super::super::color_picker::ColorPickerTag>,
                    c_key_input_field::update::update_selected_key_input_field::<6, {usize::MAX}, super::super::color_picker::ColorPickerTag>
                )
            );
    }
}