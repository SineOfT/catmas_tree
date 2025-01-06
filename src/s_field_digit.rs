use bevy::prelude::*;


#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum FieldDigit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine
}

impl FieldDigit {
    pub fn as_char(&self) -> char {
        match self {
            Self::Zero  => '0',
            Self::One   => '1',
            Self::Two   => '2',
            Self::Three => '3',
            Self::Four  => '4',
            Self::Five  => '5',
            Self::Six   => '6',
            Self::Seven => '7',
            Self::Eight => '8',
            Self::Nine  => '9'
        }
    }

    pub fn as_num<N: From<u8>>(&self) -> N {
        match self {
            Self::Zero  => 0,
            Self::One   => 1,
            Self::Two   => 2,
            Self::Three => 3,
            Self::Four  => 4,
            Self::Five  => 5,
            Self::Six   => 6,
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine  => 9
        }.into()
    }

    pub fn from_int(int: u8) -> Self {
        match int {
            0 => Self::Zero,
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            9 => Self::Nine,
            n => { warn!("invalid digit {}", n); Self::Zero }
        }
    }
}


#[derive(Deref, DerefMut, Clone)]
pub struct FieldDigits<const N: usize, const D: usize> {
    #[deref]
    digits: [FieldDigit; N]
}

impl<const N: usize, const D: usize> FieldDigits<N, D> {
    pub const fn zero() -> Self {
        let digits = [FieldDigit::Zero; N];
        Self { digits }
    }

    pub const fn one() -> Self {
        let mut digits = [FieldDigit::Zero; N];
        let decimal_position = if D > N-1 { N-1 } else { D };
        digits[decimal_position] = FieldDigit::One;
        Self { digits }
    }
    
    pub const fn new(digits: [FieldDigit; N]) -> Self {
        Self { digits }
    }

    pub fn from_value(value: f32) -> Self {
        let decimal_position = D.min(N-1) as i32;
        let mut digits = [FieldDigit::Zero; N];
        for i in 0..N {
            let digit_position = decimal_position - i as i32;
            let digit_int = (value * 10_f32.powi(-digit_position) + 0.5) as i32 % 1;
            digits[i] = FieldDigit::from_int(digit_int as u8);
        }
    
        Self { digits }
    }
    
    pub fn to_value(&self) -> f32 {
        let decimal_position = D.min(N-1) as i32;
        let mut value = 0.;
        for (i, digit) in self.digits.iter().enumerate() {
            let digit_position = decimal_position - i as i32;
            value += digit.as_num::<f32>() * 10_f32.powi(digit_position);
        }
        value
    }
}