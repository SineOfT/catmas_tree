use bevy::prelude::*;

use super::{
    m_selected_ui::MSelectedUI,
    c_selectable_ui::{CSelectableUI, SelectableUIMarker}
};


pub enum GridDimension { X, Y, Z, W }

impl GridDimension {
    fn negative_keycode(&self) -> KeyCode {
        match self {
            GridDimension::X => KeyCode::ArrowLeft,
            GridDimension::Y => KeyCode::ArrowUp,
            GridDimension::Z => KeyCode::KeyA,
            GridDimension::W => KeyCode::KeyW
        }
    }

    fn positive_keycode(&self) -> KeyCode {
        match self {
            GridDimension::X => KeyCode::ArrowRight,
            GridDimension::Y => KeyCode::ArrowDown,
            GridDimension::Z => KeyCode::KeyD,
            GridDimension::W => KeyCode::KeyS
        }
    }
}


pub struct SelectableGrid<const N: usize>;

impl<const N: usize> SelectableUIMarker for SelectableGrid<N> {
    type ConstructorParam = [GridDimension; N];
    type Data = [f32; N];
    const DEFAULT_DATA: Self::Data = [0.; N];
}


#[derive(Component)]
pub struct ArrowInputGrid {
    arrow_speed: f32
}

impl ArrowInputGrid {
    pub fn new(arrow_speed: f32) -> Self {
        Self { arrow_speed }
    }
}


#[derive(Component)]
pub struct MouseInputGrid;


pub mod update {
    use super::*;


    pub fn update_selected_arrow_input_grid<const N: usize>(
        time: Res<Time>,
        keys: Res<ButtonInput<KeyCode>>,
        mut selected_grid_query: Query<(&mut CSelectableUI<SelectableGrid<N>>, &ArrowInputGrid), With<MSelectedUI>>
    ) {

        for (mut grid, arrow_input) in selected_grid_query.iter_mut() {
            let mut direction = [0.; N];
            let mut is_changed = false;

            for (index, dimension) in grid.param().iter().enumerate() {
                if keys.pressed(dimension.negative_keycode()) { direction[index] -= 1.; }
                if keys.pressed(dimension.positive_keycode()) { direction[index] += 1.; }

                if direction[index] != 0. { is_changed = true; }
            }

            if !is_changed { continue; };

            let move_distance = arrow_input.arrow_speed * time.delta_secs();
            let mut new_data = grid.data();
            for i in 0..N {
                new_data[i] += move_distance * direction[i];
            }
            
            grid.set_data(new_data);
        }
    }

    // pub fn update_selected_mouse_input_grid(

    //     mut selected_grid_query: Query<(&mut SelectableGrid, &ArrowInputGrid), With<MSelectedUI>>
    // )
}