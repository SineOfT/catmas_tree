use bevy::prelude::*;

use super::{
    l_color_swatch::LColorSwatch,
    s_color_swatch_entities::ColorSwatchEntities
};

use super::super::{
    s_field_digit::FieldDigit,
    c_node_size::{CNodeSize, NodeSizeOperator, NodeWidth, NodeHeight, NodeJustify},
    node_orientation::NodeOrientation,
    continuous_slider::s_continuous_slider_entities::ContinuousSliderEntities
};


pub struct ColorPickerEntities {
    color_display_entity: Entity,
    color_swatch_entities: ColorSwatchEntities,
    hue_slider_entities: ContinuousSliderEntities,
    alpha_slider_entities: ContinuousSliderEntities
}

impl ColorPickerEntities {
    const DIGIT_COUNT: usize = 3;
    const DIGIT_DECIMAL_POSITION: Option<usize> = Some(0);
    const DIGIT_MIN_VALUE: [FieldDigit; Self::DIGIT_COUNT] = [FieldDigit::Zero, FieldDigit::Zero, FieldDigit::Zero];
    const DIGIT_MAX_VALUE: [FieldDigit; Self::DIGIT_COUNT] = [FieldDigit::One, FieldDigit::Zero, FieldDigit::Zero];
    

    pub fn build(
        commands: &mut Commands,
        asset_server: &AssetServer,
        color_swatch_materials: &mut Assets<LColorSwatch>,
        width: CNodeSize<NodeWidth>,
        height: CNodeSize<NodeHeight>
    ) -> (Entity, Self) {

        let color_display_entity = commands
            .spawn((
                Node {
                    ..default()
                },
                CNodeSize::<NodeWidth>::new(NodeSizeOperator::Em(1.)),
                height.clone(),
                BackgroundColor(Color::srgb(0., 0., 1.))
            ))
            .id();

        let (color_swatch_entity, color_swatch_entities) =
            ColorSwatchEntities::build(
                commands,
                asset_server,
                color_swatch_materials,
                height.clone()
            );

        let (hue_slider_entity, hue_slider_entities) =
            ContinuousSliderEntities::build::<{Self::DIGIT_COUNT}>(
                commands,
                asset_server,
                NodeOrientation::ThreeForth,
                height.clone_as::<NodeJustify>(),
                Self::DIGIT_MIN_VALUE,
                Self::DIGIT_MAX_VALUE,
                Self::DIGIT_DECIMAL_POSITION,
                Color::srgb(0.8, 0.8, 0.8),
                Color::srgb(0.5, 0.5, 0.5),
                Color::srgb(0.2, 0.2, 0.2)
            );

        let (alpha_slider_entity, alpha_slider_entities) =
            ContinuousSliderEntities::build::<{Self::DIGIT_COUNT}>(
                commands,
                asset_server,
                NodeOrientation::ThreeForth,
                height.clone_as::<NodeJustify>(),
                Self::DIGIT_MIN_VALUE,
                Self::DIGIT_MAX_VALUE,
                Self::DIGIT_DECIMAL_POSITION,
                Color::srgb(0.8, 0.8, 0.8),
                Color::srgb(0.5, 0.5, 0.5),
                Color::srgb(0.2, 0.2, 0.2)
            );

        let entity = commands
            .spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                width,
                height
            ))
            .add_child(color_display_entity)
            .add_child(color_swatch_entity)
            .add_child(hue_slider_entity)
            .add_child(alpha_slider_entity)
            .id();

        (entity, Self { color_display_entity, color_swatch_entities, hue_slider_entities, alpha_slider_entities })
    }
}