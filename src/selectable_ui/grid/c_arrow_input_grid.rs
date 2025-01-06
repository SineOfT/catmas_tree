use bevy::{
    prelude::*,
    input::keyboard::{Key, KeyboardInput}
};

use super::{EGridChanged, CGrid};

use super::super::m_selectable_ui::MSelectedUI;


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
        // trigger: Trigger<KeyboardInput>,
        mut commands: Commands,
        time: Res<Time>,
        keys: Res<ButtonInput<KeyCode>>,
        selected_grid_query: Query<(Entity, &CGrid<N>, &CArrowInputGrid), With<MSelectedUI>>
    ) {

        for (entity, grid, arrow_input) in selected_grid_query.iter() {
            let mut direction = [0.; N];
            let mut is_pressed = false;
            let mut is_changed = false;

            for (index, dimension) in grid.dimensions().iter().enumerate() {
                if keys.pressed(dimension.negative_keycode()) { direction[index] -= 1.; is_pressed = true; }
                if keys.pressed(dimension.positive_keycode()) { direction[index] += 1.; is_pressed = true; }
            }

            if !is_pressed { continue; }

            let move_distance = arrow_input.arrow_speed * time.delta_secs();
            let prev_t = grid.data();
            let mut new_t = prev_t.clone();
            for i in 0..N {
                new_t[i] = (new_t[i] + move_distance * direction[i]).clamp(0., 1.);
                if new_t[i] != prev_t[i] { is_changed = true; }
            }
            
            if !is_changed { continue; }

            commands.trigger_targets(EGridChanged { t: new_t, tag: grid.tag() }, entity);
        }
    }
}