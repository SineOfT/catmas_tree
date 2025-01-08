use bevy::prelude::*;

use super::{
    c_color_swatch_entities::CColorSwatchEntities,
    l_color_swatch::LColorSwatch,
    e_color_picker_updated::{
        EColorPickerHueUpdated,
        EColorPickerAlphaUpdated,
        EColorPickerWhitenessUpdated,
        EColorPickerBlacknessUpdated,
        EColorPickerValueUpdated
    },
    e_color_swatch_updated::{
        EColorSwatchHueUpdated,
        EColorSwatchAlphaUpdated,
        EColorSwatchWhitenessUpdated,
        EColorSwatchBlacknessUpdated,
        EColorSwatchValueUpdated
    },
    consts
};

use super::super::{
    slider_field::{CSliderFieldEntities, ESliderFieldUpdated},
    c_node_size::{CNodeSize, NodeSizeOperator, NodeWidth, NodeHeight, NodeJustify},
    s_node_orientation::NodeOrientation,
};


#[derive(Component)]
pub struct CColorPickerEntities {
    color_display_entity: Entity,
    color_swatch_entity: Entity,
    hue_deg_slider_field_entity: Entity,
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

        let hue_deg_slider_field_entity =
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
                Self { color_display_entity, color_swatch_entity, hue_deg_slider_field_entity, alpha_t_slider_field_entity }
            ))
            .add_child(color_display_entity)
            .add_child(color_swatch_entity)
            .add_child(hue_deg_slider_field_entity)
            .add_child(alpha_t_slider_field_entity)
            .observe(trigger::update_hue)
            .observe(trigger::update_alpha)
            .observe(trigger::update_whiteness)
            .observe(trigger::update_blackness)
            .observe(trigger::update_value)
            .id()
    }
}

mod trigger {
    use super::*;


    pub fn update_hue(
        trigger: Trigger<EColorPickerHueUpdated>,
        mut commands: Commands,
        color_picker_entities_query: Query<&CColorPickerEntities>,
        mut bg_color_query: Query<&mut BackgroundColor>
    ) {

        let Ok(entities) = color_picker_entities_query.get(trigger.entity())
            else { warn!("updated color picker has no entities"); return; };
        let Ok(mut color_display_bg_color) = bg_color_query.get_mut(entities.color_display_entity)
            else { warn!("updated color picker has no color display"); return; };

        let EColorPickerHueUpdated { color, degree, t, digits } = trigger.event().clone();
        color_display_bg_color.0 = color;

        commands.trigger_targets(ESliderFieldUpdated { t, digits }, entities.hue_deg_slider_field_entity);
        commands.trigger_targets(EColorSwatchHueUpdated { degree }, entities.color_swatch_entity);
    }


    pub fn update_alpha(
        trigger: Trigger<EColorPickerAlphaUpdated>,
        mut commands: Commands,
        color_picker_entities_query: Query<&CColorPickerEntities>,
        mut bg_color_query: Query<&mut BackgroundColor>
    ) {

        let Ok(entities) = color_picker_entities_query.get(trigger.entity())
            else { warn!("updated color picker has no entities"); return; };
        let Ok(mut color_display_bg_color) = bg_color_query.get_mut(entities.color_display_entity)
            else { warn!("updated color picker has no color display"); return; };

        let EColorPickerAlphaUpdated { color, t, digits } = trigger.event().clone();
        color_display_bg_color.0 = color;

        commands.trigger_targets(ESliderFieldUpdated { t, digits }, entities.alpha_t_slider_field_entity);
        commands.trigger_targets(EColorSwatchAlphaUpdated { t }, entities.color_swatch_entity);
    }


    pub fn update_whiteness(
        trigger: Trigger<EColorPickerWhitenessUpdated>,
        mut commands: Commands,
        color_picker_entities_query: Query<&CColorPickerEntities>,
        mut bg_color_query: Query<&mut BackgroundColor>
    ) {

        let Ok(entities) = color_picker_entities_query.get(trigger.entity())
            else { warn!("updated color picker has no entities"); return; };
        let Ok(mut color_display_bg_color) = bg_color_query.get_mut(entities.color_display_entity)
            else { warn!("updated color picker has no color display"); return; };

        let EColorPickerWhitenessUpdated { color, t, blackness_t, digits } = trigger.event().clone();
        color_display_bg_color.0 = color;

        commands.trigger_targets(EColorSwatchWhitenessUpdated { t, blackness_t, digits }, entities.color_swatch_entity);
    }


    pub fn update_blackness(
        trigger: Trigger<EColorPickerBlacknessUpdated>,
        mut commands: Commands,
        color_picker_entities_query: Query<&CColorPickerEntities>,
        mut bg_color_query: Query<&mut BackgroundColor>
    ) {

        let Ok(entities) = color_picker_entities_query.get(trigger.entity())
            else { warn!("updated color picker has no entities"); return; };
        let Ok(mut color_display_bg_color) = bg_color_query.get_mut(entities.color_display_entity)
            else { warn!("updated color picker has no color display"); return; };

        let EColorPickerBlacknessUpdated { color, t, whiteness_t, digits } = trigger.event().clone();
        color_display_bg_color.0 = color;

        commands.trigger_targets(EColorSwatchBlacknessUpdated { t, whiteness_t, digits }, entities.color_swatch_entity);
    }


    pub fn update_value(
        trigger: Trigger<EColorPickerValueUpdated>,
        mut commands: Commands,
        color_picker_entities_query: Query<&CColorPickerEntities>,
        mut bg_color_query: Query<&mut BackgroundColor>
    ) {

        let Ok(entities) = color_picker_entities_query.get(trigger.entity())
            else { warn!("updated color picker has no entities"); return; };
        let Ok(mut color_display_bg_color) = bg_color_query.get_mut(entities.color_display_entity)
            else { warn!("updated color picker has no color display"); return; };

        let EColorPickerValueUpdated {
            color,
            whiteness_t,
            blackness_t,
            whiteness_digits,
            blackness_digits
        } = trigger.event().clone();
        color_display_bg_color.0 = color;

        commands.trigger_targets(EColorSwatchValueUpdated {
            whiteness_t,
            blackness_t,
            whiteness_digits,
            blackness_digits
        }, entities.color_swatch_entity);
    }
}