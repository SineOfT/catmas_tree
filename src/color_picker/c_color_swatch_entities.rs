use bevy::prelude::*;

use super::{
    l_color_swatch::LColorSwatch,
    consts
};

use super::super::{
    c_node_size::{CNodeSize, NodeSizeOperator, NodeWidth, NodeHeight},
    digit_field::CDigitFieldEntities,
    selectable_ui::{
        SelectableUIBundle,
        grid::{CGrid, CArrowInputGrid, GridDimension},
    },
    s_field_digit::FieldDigits
};


#[derive(Component)]
pub struct CColorSwatchEntities {
    color_swatch_entity: Entity,
    whiteness_t_field_entity: Entity,
    blackness_t_field_entity: Entity
}

impl CColorSwatchEntities {
    pub const DIMENSION_COUNT: usize = 2;
    pub const DIMENSIONS: [GridDimension; Self::DIMENSION_COUNT] = [GridDimension::X, GridDimension::Y];

    pub fn build<T: Sync + Send + Copy + 'static>(
        commands: &mut Commands,
        asset_server: &AssetServer,
        color_swatch_materials: &mut Assets<LColorSwatch>,
        height: CNodeSize<NodeHeight>,
        whiteness_tag: T,
        blackness_tag: T,
        swatch_tag: T
    ) -> Entity {

        let whiteness_t_field_entity =
            CDigitFieldEntities::build(
                commands,
                asset_server,
                consts::T_DIGIT_MIN_VALUE,
                consts::T_DIGIT_MAX_VALUE,
                whiteness_tag
            );
        let blackness_t_field_entity =
            CDigitFieldEntities::build(
                commands,
                asset_server,
                consts::T_DIGIT_MIN_VALUE,
                consts::T_DIGIT_MAX_VALUE,
                blackness_tag
            );

        let value_field_entities = commands
            .spawn((
                Node {
                    width: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
            ))
            .add_child(whiteness_t_field_entity)
            .add_child(blackness_t_field_entity)
            .id();

        let color_swatch_entity = commands
            .spawn((
                Node::default(),
                CNodeSize::<NodeWidth>::calc([
                    height.operators().clone(),
                    vec![NodeSizeOperator::Em(1.), NodeSizeOperator::Subtract]
                ].concat()),
                CNodeSize::<NodeHeight>::calc([
                    height.operators().clone(),
                    vec![NodeSizeOperator::Em(1.), NodeSizeOperator::Subtract]
                ].concat()),                
                MaterialNode(color_swatch_materials.add(LColorSwatch::new())),
                SelectableUIBundle::new(Val::Px(5.)),
                CGrid::new(Self::DIMENSIONS, swatch_tag),
                CArrowInputGrid::new(1.)
            ))
            .id();

        commands
            .spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_content: AlignContent::Center,
                    ..default()
                },
                height.clone(),
                Self { color_swatch_entity, whiteness_t_field_entity, blackness_t_field_entity }
            ))
            .add_child(value_field_entities)
            .add_child(color_swatch_entity)
            .id()
    }
}