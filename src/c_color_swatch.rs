use bevy::{
    prelude::*,
    render::render_resource::{ShaderRef, AsBindGroup}
};

use super::{
    c_node_size::{CNodeSize, NodeSizeOperator, NodeWidth, NodeHeight},
    digit_field::c_digit_field::CDigitField,
    selectable_ui::{
        SelectableUIBundle,
        c_selectable_ui::CSelectableUI,
        grid::{
            GridDimension,
            SelectableGrid,
            ArrowInputGrid        
        },
        digit_field::FieldDigit
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
    const DIGIT_COUNT: usize = 3;
    const DIGIT_DECIMAL_POSITION: Option<usize> = Some(0);
    const DIGIT_MIN_VALUE: [FieldDigit; Self::DIGIT_COUNT] = [FieldDigit::Zero, FieldDigit::Zero, FieldDigit::Zero];
    const DIGIT_MAX_VALUE: [FieldDigit; Self::DIGIT_COUNT] = [FieldDigit::One, FieldDigit::Zero, FieldDigit::Zero];


    pub fn new() -> Self {
        Self { hue: 0., alpha: 1. }
    }

    pub fn build(
        commands: &mut Commands,
        asset_server: &AssetServer,
        color_swatch_materials: &mut Assets<ColorSwatchMaterial>,
        height: CNodeSize<NodeHeight>
    ) -> Entity {

        let whiteness_field_entity = CDigitField::build::<{Self::DIGIT_COUNT}>(
            commands,
            asset_server,
            Self::DIGIT_MIN_VALUE,
            Self::DIGIT_MAX_VALUE,
            Self::DIGIT_DECIMAL_POSITION
        );
        let blackness_field_entity = CDigitField::build::<{Self::DIGIT_COUNT}>(
            commands,
            asset_server,
            Self::DIGIT_MIN_VALUE,
            Self::DIGIT_MAX_VALUE,
            Self::DIGIT_DECIMAL_POSITION
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
            .add_child(whiteness_field_entity)
            .add_child(blackness_field_entity)
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
            .add_child(value_field_entities)
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
                MaterialNode(color_swatch_materials.add(ColorSwatchMaterial::new())),
                SelectableUIBundle::new(
                    CSelectableUI::<SelectableGrid<2>>::new([GridDimension::X, GridDimension::Y]),
                    Val::Px(5.),
                ),
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