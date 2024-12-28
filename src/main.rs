use bevy::prelude::*;

use lights_material::{LightsMaterial, LightMaskItem, LIGHT_MASK_COUNT, MAX_LIGHT_ALTER_COUNT};
use c_color_swatch::ColorSwatchMaterial;
use c_color_picker::CColorPicker;
use c_node_size::{RRem, CNodeSize, NodeSizeOperator, NodeWidth, NodeHeight};

mod lights_material;
mod c_color_swatch;
mod c_continuous_slider;
mod c_color_picker;
mod c_text_field;
mod c_selectable_ui;
// mod rem;
// mod em;
mod c_node_size;

mod node_orientation;


const LIGHTS_MENU_BG_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            UiMaterialPlugin::<LightsMaterial>::default(),
            UiMaterialPlugin::<ColorSwatchMaterial>::default()
        ))
        .insert_resource(RRem(20.))
        .add_systems(Startup,
            (
                instantiate_camera,
                build
            )
        )
        .add_systems(PostStartup,
            (
                c_node_size::update::update_node_sizes::<c_node_size::NodeWidth>,
                c_node_size::update::update_node_sizes::<c_node_size::NodeHeight>,
                c_node_size::update::update_node_sizes::<c_node_size::NodeJustify>,
                c_node_size::update::update_node_sizes::<c_node_size::NodeAlign>,
                c_node_size::update::update_node_sizes::<c_node_size::NodeFontSize>,
            )
        )
        .run();
}

fn instantiate_camera(
    mut commands: Commands
) {
    
    commands.spawn(Camera2d);
}

fn build(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut lights_materials: ResMut<Assets<LightsMaterial>>,
    mut color_swatch_materials: ResMut<Assets<ColorSwatchMaterial>>
) {

    fn build_image(
        commands: &mut Commands,
        asset_server: &AssetServer,
        lights_materials: &mut Assets<LightsMaterial>,
        z_index: i32
    ) -> Entity {

        let (light_mask_items, duration) = lights_red_green_chasing();    

        commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ZIndex(z_index)
            ))
            .with_child((
                Node {
                    width: Val::VMin(100.),
                    height: Val::VMin(100.),
                    ..default()
                },
                MaterialNode(lights_materials.add(LightsMaterial::new(&asset_server, light_mask_items, duration)))
            ))
            .id()
    }

    fn build_lights_menu(
        commands: &mut Commands,
        asset_server: &AssetServer,
        color_swatch_materials: &mut Assets<ColorSwatchMaterial>,
        z_index: i32
    ) -> Entity {

        let color_picker_entities = (0..5).into_iter()
            .map(|_|
                CColorPicker::build(
                    commands,
                    asset_server,
                    color_swatch_materials,
                    CNodeSize::<NodeWidth>::new(NodeSizeOperator::Rem(12.)),
                    CNodeSize::<NodeHeight>::new(NodeSizeOperator::Rem(7.)),
                )
            ).collect::<Vec<_>>();

        commands
            .spawn((
                Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Start,
                    align_content: AlignContent::Start,
                    align_items: AlignItems::Start,
                    flex_wrap: FlexWrap::Wrap,
                    row_gap: Val::ZERO,
                    column_gap: Val::ZERO,
                    ..default()
                },
                ZIndex(z_index)
            ))
            .add_children(&color_picker_entities)
            .id()
    }

    let image_entity = build_image(&mut commands, &asset_server, &mut lights_materials, 0);
    let lights_menu_entity = build_lights_menu(&mut commands, &asset_server, &mut color_swatch_materials, 1);
}

fn lights_red_green_chasing() -> ([LightMaskItem; LIGHT_MASK_COUNT], f32) {
    let mut light_mask_items = [LightMaskItem::default(); LIGHT_MASK_COUNT];
    
    let mut t_timings = [Vec4::ZERO; MAX_LIGHT_ALTER_COUNT];
    t_timings[0] = Vec4::new(0., 0.25, 0., 1.);
    t_timings[1] = Vec4::new(0.25, 0.5, 1., 1.);
    t_timings[2] = Vec4::new(0.5, 0.75, 1., 0.);
    t_timings[3] = Vec4::new(0.75, 1., 0., 0.);

    for i in 0..LIGHT_MASK_COUNT {
        let t_offset = i as f32 / 15.;
        
        if i % 2 == 0 {
            light_mask_items[i] = LightMaskItem::new(Vec3::new(1., 0., 0.), t_offset, t_timings)
        } else {
            light_mask_items[i] = LightMaskItem::new(Vec3::new(0., 1., 0.), t_offset, t_timings)
        }
    }

    let duration = 3.;

    (light_mask_items, duration)
}