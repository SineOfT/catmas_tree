use bevy::prelude::*;

use super::super::c_selectable_ui::SelectableUIMarker;


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


pub struct SelectableGrid<const N: usize>;

impl<const N: usize> SelectableUIMarker for SelectableGrid<N> {
    type ConstructorParam = [GridDimension; N];
    type Data = [f32; N];
    const DEFAULT_DATA: Self::Data = [0.; N];
}