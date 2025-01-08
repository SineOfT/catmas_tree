use bevy::prelude::*;

use super::{
    c_color_picker_entities::CColorPickerEntities,
    c_color_swatch_entities::CColorSwatchEntities,
    e_color_picker_updated::{
        EColorPickerHueUpdated,
        EColorPickerWhitenessUpdated,
        EColorPickerBlacknessUpdated,
        EColorPickerAlphaUpdated,
        EColorPickerValueUpdated
    },
    l_color_swatch::LColorSwatch,
    consts::{self, TFieldDigits, DegFieldDigits}
};

use super::super::{
    selectable_ui::{
        digit_field::EDigitsChanged,
        grid::EGridChanged
    },
    slider_field::CSliderFieldEntities,
    c_node_size::{CNodeSize, NodeWidth, NodeHeight}
};


#[derive(Clone, Copy)]
pub enum ColorPickerTag {
    Hue,
    Whiteness,
    Blackness,
    Alpha,
    Swatch
}


#[derive(Component)]
pub struct CColorPickerData {
    color: Vec4
}

impl CColorPickerData {
    fn new() -> Self {
        Self { color: Vec4::ZERO }
    }

    pub fn build(
        commands: &mut Commands,
        asset_server: &AssetServer,
        color_swatch_materials: &mut Assets<LColorSwatch>,
        width: CNodeSize<NodeWidth>,
        height: CNodeSize<NodeHeight>
    ) -> Entity {

        let entity = CColorPickerEntities::build(
            commands,
            asset_server,
            color_swatch_materials,
            width,
            height,
            ColorPickerTag::Hue,
            ColorPickerTag::Whiteness,
            ColorPickerTag::Blackness,
            ColorPickerTag::Alpha,
            ColorPickerTag::Swatch
        );

        commands.entity(entity)
            .insert(Self::new())
            .observe(trigger::update_slider)
            .observe(trigger::update_swatch)
            .observe(trigger::update_t_digits)
            .observe(trigger::update_deg_digits);

        entity
    }
}


mod trigger {
    use super::*;

    pub fn update_slider(
        mut trigger: Trigger<EGridChanged<{CSliderFieldEntities::DIMENSION_COUNT}, ColorPickerTag>>,
        mut commands: Commands,
        mut color_picker_data_query: Query<&mut CColorPickerData>
    ) {

        trigger.propagate(false);

        let entity = trigger.entity();
        let Ok(mut data) = color_picker_data_query.get_mut(entity)
            else { warn!("changed slider doesn't have data parent"); return; };

        let event = trigger.event();
        let t = event.t[0];
        let tag = event.tag;

        match tag {
            ColorPickerTag::Hue => {
                let degree = consts::get_degree_from_t(t);
                let digits = DegFieldDigits::from_value(degree);
                data.color.x = degree;
                let color = Color::Hwba(Hwba::from_vec4(data.color));
                info!(degree);
                commands.trigger_targets(EColorPickerHueUpdated { color, degree, t, digits}, entity);
            },
            ColorPickerTag::Alpha => {
                let digits = TFieldDigits::from_value(t);
                data.color.w = t;
                let color = Color::Hwba(Hwba::from_vec4(data.color));
                commands.trigger_targets(EColorPickerAlphaUpdated { color, t, digits }, entity);
            },
            _ => { warn!("uncovered match branch"); }
        }
    }
    
    
    pub fn update_swatch(
        mut trigger: Trigger<EGridChanged<{CColorSwatchEntities::DIMENSION_COUNT}, ColorPickerTag>>,
        mut commands: Commands,
        mut color_picker_data_query: Query<&mut CColorPickerData>
    ) {

        trigger.propagate(false);

        let entity = trigger.entity();
        let Ok(mut data) = color_picker_data_query.get_mut(entity)
            else { warn!("changed slider doesn't have data parent"); return; };

        let event = trigger.event();
        let blackness_t = event.t[0];
        let whiteness_t = event.t[1];
        let tag = event.tag;

        match tag {
            ColorPickerTag::Swatch => {
                let whiteness_digits = TFieldDigits::from_value(whiteness_t);
                let blackness_digits = TFieldDigits::from_value(blackness_t);
                data.color.y = whiteness_t;
                data.color.z = blackness_t;
                let color = Color::Hwba(Hwba::from_vec4(data.color));
                commands.trigger_targets(
                    EColorPickerValueUpdated {
                        color,
                        whiteness_t,
                        blackness_t,
                        whiteness_digits,
                        blackness_digits
                    },
                    entity
                );
            },
            _ => { warn!("uncovered match branch"); }
        }
    }


    pub fn update_deg_digits(
        mut trigger: Trigger<EDigitsChanged<{consts::DIGIT_COUNT}, {consts::DEG_DIGIT_DECIMAL_POSITION}, ColorPickerTag>>,
        mut commands: Commands,
        mut color_picker_data_query: Query<&mut CColorPickerData>
    ) {

        info!("update deg digits");

        trigger.propagate(false);

        let entity = trigger.entity();
        let Ok(mut data) = color_picker_data_query.get_mut(entity)
            else { warn!("changed hue digits doesn't have data parent"); return; };

        let event = trigger.event();
        let digits = event.digits.clone();
        let tag = event.tag;
        match tag {
            ColorPickerTag::Hue => {
                let degree = digits.to_value();
                let t = consts::get_t_from_degree(degree);
                data.color.x = degree;
                let color = Color::Hwba(Hwba::from_vec4(data.color));
                commands.trigger_targets(EColorPickerHueUpdated { color, degree, t, digits }, entity);
            },
            _ => { warn!("uncovered match branch"); }
        }
    }


    pub fn update_t_digits(
        mut trigger: Trigger<EDigitsChanged<{consts::DIGIT_COUNT}, {consts::T_DIGIT_DECIMAL_POSITION}, ColorPickerTag>>,
        mut commands: Commands,
        mut color_picker_data_query: Query<&mut CColorPickerData>
    ) {

        info!("update t digits");

        trigger.propagate(false);

        let entity = trigger.entity();
        let Ok(mut data) = color_picker_data_query.get_mut(entity)
            else { warn!("changed hue digits doesn't have data parent"); return; };

        let event = trigger.event();
        let digits = event.digits.clone();
        let tag = event.tag;
        match tag {
            ColorPickerTag::Whiteness => {
                let t = digits.to_value();
                let blackness_t = data.color.z;
                data.color.y = t;
                let color = Color::Hwba(Hwba::from_vec4(data.color));
                commands.trigger_targets(EColorPickerWhitenessUpdated { color, t, blackness_t, digits }, entity);
            },
            ColorPickerTag::Blackness => {
                let t = digits.to_value();
                let whiteness_t = data.color.y;
                data.color.z = t;
                let color = Color::Hwba(Hwba::from_vec4(data.color));
                commands.trigger_targets(EColorPickerBlacknessUpdated { color, t, whiteness_t, digits }, entity);
            },
            ColorPickerTag::Alpha => {
                let t = digits.to_value();
                data.color.w = t;
                let color = Color::Hwba(Hwba::from_vec4(data.color));
                commands.trigger_targets(EColorPickerAlphaUpdated { color, t, digits }, entity);
            },
            _ => { warn!("uncovered match branch"); }
        }
    }
}