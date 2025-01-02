use bevy::prelude::*;

use super::{
    m_selected_ui::MSelectedUI,
    c_selectable_ui::{CSelectableUI, SelectableUIMarker}
};


pub enum GridDimension { X, Y, Z, W, NegX, NegY, NegZ, NegW }

impl GridDimension {
    fn negative_keycode(&self) -> KeyCode {
        match self {
            Self::X => KeyCode::ArrowLeft,
            Self::Y => KeyCode::ArrowUp,
            Self::Z => KeyCode::KeyA,
            Self::W => KeyCode::KeyW,
            Self::NegX => Self::X.positive_keycode(),
            Self::NegY => Self::Y.positive_keycode(),
            Self::NegZ => Self::Z.positive_keycode(),
            Self::NegW => Self::W.positive_keycode()
        }
    }

    fn positive_keycode(&self) -> KeyCode {
        match self {
            Self::X => KeyCode::ArrowRight,
            Self::Y => KeyCode::ArrowDown,
            Self::Z => KeyCode::KeyD,
            Self::W => KeyCode::KeyS,
            Self::NegX => Self::X.negative_keycode(),
            Self::NegY => Self::Y.negative_keycode(),
            Self::NegZ => Self::Z.negative_keycode(),
            Self::NegW => Self::W.negative_keycode()
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
            let mut is_pressed = false;
            let mut is_changed = false;

            for (index, dimension) in grid.param().iter().enumerate() {
                if keys.pressed(dimension.negative_keycode()) { direction[index] -= 1.; is_pressed = true; }
                if keys.pressed(dimension.positive_keycode()) { direction[index] += 1.; is_pressed = true; }
            }

            if !is_pressed { continue; }

            let move_distance = arrow_input.arrow_speed * time.delta_secs();
            let prev_data = grid.data();
            let mut new_data = prev_data.clone();
            for i in 0..N {
                new_data[i] = (new_data[i] + move_distance * direction[i]).clamp(0., 1.);
                if new_data[i] != prev_data[i] { is_changed = true; }
            }
            
            if !is_changed { continue; }

            grid.set_data(new_data);
        }
    }

    // pub fn update_selected_mouse_input_grid(

    //     mut selected_grid_query: Query<(&mut SelectableGrid, &ArrowInputGrid), With<MSelectedUI>>
    // )
}