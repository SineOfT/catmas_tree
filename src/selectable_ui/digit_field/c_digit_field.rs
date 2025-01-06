use bevy::prelude::*;

use super::super::super::s_field_digit::FieldDigits;


#[derive(Component)]
pub struct CDigitField<const N: usize, const D: usize, T: Sync + Send + Copy = ()> {
    min_digits: FieldDigits<N, D>,
    max_digits: FieldDigits<N, D>,
    digits: FieldDigits<N, D>,
    cursor: usize,
    tag: T
}

impl<const N: usize, const D: usize, T: Sync + Send + Copy> CDigitField<N, D, T> {
    pub fn new(min_digits: FieldDigits<N, D>, max_digits: FieldDigits<N, D>, tag: T) -> Self {
        Self { min_digits, max_digits, digits: FieldDigits::zero(), cursor: 0, tag }
    }

    pub fn min_digits(&self) -> &FieldDigits<N, D> { &self.min_digits }
    pub fn max_digits(&self) -> &FieldDigits<N, D> { &self.max_digits }
    pub fn digits(&self) -> &FieldDigits<N, D> { &self.digits }
    pub fn cursor(&self) -> usize { self.cursor }
    pub fn tag(&self) -> T { self.tag }
}