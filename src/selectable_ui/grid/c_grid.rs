use bevy::prelude::*;


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
pub struct CGrid<const N: usize, T: Sync + Send + Copy = ()> {
    dimensions: [GridDimension; N],
    data: [f32; N],
    tag: T
}

impl<const N: usize, T: Sync + Send + Copy> CGrid<N, T> {
    pub fn new(dimensions: [GridDimension; N], tag: T) -> Self {
        Self { dimensions, data: [0.; N], tag }
    }

    pub fn dimensions(&self) -> &[GridDimension; N] { &self.dimensions }
    pub fn data(&self) -> &[f32; N] { &self.data }
    pub fn tag(&self) -> T { self.tag }
}