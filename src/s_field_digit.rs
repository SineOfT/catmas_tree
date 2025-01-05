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
}