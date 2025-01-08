use bevy::prelude::*;

use super::c_digit_field_entities::CDigitFieldEntities;

use super::super::{
    selectable_ui::digit_field::{EDigitsChanged, EDigitsUpdated},
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
            .observe(trigger::update_digits::<N, D>);

        entity
    }
}


mod trigger {
    use super::*;


    pub fn update_digits<const N: usize, const D: usize>(
        mut trigger: Trigger<EDigitsChanged<N, D, ()>>,
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
}