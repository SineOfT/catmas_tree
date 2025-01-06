use bevy::prelude::*;

use super::{
    c_slider_field_entities::CSliderFieldEntities,
    e_slider_field_updated::ESliderFieldUpdated
};

use super::super::{
    c_node_size::{CNodeSize, NodeJustify},
    s_node_orientation::NodeOrientation,
    selectable_ui::{
        grid::EGridChanged,
        digit_field::{ECursorChanged, ECursorUpdated, EDigitsChanged}
    },
    s_field_digit::FieldDigits
};


#[derive(Component)]
pub struct CSliderFieldData {
    range: Vec2,
    value: f32
}

impl CSliderFieldData {
    fn new(range: Vec2) -> Self {
        Self { range, value: 0. }
    }

    pub fn build<const N: usize, const D: usize>(
        commands: &mut Commands,
        asset_server: &AssetServer,
        orientation: NodeOrientation,
        justify_size: CNodeSize<NodeJustify>,
        min_digits: FieldDigits<N, D>,
        max_digits: FieldDigits<N, D>,
        empty_color: Color,
        hover_color: Color,
        fill_color: Color
    ) -> Entity {

        let range = Vec2::new(
            min_digits.to_value(),
            max_digits.to_value()
        );

        let entity = CSliderFieldEntities::build(
            commands,
            asset_server,
            orientation,
            justify_size,
            min_digits,
            max_digits,
            empty_color,
            hover_color,
            fill_color,
            ()
        );

        commands.entity(entity)
            .insert(Self::new(range))
            .observe(trigger::update_slider::<N, D>)
            .observe(trigger::update_digits::<N, D>)
            .observe(trigger::update_cursor);

        entity
    }
}


mod trigger {
    use super::*;


    fn t_to_value(range: Vec2, t: f32) -> f32 {
        f32::lerp(range.x, range.y, t)
    }

    fn value_to_t(range: Vec2, value: f32) -> f32 {
        f32::inverse_lerp(range.x, range.y, value)
    }


    pub fn update_slider<const N: usize, const D: usize>(
        mut trigger: Trigger<EGridChanged<{CSliderFieldEntities::DIMENSION_COUNT}>>,
        mut commands: Commands,
        mut continuous_slider_data_query: Query<&mut CSliderFieldData>
    ) {

        trigger.propagate(false);

        let entity = trigger.entity();
        let Ok(mut data) = continuous_slider_data_query.get_mut(entity)
            else { warn!("changed slider doesn't have data parent"); return; };
        
        let t = trigger.event().t[0];
        let value = t_to_value(data.range, t);
        let digits = FieldDigits::<N, D>::from_value(value);

        data.value = value;

        commands.trigger_targets(ESliderFieldUpdated { t, digits }, entity);
    }


    pub fn update_digits<const N: usize, const D: usize>(
        mut trigger: Trigger<EDigitsChanged<N, D>>,
        mut commands: Commands,
        mut continuous_slider_data_query: Query<&mut CSliderFieldData>
    ) {

        trigger.propagate(false);

        let entity = trigger.entity();
        let Ok(mut continuous_slider_data) = continuous_slider_data_query.get_mut(entity)
            else { warn!("changed digits doesn't have data parent"); return; };
        
        let digits = trigger.event().digits.clone();
        let value = digits.to_value();
        let t = value_to_t(continuous_slider_data.range, value);

        continuous_slider_data.value = value;

        commands.trigger_targets(ESliderFieldUpdated { t, digits }, entity);
    }


    pub fn update_cursor(
        mut trigger: Trigger<ECursorChanged>,
        mut commands: Commands
    ) {

        trigger.propagate(false);

        let entity = trigger.entity();
        let cursor = trigger.event().cursor;
        commands.trigger_targets(ECursorUpdated { cursor }, entity);
    }
}