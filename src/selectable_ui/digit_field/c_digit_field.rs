use bevy::prelude::*;

use super::{
    c_key_input_field::CKeyInputField,
    e_cursor_updated::ECursorUpdated,
    e_digits_changed::EDigitsUpdated
};

use super::super::{
    SelectableUIBundle,
    e_ui_selected::EUISelected
};

use super::super::super::s_field_digit::FieldDigits;


#[derive(Component)]
pub struct CDigitField<const N: usize, const D: usize, T: Sync + Send + Copy + 'static> {
    min_digits: FieldDigits<N, D>,
    max_digits: FieldDigits<N, D>,
    digits: FieldDigits<N, D>,
    tag: T
}

impl<const N: usize, const D: usize, T: Sync + Send + Copy + 'static> CDigitField<N, D, T> {
    pub fn min_digits(&self) -> &FieldDigits<N, D> { &self.min_digits }
    pub fn max_digits(&self) -> &FieldDigits<N, D> { &self.max_digits }
    pub fn digits(&self) -> &FieldDigits<N, D> { &self.digits }
    pub fn tag(&self) -> T { self.tag }


    pub fn build(
        commands: &mut Commands,
        min_digits: FieldDigits<N, D>,
        max_digits: FieldDigits<N, D>,
        outline_width: Val,
        tag: T
    ) -> Entity {

        commands
            .spawn((
                SelectableUIBundle::new(outline_width),
                Self { min_digits, max_digits, digits: FieldDigits::zero(), tag },
                CFieldCursor::new(),
                CKeyInputField
            ))
            .observe(trigger::update_digits::<N, D, T>)
            .observe(trigger::update_cursor)
            .observe(trigger::reset_cursor_on_selected)
            .id()
    }
}


#[derive(Component)]
pub struct CFieldCursor {
    cursor: usize
}

impl CFieldCursor {
    pub fn new() -> Self {
        Self { cursor: 0 }
    }

    pub fn cursor(&self) -> usize { self.cursor }
}


mod trigger {
    use super::*;


    pub fn update_digits<const N: usize, const D: usize, T: Sync + Send + Copy + 'static>(
        trigger: Trigger<EDigitsUpdated<N, D>>,
        mut digit_field_query: Query<&mut CDigitField<N, D, T>>
    ) {

        let Ok(mut digit_field) = digit_field_query.get_mut(trigger.entity())
            else { warn!("updated digits has no field"); return; };

        let digits = trigger.event().digits;
        digit_field.digits = digits;
    }

    pub fn update_cursor(
        trigger: Trigger<ECursorUpdated>,
        mut field_cursor_query: Query<&mut CFieldCursor>
    ) {

        let Ok(mut field_cursor) = field_cursor_query.get_mut(trigger.entity())
            else { warn!("updated cursor has no field"); return; };

        let cursor = trigger.event().cursor;
        field_cursor.cursor = cursor;
    }

    pub fn reset_cursor_on_selected(
        trigger: Trigger<EUISelected>,
        mut commands: Commands
    ) {

        commands.trigger_targets(ECursorUpdated { cursor: 0 }, trigger.entity());
    }
}