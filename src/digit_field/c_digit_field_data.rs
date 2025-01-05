use bevy::prelude::*;

use super::s_digit_field_entities::DigitFieldEntities;

use super::super::{
    selectable_ui::digit_field::{ECursorChanged, ECursorUpdated, EDigitsChanged, EDigitsUpdated},
    s_field_digit::FieldDigit
};


#[derive(Component)]
pub struct CDigitFieldData<const N: usize> {
    entities: DigitFieldEntities,
    digits: [FieldDigit; N]
}

impl<const N: usize> CDigitFieldData<N> {
    pub fn new(entities: DigitFieldEntities) -> Self {
        Self { entities, digits: [FieldDigit::Zero; N] }
    }

    pub fn build(
        commands: &mut Commands,
        asset_server: &AssetServer,
        min_digits: [FieldDigit; N],
        max_digits: [FieldDigit; N],
        decimal_position: Option<usize>
    ) -> Entity {

        let (entity, entities) = DigitFieldEntities::build(
            commands,
            asset_server,
            min_digits,
            max_digits,
            decimal_position
        );

        commands.entity(entity)
            .insert(Self::new(entities))
            .observe(trigger::update_digits::<N>)
            .observe(trigger::update_cursor::<N>);

        entity
    }
}


mod trigger {
    use super::*;


    pub fn update_digits<const N: usize>(
        mut trigger: Trigger<EDigitsChanged<N>>,
        mut commands: Commands,
        mut digit_field_data_query: Query<&mut CDigitFieldData<N>>
    ) {

        trigger.propagate(false);

        let Ok(mut digit_field_data) = digit_field_data_query.get_mut(trigger.entity())
            else { warn!("changed digits doesn't have data parent"); return; };

        let digits = trigger.event().0;

        digit_field_data.digits = digits;

        commands.trigger_targets(EDigitsUpdated(digits), digit_field_data.entities.field_entity());
    }


    pub fn update_cursor<const N: usize>(
        mut trigger: Trigger<ECursorChanged>,
        mut commands: Commands,
        digit_field_data_query: Query<&CDigitFieldData<N>>
    ) {

        trigger.propagate(false);

        let Ok(digit_field_data) = digit_field_data_query.get(trigger.entity())
            else { warn!("changed cursor doesn't have data parent"); return; };

        let cursor = trigger.event().0;

        commands.trigger_targets(ECursorUpdated(cursor), digit_field_data.entities.cursor_entity());
    }
}