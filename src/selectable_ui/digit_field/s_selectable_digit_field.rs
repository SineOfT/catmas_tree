use bevy::prelude::*;

use super::super::c_selectable_ui::SelectableUIMarker;

use super::super::super::s_field_digit::FieldDigit;


pub struct DigitFieldParam<const N: usize> {
    min_digits: [FieldDigit; N],
    max_digits: [FieldDigit; N],
    decimal_position: Option<usize>
}

impl<const N: usize> DigitFieldParam<N> {    
    pub fn new(min_digits: [FieldDigit; N], max_digits: [FieldDigit; N], decimal_position: Option<usize>) -> Self {
        Self { min_digits, max_digits, decimal_position }
    }

    pub fn min_digits(&self) -> &[FieldDigit; N] { &self.min_digits }
    pub fn max_digits(&self) -> &[FieldDigit; N] { &self.max_digits }
    pub fn decimal_position(&self) -> Option<usize> { self.decimal_position }

}


#[derive(Clone)]
pub struct DigitFieldData<const N: usize> {
    digits: [FieldDigit; N],
    cursor: usize
}

impl<const N: usize> DigitFieldData<N> {
    pub const DEFAULT: Self = Self { digits: [FieldDigit::Zero; N], cursor: 0 };

    pub fn new(digits: [FieldDigit; N]) -> Self {
        Self { digits, cursor: 0 }
    }

    pub fn digits(&self) -> &[FieldDigit; N] { &self.digits }
    pub fn cursor(&self) -> usize { self.cursor }

    pub fn set_digits(&mut self, digits: [FieldDigit; N]) { self.digits = digits; }
    pub fn set_cursor(&mut self, cursor: usize) { self.cursor = cursor; }
}


pub struct SelectableDigitField<const N: usize>;

impl<const N: usize> SelectableUIMarker for SelectableDigitField<N> {
    type ConstructorParam = DigitFieldParam<N>;
    type Data = DigitFieldData<N>;
    const DEFAULT_DATA: Self::Data = DigitFieldData::DEFAULT;
}