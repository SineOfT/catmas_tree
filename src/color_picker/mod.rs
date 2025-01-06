use bevy::prelude::*;

pub use c_color_picker_data::CColorPickerData;
pub use c_color_picker_entities::CColorPickerEntities;
pub use l_color_swatch::LColorSwatch;


mod c_color_picker_data;
mod c_color_picker_entities;
mod c_color_swatch_entities;
mod e_color_picker_updated;
mod l_color_swatch;
mod consts;


pub struct ColorPickerPlugin;

impl Plugin for ColorPickerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(UiMaterialPlugin::<l_color_swatch::LColorSwatch>::default());

    }
}