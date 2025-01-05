use bevy::prelude::*;

use super::implement_selectable_ui;

pub use c_key_input_field::CKeyInputField;
pub use e_cursor_changed::{ECursorChanged, ECursorUpdated};
pub use e_digits_changed::{EDigitsChanged, EDigitsUpdated};
pub use s_selectable_digit_field::{SelectableDigitField, DigitFieldParam, DigitFieldData};


mod c_key_input_field;
mod e_cursor_changed;
mod e_digits_changed;
mod s_selectable_digit_field;


pub struct DigitFieldPlugin;

impl Plugin for DigitFieldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update,
                (
                    c_key_input_field::update::update_selected_key_input_field::<1>,
                    c_key_input_field::update::update_selected_key_input_field::<2>,
                    c_key_input_field::update::update_selected_key_input_field::<3>,
                    c_key_input_field::update::update_selected_key_input_field::<4>,
                    c_key_input_field::update::update_selected_key_input_field::<5>,
                    c_key_input_field::update::update_selected_key_input_field::<6>
                )
            );

        implement_selectable_ui::<SelectableDigitField<1>>(app);
        implement_selectable_ui::<SelectableDigitField<2>>(app);
        implement_selectable_ui::<SelectableDigitField<3>>(app);
        implement_selectable_ui::<SelectableDigitField<4>>(app);
        implement_selectable_ui::<SelectableDigitField<5>>(app);
        implement_selectable_ui::<SelectableDigitField<6>>(app);
    }
}