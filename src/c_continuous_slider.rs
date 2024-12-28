use bevy::prelude::*;

use super::{
    c_node_size::{CNodeSize, NodeSizeOperator, NodeJustify},
    c_text_field::CTextField,
    node_orientation::NodeOrientation
};


#[derive(Component)]
pub struct CContinuousSlider {
    min_value: f32,
    max_value: f32,
    decimal_count: u8,
    t: f32
}

impl CContinuousSlider {
    pub fn new(min_value: f32, max_value: f32, decimal_count: u8) -> Self {
        Self { min_value, max_value, decimal_count, t: 0. }
    }

    pub fn get_value(&self) -> f32 {
        f32::lerp(self.min_value, self.max_value, self.t)
    }


    pub fn build(
        commands: &mut Commands,
        asset_server: &AssetServer,
        slider: Self,
        orientation: NodeOrientation,
        justify_size: CNodeSize<NodeJustify>,
        empty_color: Color,
        hover_color: Color,
        fill_color: Color
    ) -> Entity {

        let text_field_entity = CTextField::build(
            commands,
            asset_server,
            CTextField::new(3)
        );

        let slider_entity = Self::build_slider(
            commands,
            slider,
            orientation,
            empty_color,
            hover_color,
            fill_color
        );

        commands
            .spawn((
                orientation.apply(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Stretch,
                    align_items: AlignItems::Center,
                    ..default()
                }),
                justify_size
            ))
            .add_child(slider_entity)
            .add_child(text_field_entity)
            .id()
    }

    fn build_slider(
        commands: &mut Commands,
        slider: Self,
        orientation: NodeOrientation,
        empty_color: Color,
        hover_color: Color,
        fill_color: Color
    ) -> Entity {
        
        commands
            .spawn((
                orientation.apply(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Start,
                    flex_basis: Val::Percent(100.),
                    ..default()
                }),
                CNodeSize::<NodeJustify>::new(NodeSizeOperator::Em(0.5)),
                BackgroundColor(empty_color),
                slider
            ))
            .with_child((
                orientation.apply(Node {
                    position_type: PositionType::Absolute,
                    width: Val::ZERO,
                    height: Val::Percent(100.),
                    ..default()
                }),
                BackgroundColor(hover_color),
                ZIndex(0)
            ))
            .with_child((
                orientation.apply(Node {
                    position_type: PositionType::Absolute,
                    width: Val::ZERO,
                    height: Val::Percent(100.),
                    ..default()
                }),
                BackgroundColor(fill_color),
                ZIndex(1)
            ))
            .id()
    }
}


pub mod update {
    use super::*;


    pub fn update_slider_on_input(
        
    ) {

    }
}