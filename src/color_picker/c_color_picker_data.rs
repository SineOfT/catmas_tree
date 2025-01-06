use bevy::prelude::*;

use super::{
    c_color_picker_entities::CColorPickerEntities,
    c_color_swatch_entities::CColorSwatchEntities,
    e_color_picker_updated::{EHueUpdated, EWhitenessUpdated, EBlacknessUpdated, EAlphaUpdated},
    l_color_swatch::LColorSwatch,
    consts::{self, TFieldDigits, DegFieldDigits}
};

use super::super::{
    selectable_ui::{
        digit_field::{ECursorChanged, EDigitsChanged},
        grid::{EGridChanged}
    },
    slider_field::CSliderFieldEntities,
    c_node_size::{CNodeSize, NodeWidth, NodeHeight},
    s_field_digit::FieldDigits
};


#[derive(Clone, Copy)]
enum ColorPickerTag {
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
            .insert(Self::new());

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
                commands.trigger_targets(EHueUpdated { degree, t, digits}, entity);
            },
            ColorPickerTag::Alpha => {
                let digits = TFieldDigits::from_value(t);
                data.color.w = t;
                commands.trigger_targets(EAlphaUpdated { t, digits }, entity);
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
        let whiteness_t = event.t[0];
        let blackness_t = event.t[1];
        let tag = event.tag;

        match tag {
            ColorPickerTag::Swatch => {
                let whiteness_digits = TFieldDigits::from_value(whiteness_t);
                let blackness_digits = TFieldDigits::from_value(blackness_t);
                data.color.y = whiteness_t;
                data.color.z = blackness_t;
                commands.trigger_targets(EWhitenessUpdated { t: whiteness_t, digits: whiteness_digits }, entity);
                commands.trigger_targets(EBlacknessUpdated { t: blackness_t, digits: blackness_digits }, entity);
            },
            _ => { warn!("uncovered match branch"); }
        }
    }


    pub fn update_deg_digits(
        mut trigger: Trigger<EDigitsChanged<{consts::DIGIT_COUNT}, {consts::DEG_DIGIT_DECIMAL_POSITION}, ColorPickerTag>>,
        mut commands: Commands,
        mut color_picker_data_query: Query<&mut CColorPickerData>
    ) {

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
                commands.trigger_targets(EHueUpdated { degree, t, digits }, entity);
            },
            _ => { warn!("uncovered match branch"); }
        }
    }


    pub fn update_t_digits(
        mut trigger: Trigger<EDigitsChanged<{consts::DIGIT_COUNT}, {consts::T_DIGIT_DECIMAL_POSITION}, ColorPickerTag>>,
        mut commands: Commands,
        mut color_picker_data_query: Query<&mut CColorPickerData>
    ) {

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
                data.color.y = t;
                commands.trigger_targets(EWhitenessUpdated { t, digits }, entity);
            },
            ColorPickerTag::Blackness => {
                let t = digits.to_value();
                data.color.z = t;
                commands.trigger_targets(EBlacknessUpdated { t, digits }, entity);
            },
            ColorPickerTag::Alpha => {
                let t = digits.to_value();
                data.color.w = t;
                commands.trigger_targets(EAlphaUpdated { t, digits }, entity);
            },
            _ => { warn!("uncovered match branch"); }
        }
    }
}