use bevy::prelude::*;

use lights_material::{LightsMaterial, LightMaskItem, LIGHT_MASK_COUNT, MAX_LIGHT_ALTER_COUNT};
use color_picker::l_color_swatch::LColorSwatch;
use c_node_size::{RRem, CNodeSize, NodeSizeOperator, NodeWidth, NodeHeight};
use color_picker::c_color_picker_data::CColorPickerData;


mod lights_material;
mod continuous_slider;
mod color_picker;
mod digit_field;
mod selectable_ui;
// mod rem;
// mod em;
mod c_node_size;
mod s_field_digit;

mod node_orientation;


const LIGHTS_MENU_BG_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            UiMaterialPlugin::<LightsMaterial>::default(),
            selectable_ui::SelectableUIPlugin,
            digit_field::DigitFieldPlugin,
            continuous_slider::ContinuousSliderPlugin,
            color_picker::ColorPickerPlugin,
        ))
        .add_event::<digit_field::e_digit_changed::EDigitChanged>()
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
                c_node_size::update::update_node_sizes::<c_node_size::NodeFontSize>
            )
        )
        // .add_systems(Update,
        //     (
        //         // continuous_slider::s_continuous_slider_entities::update::update_slider_on_selected_changed,

        //         // digit_field::s_digit_field_entities::update::update_text_on_selected_changed::<1>,
        //         // digit_field::s_digit_field_entities::update::update_text_on_selected_changed::<2>,
        //         // digit_field::s_digit_field_entities::update::update_text_on_selected_changed::<3>,
        //         // digit_field::s_digit_field_entities::update::update_text_on_selected_changed::<4>,
        //         // digit_field::s_digit_field_entities::update::update_text_on_selected_changed::<5>,
        //         // digit_field::s_digit_field_entities::update::update_text_on_selected_changed::<6>
        //     )
        // )
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
    mut color_swatch_materials: ResMut<Assets<LColorSwatch>>
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
        color_swatch_materials: &mut Assets<LColorSwatch>,
        z_index: i32
    ) -> Entity {

        let color_picker_entities = (0..7).into_iter()
            .map(|_|
                CColorPickerData::build(
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
                    justify_content: JustifyContent::FlexStart,
                    align_content: AlignContent::FlexStart,
                    align_items: AlignItems::FlexStart,
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