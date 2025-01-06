use bevy::prelude::*;

use super::e_slider_field_updated::ESliderFieldUpdated;

use super::super::{
    selectable_ui::{
        grid::{EGridUpdated, GridDimension},
        digit_field::{ECursorUpdated, EDigitsUpdated}
    },
    digit_field::CDigitFieldEntities,
    continuous_slider::CContinuousSliderEntities,
    c_node_size::{CNodeSize, NodeSizeOperator, NodeJustify},
    s_node_orientation::NodeOrientation,
    s_field_digit::FieldDigits
};


#[derive(Component)]
pub struct CSliderFieldEntities {
    slider_entity: Entity,
    digit_field_entity: Entity
}

impl CSliderFieldEntities {
    pub const DIMENSION_COUNT: usize = CContinuousSliderEntities::DIMENSION_COUNT;
    pub const DEFAULT_DIMENSIONS: [GridDimension; Self::DIMENSION_COUNT] = CContinuousSliderEntities::DEFAULT_DIMENSIONS;


    pub fn build<const N: usize, const D: usize, T: Sync + Send + Copy + 'static>(
        commands: &mut Commands,
        asset_server: &AssetServer,
        orientation: NodeOrientation,
        length: CNodeSize<NodeJustify>,
        min_digits: FieldDigits<N, D>,
        max_digits: FieldDigits<N, D>,
        empty_color: Color,
        hover_color: Color,
        fill_color: Color,
        tag: T
    ) -> Entity {

        let digit_field_entity = CDigitFieldEntities::build(
            commands,
            asset_server,
            min_digits,
            max_digits,
            tag
        );

        let slider_length = CNodeSize::calc([
            length.operators().clone(),
            vec![
                NodeSizeOperator::Em(orientation.apply_to_size(CDigitFieldEntities::get_size_em::<N, D>()).x),
                NodeSizeOperator::Subtract
            ]
        ].concat());
        let slider_entity = CContinuousSliderEntities::build(
            commands,
            orientation,
            slider_length,
            empty_color,
            hover_color,
            fill_color,
            tag
        );
 
        let entity = commands
            .spawn((
                orientation.apply(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Stretch,
                    align_items: AlignItems::Center,
                    ..default()
                }),
                length,
                Self { slider_entity, digit_field_entity }
            ))
            .add_child(slider_entity)
            .add_child(digit_field_entity)
            .observe(trigger::update_slider::<N, D>)
            .observe(trigger::update_cursor)
            .id();

        entity
    }
}


mod trigger {
    use super::*;


    pub fn update_slider<const N: usize, const D: usize>(
        trigger: Trigger<ESliderFieldUpdated<N, D>>,
        mut commands: Commands,
        continuous_slider_entities_query: Query<&CSliderFieldEntities>
    ) {

        let Ok(entities) = continuous_slider_entities_query.get(trigger.entity())
            else { warn!("updated slider has no entities"); return; };
            
        let ESliderFieldUpdated { t, digits } = trigger.event();

        commands.trigger_targets(EGridUpdated { t: [*t] }, entities.slider_entity);
        commands.trigger_targets(EDigitsUpdated { digits: digits.clone() }, entities.digit_field_entity);
    }

    pub fn update_cursor(
        trigger: Trigger<ECursorUpdated>,
        mut commands: Commands,
        continuous_slider_entities_query: Query<&CSliderFieldEntities>
    ) {

        let Ok(entities) = continuous_slider_entities_query.get(trigger.entity())
            else { warn!("updated crusor has no entities"); return; };

        commands.trigger_targets(trigger.event().clone(), entities.digit_field_entity);
    }


    pub fn update_slider_node(
        trigger: Trigger<EGridUpdated<{CSliderFieldEntities::DIMENSION_COUNT}>>,
        mut node_query: Query<&mut Node>
    ) {

        let Ok(mut node) = node_query.get_mut(trigger.entity())
            else { warn!("updated slider has no node"); return; };
        
        let t = trigger.event().t[0];

        NodeOrientation::set_justify(&mut node, Val::Percent(t * 100.));
    }
}