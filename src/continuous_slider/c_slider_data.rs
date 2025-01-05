use bevy::prelude::*;

use super::s_continuous_slider_entities::ContinuousSliderEntities;

use super::super::{
    c_node_size::{CNodeSize, NodeJustify},
    node_orientation::NodeOrientation,
    s_field_digit::FieldDigit
};


#[derive(Component)]
pub struct CSliderData {
    entities: ContinuousSliderEntities,
    t: f32
}

impl CSliderData {
    pub fn new(entities: ContinuousSliderEntities) -> Self {
        Self { entities, t: 0. }
    }

    pub fn build<const N: usize>(
        commands: &mut Commands,
        asset_server: &AssetServer,
        orientation: NodeOrientation,
        justify_size: CNodeSize<NodeJustify>,
        min_digits: [FieldDigit; N],
        max_digits: [FieldDigit; N],
        decimal_position: Option<usize>,
        empty_color: Color,
        hover_color: Color,
        fill_color: Color
    ) -> Entity {

        let (entity, entities) = ContinuousSliderEntities::build(
            commands,
            asset_server,
            orientation,
            justify_size,
            min_digits,
            max_digits,
            decimal_position,
            empty_color,
            hover_color,
            fill_color
        );

        commands.entity(entity).insert(Self::new(entities));

        entity
    }
}