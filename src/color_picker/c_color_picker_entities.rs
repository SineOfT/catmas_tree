use bevy::prelude::*;

use super::{
    c_color_swatch_entities::CColorSwatchEntities,
    l_color_swatch::LColorSwatch,
    consts
};

use super::super::{
    selectable_ui::{
        grid::EGridChanged,
        digit_field::EDigitsChanged
    },
    slider_field::CSliderFieldEntities,
    c_node_size::{CNodeSize, NodeSizeOperator, NodeWidth, NodeHeight, NodeJustify},
    s_node_orientation::NodeOrientation,
};


#[derive(Component)]
pub struct CColorPickerEntities {
    color_display_entity: Entity,
    color_swatch_entity: Entity,
    hue_t_slider_field_entity: Entity,
    alpha_t_slider_field_entity: Entity
}

impl CColorPickerEntities {
    pub fn build<T: Sync + Send + Copy + 'static>(
        commands: &mut Commands,
        asset_server: &AssetServer,
        color_swatch_materials: &mut Assets<LColorSwatch>,
        width: CNodeSize<NodeWidth>,
        height: CNodeSize<NodeHeight>,
        hue_tag: T,
        whiteness_tag: T,
        blackness_tag: T,
        alpha_tag: T,
        swatch_tag: T
    ) -> Entity {

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

        let color_swatch_entity =
            CColorSwatchEntities::build(
                commands,
                asset_server,
                color_swatch_materials,
                height.clone(),
                whiteness_tag,
                blackness_tag,
                swatch_tag
            );

        let hue_t_slider_field_entity =
            CSliderFieldEntities::build(
                commands,
                asset_server,
                NodeOrientation::ThreeForth,
                height.clone_as::<NodeJustify>(),
                consts::DEG_DIGIT_MIN_VALUE,
                consts::DEG_DIGIT_MAX_VALUE,
                Color::srgb(0.8, 0.8, 0.8),
                Color::srgb(0.5, 0.5, 0.5),
                Color::srgb(0.2, 0.2, 0.2),
                hue_tag
            );

        let alpha_t_slider_field_entity =
            CSliderFieldEntities::build(
                commands,
                asset_server,
                NodeOrientation::ThreeForth,
                height.clone_as::<NodeJustify>(),
                consts::T_DIGIT_MIN_VALUE,
                consts::T_DIGIT_MAX_VALUE,
                Color::srgb(0.8, 0.8, 0.8),
                Color::srgb(0.5, 0.5, 0.5),
                Color::srgb(0.2, 0.2, 0.2),
                alpha_tag
            );

        commands
            .spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                width,
                height,
                Self { color_display_entity, color_swatch_entity, hue_t_slider_field_entity, alpha_t_slider_field_entity }
            ))
            .add_child(color_display_entity)
            .add_child(color_swatch_entity)
            .add_child(hue_t_slider_field_entity)
            .add_child(alpha_t_slider_field_entity)
            .id()
    }
}

// mod trigger {
//     use super::*;


//     pub fn propagate_hue_grid_changed(
//         trigger: Trigger<EGridChanged<{CSliderFieldEntities::DIMENSION_COUNT}>>,
//         mut commands: Commands
//     ) {

//         let entity = trigger.entity();
//         let degree = trigger.event().0[0];
//         commands.trigger_targets(EHueGridChanged(degree), entity);
//     }

//     pub fn propagate_hue_digits_changed(
//         trigger: Trigger<EDigitsChanged<{consts::DIGIT_COUNT}, {consts::DEG_DIGIT_DECIMAL_POSITION}>>,
//         mut commands: Commands
//     ) {

//         let entity = trigger.entity();
//         let digits = trigger.event().0.clone();
//         commands.trigger_targets(EHueDigitsChanged(digits), entity);
//     }

//     pub fn propagate_alpha_grid_changed(
//         trigger: Trigger<EGridChanged<{CSliderFieldEntities::DIMENSION_COUNT}>>,
//         mut commands: Commands
//     ) {

//         let entity = trigger.entity();
//         let t = trigger.event().0[0];
//         commands.trigger_targets(EAlphaGridChanged(t), entity);
//     }

//     pub fn propagate_alpha_digits_changed(
//         trigger: Trigger<EDigitsChanged<{consts::DIGIT_COUNT}, {consts::T_DIGIT_DECIMAL_POSITION}>>,
//         mut commands: Commands
//     ) {

//         let entity = trigger.entity();
//         let digits = trigger.event().0.clone();
//         commands.trigger_targets(EAlphaDigitsChanged(digits), entity);
//     }

//     pub fn propagate_value_grid_changed(
//         trigger: Trigger<EGridChanged<{CColorSwatchEntities::DIMENSION_COUNT}>>,
//         mut commands: Commands
//     ) {

//         let entity = trigger.entity();
//         let t = trigger.event().0;
//         commands.trigger_targets(EValueGridChanged(t), entity);
//     }

//     pub fn propagate_whiteness_digits_changed(
//         trigger: Trigger<EDigitsChanged<{consts::DIGIT_COUNT}, {consts::T_DIGIT_DECIMAL_POSITION}>>,
//         mut commands: Commands
//     ) {

//         let entity = trigger.entity();
//         let digits = trigger.event().0.clone();
//         commands.trigger_targets(EWhitenessDigitsChanged(digits), entity);
//     }

//     pub fn propagate_blackness_digits_changed(
//         trigger: Trigger<EDigitsChanged<{consts::DIGIT_COUNT}, {consts::T_DIGIT_DECIMAL_POSITION}>>,
//         mut commands: Commands
//     ) {

//         let entity = trigger.entity();
//         let digits = trigger.event().0.clone();
//         commands.trigger_targets(EBlacknessDigitsChanged(digits), entity);
//     }
// }