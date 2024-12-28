use bevy::prelude::*;

use super::{
    c_node_size::{CNodeSize, NodeSizeOperator, NodeWidth, NodeHeight, NodeJustify},
    c_continuous_slider::CContinuousSlider,
    c_color_swatch::ColorSwatchMaterial,
    node_orientation::NodeOrientation,
};


#[derive(Component)]
pub struct CColorPicker {
    color: Color
}

#[derive(Component)]
struct MColorDisplay;


impl CColorPicker {
    pub fn build(
        commands: &mut Commands,
        asset_server: &AssetServer,
        color_swatch_materials: &mut Assets<ColorSwatchMaterial>,
        width: CNodeSize<NodeWidth>,
        height: CNodeSize<NodeHeight>
    ) -> Entity {

        let color_swatch_entity = ColorSwatchMaterial::build(
            commands,
            asset_server,
            color_swatch_materials,
            height.clone()
        );

        let hue_slider_entity = CContinuousSlider::build(
            commands,
            asset_server,
            CContinuousSlider::new(0., 1., 2),
            NodeOrientation::ThreeForth,
            height.clone_as::<NodeJustify>(),
            Color::srgb(0.8, 0.8, 0.8),
            Color::srgb(0.5, 0.5, 0.5),
            Color::srgb(0.2, 0.2, 0.2)
        );


        let alpha_slider_entity = CContinuousSlider::build(
            commands,
            asset_server,
            CContinuousSlider::new(0., 1., 2),
            NodeOrientation::ThreeForth,
            height.clone_as::<NodeJustify>(),
            Color::srgb(0.8, 0.8, 0.8),
            Color::srgb(0.5, 0.5, 0.5),
            Color::srgb(0.2, 0.2, 0.2)
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
                height.clone(),
                Self { color: Color::NONE }
            ))
            .with_child((
                Node {
                    ..default()
                },
                CNodeSize::<NodeWidth>::new(NodeSizeOperator::Em(1.)),
                height,
                BackgroundColor(Color::srgb(0., 0., 1.))
            ))
            .add_child(color_swatch_entity)
            .add_child(hue_slider_entity)
            .add_child(alpha_slider_entity)
            .id()
    }
}