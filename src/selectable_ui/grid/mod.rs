use bevy::prelude::*;

pub use c_arrow_input_grid::CArrowInputGrid;
pub use c_mouse_input_grid::CMouseInputGrid;
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
                    c_arrow_input_grid::update::update_selected_arrow_input_grid::<1>,
                    c_arrow_input_grid::update::update_selected_arrow_input_grid::<2>,
                    c_arrow_input_grid::update::update_selected_arrow_input_grid::<3>,
                    c_arrow_input_grid::update::update_selected_arrow_input_grid::<4>
                )
            );
    }
}