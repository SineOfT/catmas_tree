use bevy::prelude::*;

use grid::SelectableGrid;


pub mod grid;
pub mod e_selected_updated;
pub mod c_selectable_ui;
pub mod m_selected_ui;


pub struct SelectableUIPlugin;

impl Plugin for SelectableUIPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_event::<e_selected_updated::ESelectedUpdated<>>()
            .add_systems(Update, 
                (
                    // c_selectable_ui::update::update_on_change::<SelectableGrid<1>>,
                    // c_selectable_ui::update::update_on_change::<SelectableGrid<2>>,
                    // c_selectable_ui::update::update_on_change::<SelectableGrid<3>>,
                    // c_selectable_ui::update::update_on_change::<SelectableGrid<4>>,

                    grid::update::update_selected_arrow_input_grid::<1>,
                    grid::update::update_selected_arrow_input_grid::<2>,
                    grid::update::update_selected_arrow_input_grid::<3>,
                    grid::update::update_selected_arrow_input_grid::<4>,

                    m_selected_ui::update::update_selected_on_click::<SelectableGrid<1>>,
                    m_selected_ui::update::update_selected_on_click::<SelectableGrid<2>>,
                    m_selected_ui::update::update_selected_on_click::<SelectableGrid<3>>,
                    m_selected_ui::update::update_selected_on_click::<SelectableGrid<4>>,
                    m_selected_ui::update::update_outline_on_selected,
                    m_selected_ui::update::update_outline_on_deselected,
                )
            );
    }
}