use bevy::prelude::*;


pub mod c_color_picker_data;
pub mod l_color_swatch;
pub mod s_color_picker_entities;
pub mod s_color_swatch_entities;


pub struct ColorPickerPlugin;

impl Plugin for ColorPickerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(UiMaterialPlugin::<l_color_swatch::LColorSwatch>::default());

    }
}