use bevy::prelude::*;

use super::{
    e_color_swatch_updated::{
        EColorSwatchHueUpdated,
        EColorSwatchAlphaUpdated,
        EColorSwatchWhitenessUpdated,
        EColorSwatchBlacknessUpdated,
        EColorSwatchValueUpdated
    },
    l_color_swatch::LColorSwatch,
    consts
};

use super::super::{
    c_node_size::{CNodeSize, NodeSizeOperator, NodeWidth, NodeHeight},
    digit_field::CDigitFieldEntities,
    selectable_ui::{
        digit_field::EDigitsUpdated,
        grid::{CGrid, GridDimension, EGridUpdated}
    }
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

        let color_swatch_entity = CGrid::build(
            commands,
            Self::DIMENSIONS,
            1.,
            Val::Px(5.),
            swatch_tag
        );
        
        commands.entity(color_swatch_entity)
            .insert((
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
            ));

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
        trigger: Trigger<EColorSwatchHueUpdated>,
        mut l_color_swatches: ResMut<Assets<LColorSwatch>>,
        color_swatch_entities_query: Query<&CColorSwatchEntities>,
        l_color_swatch_handle_query: Query<&MaterialNode<LColorSwatch>>
    ) {

        let Ok(entities) = color_swatch_entities_query.get(trigger.entity())
            else { warn!("updated color swatch has no entities"); return; };
        let Ok(l_color_swatch_handle) = l_color_swatch_handle_query.get(entities.color_swatch_entity)
            else { warn!("udpated color swatch has no material handle"); return; };
        let Some(l_color_swatch) = l_color_swatches.get_mut(l_color_swatch_handle.id())
            else { warn!("updated color swatch handle does not have a material"); return; };

        let degree = trigger.event().degree;
        l_color_swatch.set_hue(degree);
    }


    pub fn update_alpha(
        trigger: Trigger<EColorSwatchAlphaUpdated>,
        mut l_color_swatches: ResMut<Assets<LColorSwatch>>,
        color_swatch_entities_query: Query<&CColorSwatchEntities>,
        l_color_swatch_handle_query: Query<&MaterialNode<LColorSwatch>>
    ) {

        let Ok(entities) = color_swatch_entities_query.get(trigger.entity())
            else { warn!("updated color swatch has no entities"); return; };
        let Ok(l_color_swatch_handle) = l_color_swatch_handle_query.get(entities.color_swatch_entity)
            else { warn!("udpated color swatch has no material handle"); return; };
        let Some(l_color_swatch) = l_color_swatches.get_mut(l_color_swatch_handle.id())
            else { warn!("updated color swatch handle does not have a material"); return; };

        let t = trigger.event().t;
        l_color_swatch.set_alpha(t);
    }


    pub fn update_whiteness(
        trigger: Trigger<EColorSwatchWhitenessUpdated>,
        mut commands: Commands,
        mut l_color_swatches: ResMut<Assets<LColorSwatch>>,
        color_swatch_entities_query: Query<&CColorSwatchEntities>,
        l_color_swatch_handle_query: Query<&MaterialNode<LColorSwatch>>
    ) {

        let Ok(entities) = color_swatch_entities_query.get(trigger.entity())
            else { warn!("updated color swatch has no entities"); return; };
        let Ok(l_color_swatch_handle) = l_color_swatch_handle_query.get(entities.color_swatch_entity)
            else { warn!("udpated color swatch has no material handle"); return; };
        let Some(l_color_swatch) = l_color_swatches.get_mut(l_color_swatch_handle.id())
            else { warn!("updated color swatch handle does not have a material"); return; };

        let EColorSwatchWhitenessUpdated { t, blackness_t, digits } = trigger.event().clone();
        l_color_swatch.set_whiteness(t);

        commands.trigger_targets(EGridUpdated { t: [blackness_t, t] }, entities.color_swatch_entity);
        commands.trigger_targets(EDigitsUpdated { digits }, entities.whiteness_t_field_entity);
    }


    pub fn update_blackness(
        trigger: Trigger<EColorSwatchBlacknessUpdated>,
        mut commands: Commands,
        mut l_color_swatches: ResMut<Assets<LColorSwatch>>,
        color_swatch_entities_query: Query<&CColorSwatchEntities>,
        l_color_swatch_handle_query: Query<&MaterialNode<LColorSwatch>>
    ) {

        let Ok(entities) = color_swatch_entities_query.get(trigger.entity())
            else { warn!("updated color swatch has no entities"); return; };
        let Ok(l_color_swatch_handle) = l_color_swatch_handle_query.get(entities.color_swatch_entity)
            else { warn!("udpated color swatch has no material handle"); return; };
        let Some(l_color_swatch) = l_color_swatches.get_mut(l_color_swatch_handle.id())
            else { warn!("updated color swatch handle does not have a material"); return; };

        let EColorSwatchBlacknessUpdated { t, whiteness_t, digits } = trigger.event().clone();
        l_color_swatch.set_blackness(t);

        commands.trigger_targets(EGridUpdated { t: [t, whiteness_t] }, entities.color_swatch_entity);
        commands.trigger_targets(EDigitsUpdated { digits }, entities.blackness_t_field_entity);
    }


    pub fn update_value(
        trigger: Trigger<EColorSwatchValueUpdated>,
        mut commands: Commands,
        mut l_color_swatches: ResMut<Assets<LColorSwatch>>,
        color_swatch_entities_query: Query<&CColorSwatchEntities>,
        l_color_swatch_handle_query: Query<&MaterialNode<LColorSwatch>>
    ) {

        let Ok(entities) = color_swatch_entities_query.get(trigger.entity())
            else { warn!("updated color swatch has no entities"); return; };
        let Ok(l_color_swatch_handle) = l_color_swatch_handle_query.get(entities.color_swatch_entity)
            else { warn!("udpated color swatch has no material handle"); return; };
        let Some(l_color_swatch) = l_color_swatches.get_mut(l_color_swatch_handle.id())
            else { warn!("updated color swatch handle does not have a material"); return; };

        let EColorSwatchValueUpdated {
            whiteness_t,
            blackness_t,
            whiteness_digits,
            blackness_digits
        } = trigger.event().clone();
        l_color_swatch.set_whiteness(whiteness_t);
        l_color_swatch.set_blackness(blackness_t);

        commands.trigger_targets(EGridUpdated { t: [blackness_t, whiteness_t] }, entities.color_swatch_entity);
        commands.trigger_targets(EDigitsUpdated { digits: whiteness_digits }, entities.whiteness_t_field_entity);
        commands.trigger_targets(EDigitsUpdated { digits: blackness_digits }, entities.blackness_t_field_entity);
    }
}