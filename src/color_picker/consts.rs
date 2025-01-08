use super::super::s_field_digit::{FieldDigits, FieldDigit};


pub const DIGIT_COUNT: usize = 3;
pub const T_DIGIT_DECIMAL_POSITION: usize = 0;
pub type TFieldDigits = FieldDigits<DIGIT_COUNT, T_DIGIT_DECIMAL_POSITION>;
pub const T_DIGIT_MIN_VALUE: TFieldDigits = FieldDigits::zero();
pub const T_DIGIT_MAX_VALUE: TFieldDigits = FieldDigits::one();

pub const DEG_DIGIT_DECIMAL_POSITION: usize = usize::MAX;
pub type DegFieldDigits = FieldDigits<DIGIT_COUNT, DEG_DIGIT_DECIMAL_POSITION>;
pub const DEG_DIGIT_MIN_VALUE: DegFieldDigits = FieldDigits::zero();
pub const DEG_DIGIT_MAX_VALUE: DegFieldDigits = FieldDigits::new([FieldDigit::Three, FieldDigit::Six, FieldDigit::Zero]);

pub fn get_degree_from_t(t: f32) -> f32 { t * 360. }
pub fn get_t_from_degree(degree: f32) -> f32 { degree / 360. }