use bevy::prelude::*;

use super::{
    e_grid_changed::EGridUpdated,
    c_arrow_input_grid::CArrowInputGrid
};

use super::super::SelectableUIBundle;


pub enum GridDimension { X, Y, Z, W, NegX, NegY, NegZ, NegW }

impl GridDimension {
    pub fn negative_keycode(&self) -> KeyCode {
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

    pub fn positive_keycode(&self) -> KeyCode {
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


#[derive(Component)]
pub struct CGrid<const N: usize, T: Sync + Send + Copy + 'static> {
    dimensions: [GridDimension; N],
    t: [f32; N],
    tag: T
}

impl<const N: usize, T: Sync + Send + Copy+ 'static> CGrid<N, T> {
    pub fn dimensions(&self) -> &[GridDimension; N] { &self.dimensions }
    pub fn t(&self) -> &[f32; N] { &self.t }
    pub fn tag(&self) -> T { self.tag }

    pub fn build(
        commands: &mut Commands,
        dimensions: [GridDimension; N],
        arrow_speed: f32,
        outline_width: Val,
        tag: T
    ) -> Entity {

        commands
            .spawn((
                SelectableUIBundle::new(outline_width),
                CArrowInputGrid::new(arrow_speed),
                Self { dimensions, t: [0.; N], tag }
            ))
            .observe(trigger::update_t::<N, T>)
            .id()
    }
}


mod trigger {
    use super::*;


    pub fn update_t<const N: usize, T: Sync + Send + Copy + 'static>(
        trigger: Trigger<EGridUpdated<N>>,
        mut grid_query: Query<&mut CGrid<N, T>>
    ) {

        let Ok(mut grid) = grid_query.get_mut(trigger.entity())
            else { warn!("updated grid has no grid"); return; };
        
        let t = trigger.event().t;
        grid.t = t;
    }
}