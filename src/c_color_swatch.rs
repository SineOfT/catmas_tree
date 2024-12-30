use bevy::{
    prelude::*,
    render::render_resource::{ShaderRef, AsBindGroup}
};

use super::{
    c_node_size::{CNodeSize, NodeSizeOperator, NodeWidth, NodeHeight},
    c_text_field::CTextField,
    selectable_ui::{
        c_selectable_ui::CSelectableUI,
        grid::{
            GridDimension,
            SelectableGrid,
            ArrowInputGrid        
        }
    }
};





#[derive(AsBindGroup, Asset, TypePath, Clone)]
pub struct ColorSwatchMaterial {
    #[uniform(0)]
    hue: f32,
    #[uniform(1)]
    alpha: f32
}

impl ColorSwatchMaterial {
    pub fn new() -> Self {
        Self { hue: 0., alpha: 1. }
    }

    pub fn build(
        commands: &mut Commands,
        asset_server: &AssetServer,
        color_swatch_materials: &mut Assets<ColorSwatchMaterial>,
        height: CNodeSize<NodeHeight>
    ) -> Entity {

        let whiteness_text_field_entity = CTextField::build(
            commands,
            asset_server,
            CTextField::new(3)
        );
        let blackness_text_field_entity = CTextField::build(
            commands,
            asset_server,
            CTextField::new(3)
        );

        let text_field_entities = commands
            .spawn((
                Node {
                    width: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
            ))
            .add_child(whiteness_text_field_entity)
            .add_child(blackness_text_field_entity)
            .id();

        commands
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
            .add_child(text_field_entities)
            .with_child((
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
                // BorderColor(Color::srgb(0., 0., 1.)),
                Outline {
                    width: Val::Px(5.),
                    offset: Val::ZERO,
                    color: Color::NONE
                },
                MaterialNode(color_swatch_materials.add(ColorSwatchMaterial::new())),
                Button,
                CSelectableUI::<SelectableGrid<2>>::new([GridDimension::X, GridDimension::Y]),
                ArrowInputGrid::new(1.)
            ))
            .id()
    }
}

impl UiMaterial for ColorSwatchMaterial {
    fn fragment_shader() -> ShaderRef {
        "color_wheel_shader.wgsl".into()
    }
}