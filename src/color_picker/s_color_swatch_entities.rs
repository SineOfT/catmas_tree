use bevy::prelude::*;

use super::l_color_swatch::LColorSwatch;

use super::super::{
    c_node_size::{CNodeSize, NodeSizeOperator, NodeWidth, NodeHeight},
    digit_field::s_digit_field_entities::DigitFieldEntities,
    selectable_ui::{
        SelectableUIBundle,
        c_selectable_ui::CSelectableUI,
        grid::{
            GridDimension,
            SelectableGrid,
            CArrowInputGrid        
        },
    },
    s_field_digit::FieldDigit
};


pub struct ColorSwatchEntities {
    color_swatch_entity: Entity,
    whiteness_field_entities: DigitFieldEntities,
    blackness_field_entities: DigitFieldEntities
}

impl ColorSwatchEntities {
    const DIGIT_COUNT: usize = 3;
    const DIGIT_DECIMAL_POSITION: Option<usize> = Some(0);
    const DIGIT_MIN_VALUE: [FieldDigit; Self::DIGIT_COUNT] = [FieldDigit::Zero, FieldDigit::Zero, FieldDigit::Zero];
    const DIGIT_MAX_VALUE: [FieldDigit; Self::DIGIT_COUNT] = [FieldDigit::One, FieldDigit::Zero, FieldDigit::Zero];


    pub fn color_swatch_entity(&self) -> Entity { self.color_swatch_entity }
    pub fn whiteness_field_entities(&self) -> &DigitFieldEntities { &self.whiteness_field_entities }
    pub fn blackness_field_entities(&self) -> &DigitFieldEntities { &self.blackness_field_entities }

    pub fn build(
        commands: &mut Commands,
        asset_server: &AssetServer,
        color_swatch_materials: &mut Assets<LColorSwatch>,
        height: CNodeSize<NodeHeight>
    ) -> (Entity, Self) {

        let (whiteness_field_entity, whiteness_field_entities) =
            DigitFieldEntities::build::<{Self::DIGIT_COUNT}>(
                commands,
                asset_server,
                Self::DIGIT_MIN_VALUE,
                Self::DIGIT_MAX_VALUE,
                Self::DIGIT_DECIMAL_POSITION
            );
        let (blackness_field_entity, blackness_field_entities) =
            DigitFieldEntities::build::<{Self::DIGIT_COUNT}>(
                commands,
                asset_server,
                Self::DIGIT_MIN_VALUE,
                Self::DIGIT_MAX_VALUE,
                Self::DIGIT_DECIMAL_POSITION
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
            .add_child(whiteness_field_entity)
            .add_child(blackness_field_entity)
            .id();

        let color_swatch_entity = commands
            .spawn((
                Node {
                    // border: UiRect::all(Val::Px(5.)),
                    // margin: UiRect::all(Val::Px(50.)),
                    ..default()
                },
                CNodeSize::<NodeWidth>::calc([
                    height.operators().clone(),
                    vec![NodeSizeOperator::Em(1.), NodeSizeOperator::Subtract]
                ].concat()),
                CNodeSize::<NodeHeight>::calc([
                    height.operators().clone(),
                    vec![NodeSizeOperator::Em(1.), NodeSizeOperator::Subtract]
                ].concat()),                
                MaterialNode(color_swatch_materials.add(LColorSwatch::new())),
                SelectableUIBundle::new(
                    CSelectableUI::<SelectableGrid<2>>::new([GridDimension::X, GridDimension::Y]),
                    Val::Px(5.),
                ),
                CArrowInputGrid::new(1.)
            ))
            .id();

        let entity = commands
            .spawn((
                Node {
                    // border: UiRect::all(Val::Px(5.)),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_content: AlignContent::Center,
                    ..default()
                },
                height.clone(),
                // BorderColor(Color::srgb(0., 0., 1.)),
            ))
            .add_child(value_field_entities)
            .add_child(color_swatch_entity)
            .id();

        (entity, Self { color_swatch_entity, whiteness_field_entities, blackness_field_entities })
    }
}