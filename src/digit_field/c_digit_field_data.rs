use bevy::prelude::*;

use super::c_digit_field_entities::CDigitFieldEntities;

use super::super::{
    selectable_ui::digit_field::{ECursorChanged, ECursorUpdated, EDigitsChanged, EDigitsUpdated},
    s_field_digit::FieldDigits
};


#[derive(Component)]
pub struct CDigitFieldData<const N: usize, const D: usize> {
    entity: Entity,
    digits: FieldDigits<N, D>
}

impl<const N: usize, const D: usize> CDigitFieldData<N, D> {
    fn new(entity: Entity) -> Self {
        Self { entity, digits: FieldDigits::zero() }
    }

    pub fn build(
        commands: &mut Commands,
        asset_server: &AssetServer,
        min_digits: FieldDigits<N, D>,
        max_digits: FieldDigits<N, D>,
    ) -> Entity {

        let entity = CDigitFieldEntities::build(
            commands,
            asset_server,
            min_digits,
            max_digits,
            ()
        );

        commands.entity(entity)
            .insert(Self::new(entity))
            .observe(trigger::update_digits::<N, D>)
            .observe(trigger::update_cursor::<N, D>);

        entity
    }
}


mod trigger {
    use super::*;


    pub fn update_digits<const N: usize, const D: usize>(
        mut trigger: Trigger<EDigitsChanged<N, D>>,
        mut commands: Commands,
        mut digit_field_data_query: Query<&mut CDigitFieldData<N, D>>
    ) {

        trigger.propagate(false);

        let Ok(mut digit_field_data) = digit_field_data_query.get_mut(trigger.entity())
            else { warn!("changed digits doesn't have data parent"); return; };

        let digits = &trigger.event().digits;

        digit_field_data.digits = digits.clone();

        commands.trigger_targets(EDigitsUpdated { digits: digits.clone() }, digit_field_data.entity);
    }


    pub fn update_cursor<const N: usize, const D: usize>(
        mut trigger: Trigger<ECursorChanged>,
        mut commands: Commands,
        digit_field_data_query: Query<&CDigitFieldData<N, D>>
    ) {

        trigger.propagate(false);

        let Ok(digit_field_data) = digit_field_data_query.get(trigger.entity())
            else { warn!("changed cursor doesn't have data parent"); return; };

        let cursor = trigger.event().cursor;

        commands.trigger_targets(ECursorUpdated { cursor }, digit_field_data.entity);
    }
}