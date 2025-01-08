use bevy::prelude::*;

use super::super::{
    selectable_ui::grid::{CGrid, EGridUpdated, GridDimension},
    s_node_orientation::NodeOrientation,
    c_node_size::{CNodeSize, NodeSizeOperator, NodeJustify, NodeAlign},
};


#[derive(Component)]
pub struct CContinuousSliderEntities {
    hover_slider_entity: Entity,
    fill_slider_entity: Entity,
}

impl CContinuousSliderEntities {
    pub const DIMENSION_COUNT: usize = 1;
    pub const DEFAULT_DIMENSIONS: [GridDimension; Self::DIMENSION_COUNT] = [GridDimension::X];


    pub fn build<T: Sync + Send + Copy + 'static>(
        commands: &mut Commands,
        orientation: NodeOrientation,
        length: CNodeSize<NodeAlign>,
        empty_color: Color,
        hover_color: Color,
        fill_color: Color,
        tag: T
    ) -> Entity {

        let hover_slider_entity = commands
            .spawn((
                orientation.apply(Node {
                    position_type: PositionType::Absolute,
                    width: Val::ZERO,
                    height: Val::Percent(100.),
                    ..default()
                }),
                BackgroundColor(hover_color),
                ZIndex(0)
            ))
            .id();

        let fill_slider_entity = commands
            .spawn((
                orientation.apply(Node {
                    position_type: PositionType::Absolute,
                    width: Val::ZERO,
                    height: Val::Percent(100.),
                    ..default()
                }),
                BackgroundColor(fill_color),
                ZIndex(1)
            ))
            .id();

        let entity = CGrid::build(
            commands,
            orientation.apply_to_grid_dimensions(Self::DEFAULT_DIMENSIONS),
            1.,
            Val::Px(5.),
            tag
        );

        commands.entity(entity)
            .insert((
                orientation.apply(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexStart,
                    ..default()
                }),
                CNodeSize::<NodeJustify>::new(NodeSizeOperator::Em(0.5)),
                length,
                BackgroundColor(empty_color),
                Self { hover_slider_entity, fill_slider_entity }
            ))
            .add_child(hover_slider_entity)
            .add_child(fill_slider_entity)
            .observe(trigger::update_slider);

        entity
    }
}


mod trigger {
    use super::*;


    pub fn update_slider(
        trigger: Trigger<EGridUpdated<{CContinuousSliderEntities::DIMENSION_COUNT}>>,
        continuous_slider_entities_query: Query<&CContinuousSliderEntities>,
        mut node_query: Query<&mut Node>
    ) {

        let Ok(entities) = continuous_slider_entities_query.get(trigger.entity())
            else { warn!("updated slider has no entities"); return; };
        let Ok(mut node) = node_query.get_mut(entities.fill_slider_entity)
            else { warn!("updated slider has no fill slider node"); return; };

        let t = trigger.event().t[0];
        NodeOrientation::set_justify(&mut node, Val::Percent(t * 100.));
    }
}