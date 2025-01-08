use bevy::prelude::*;

pub use e_grid_changed::{EGridChanged, EGridUpdated};
pub use c_grid::{CGrid, GridDimension};


mod c_arrow_input_grid;
mod c_mouse_input_grid;
mod e_grid_changed;
mod c_grid;


pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update,
                (
                    c_arrow_input_grid::update::update_selected_arrow_input_grid::<1, ()>,
                    c_arrow_input_grid::update::update_selected_arrow_input_grid::<2, ()>,
                    c_arrow_input_grid::update::update_selected_arrow_input_grid::<3, ()>,
                    c_arrow_input_grid::update::update_selected_arrow_input_grid::<4, ()>,
                    c_arrow_input_grid::update::update_selected_arrow_input_grid::<1, super::super::color_picker::ColorPickerTag>,
                    c_arrow_input_grid::update::update_selected_arrow_input_grid::<2, super::super::color_picker::ColorPickerTag>,
                    c_arrow_input_grid::update::update_selected_arrow_input_grid::<3, super::super::color_picker::ColorPickerTag>,
                    c_arrow_input_grid::update::update_selected_arrow_input_grid::<4, super::super::color_picker::ColorPickerTag>
                )
            );
    }
}