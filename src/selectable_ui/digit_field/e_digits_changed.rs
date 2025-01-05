use bevy::prelude::*;

use super::super::super::s_field_digit::FieldDigit;


#[derive(Component)]
pub struct EDigitsChanged<const N: usize>(pub [FieldDigit; N]);

impl<const N: usize> Event for EDigitsChanged<N> {
    type Traversal = &'static Parent;

    const AUTO_PROPAGATE: bool = true;
}


#[derive(Event)]
pub struct EDigitsUpdated<const N: usize>(pub [FieldDigit; N]);