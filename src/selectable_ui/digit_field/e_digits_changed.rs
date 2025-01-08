use bevy::prelude::*;

use super::super::super::s_field_digit::FieldDigits;


#[derive(Component)]
pub struct EDigitsChanged<const N: usize, const D: usize, T: Sync + Send + Copy + 'static> {
    pub digits: FieldDigits<N, D>,
    pub tag: T
}

impl<const N: usize, const D: usize, T: Sync + Send + Copy + 'static> Event for EDigitsChanged<N, D, T> {
    type Traversal = &'static Parent;

    const AUTO_PROPAGATE: bool = true;
}


#[derive(Event, Clone)]
pub struct EDigitsUpdated<const N: usize, const D: usize> {
    pub digits: FieldDigits<N, D>
}