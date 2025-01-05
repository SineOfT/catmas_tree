use bevy::prelude::*;

use super::{
    l_color_swatch::LColorSwatch,
    s_color_picker_entities::ColorPickerEntities
};

use super::super::c_node_size::{CNodeSize, NodeWidth, NodeHeight};


#[derive(Component)]
pub struct CColorPickerData {
    entities: ColorPickerEntities,
    color: Color
}

impl CColorPickerData {
    pub fn new(entities: ColorPickerEntities) -> Self {
        Self { entities, color: Color::NONE }
    }

    pub fn build(
        commands: &mut Commands,
        asset_server: &AssetServer,
        color_swatch_materials: &mut Assets<LColorSwatch>,
        width: CNodeSize<NodeWidth>,
        height: CNodeSize<NodeHeight>
    ) -> Entity {

        let (entity, entities) = ColorPickerEntities::build(
            commands,
            asset_server,
            color_swatch_materials,
            width,
            height
        );

        commands.entity(entity).insert(Self::new(entities));

        entity
    }
}