use bevy::prelude::*;

use super::{EGridChanged, SelectableGrid};

use super::super::{
    c_selectable_ui::CSelectableUI,
    m_selected_ui::MSelectedUI
};


#[derive(Component)]
pub struct CArrowInputGrid {
    arrow_speed: f32
}

impl CArrowInputGrid {
    pub fn new(arrow_speed: f32) -> Self {
        Self { arrow_speed }
    }
}


pub mod update {
    use super::*;


    pub fn update_selected_arrow_input_grid<const N: usize>(
        mut commands: Commands,
        time: Res<Time>,
        keys: Res<ButtonInput<KeyCode>>,
        mut selected_grid_query: Query<(Entity, &mut CSelectableUI<SelectableGrid<N>>, &CArrowInputGrid), With<MSelectedUI>>
    ) {

        for (entity, mut grid, arrow_input) in selected_grid_query.iter_mut() {
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
            commands.trigger_targets(EGridChanged(new_data), entity);
        }
    }
}