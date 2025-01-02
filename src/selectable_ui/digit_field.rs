use bevy::prelude::*;

use super::{
    c_selectable_ui::{CSelectableUI, SelectableUIMarker},
    m_selected_ui::MSelectedUI
};


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
    fn as_char(&self) -> char {
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


pub struct DigitFieldParam<const N: usize> {
    min_digits: [FieldDigit; N],
    max_digits: [FieldDigit; N],
    decimal_position: Option<usize>
}

impl<const N: usize> DigitFieldParam<N> {    
    pub fn new(min_digits: [FieldDigit; N], max_digits: [FieldDigit; N], decimal_position: Option<usize>) -> Self {
        Self { min_digits, max_digits, decimal_position }
    }
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

    pub fn cursor(&self) -> usize { self.cursor }

    pub fn digits_string(&self) -> String {
        String::from_iter(self.digits.iter().map(|digit| digit.as_char()))
    }
}


pub struct SelectableDigitField<const N: usize>;

impl<const N: usize> SelectableUIMarker for SelectableDigitField<N> {
    type ConstructorParam = DigitFieldParam<N>;
    type Data = DigitFieldData<N>;
    const DEFAULT_DATA: Self::Data = DigitFieldData::DEFAULT;
}


#[derive(Component)]
pub struct KeyInputField;


pub mod update {
    use super::*;


    fn add_digit<const N: usize>(mut digits: [FieldDigit; N], cursor: usize, digit: FieldDigit) -> [FieldDigit; N] {
        for i in ((cursor+1)..N).rev() {
            digits[i] = digits[i - 1];
        }
        digits[cursor] = digit;
        return digits;
    } 

    fn delete_digit<const N: usize>(mut digits: [FieldDigit; N], cursor: usize) -> [FieldDigit; N] {
        for i in cursor..(N-1) {
            digits[i] = digits[i + 1];
        }
        digits[N-1] = FieldDigit::Zero;
        return digits;
    }

    fn equal_digits<const N: usize>(prev_digits: &[FieldDigit; N], new_digits: &[FieldDigit; N]) -> bool {
        prev_digits.iter().zip(new_digits.iter()).all(|(prev, new)| prev == new)
    }

    fn clamp_digits<const N: usize>(mut digits: [FieldDigit; N], min_digits: &[FieldDigit; N], max_digits: &[FieldDigit; N]) -> [FieldDigit; N] {
        let mut nearing = 0;
        
        for i in 0..N {
            let digit = digits[i];
            let min_digit = min_digits[i];
            let max_digit = max_digits[i];
            let is_nearing_min = nearing < 0;
            let is_nearing_max = nearing > 0;

            if !is_nearing_max {
                if digit < min_digit {
                    digits = min_digits.clone();
                    return digits;
                }
                if digit == min_digit {
                    nearing -= 1;
                }
            }
            if !is_nearing_min {
                if digit > max_digit {
                    digits = max_digits.clone();
                    return digits;
                }
                if digit == max_digit {
                    nearing += 1;
                }
            }
            if (digit > min_digit || is_nearing_max) &&
               (digit < max_digit || is_nearing_min) {
                return digits;
            }
        }
        
        digits
    }

    pub fn update_selected_key_input_field<const N: usize>(
        keys: Res<ButtonInput<KeyCode>>,
        mut selected_digit_field_query: Query<(&mut CSelectableUI<SelectableDigitField<N>>, &KeyInputField), With<MSelectedUI>>
    ) {

        for (mut digit_field, key_input) in selected_digit_field_query.iter_mut() {                        
            let DigitFieldParam { min_digits, max_digits, decimal_position } = digit_field.param();
            let DigitFieldData { digits: prev_digits, cursor: prev_cursor } = digit_field.data();
            let mut digit_inputs = Vec::new();
            let mut is_deleted = false;
            let mut new_cursor = *prev_cursor as usize;
            let mut is_pressed = false;

            for key in keys.get_just_pressed() {
                match key {
                    KeyCode::Digit0 => { digit_inputs.push(FieldDigit::Zero); is_pressed = true; },
                    KeyCode::Digit1 => { digit_inputs.push(FieldDigit::One); is_pressed = true; },
                    KeyCode::Digit2 => { digit_inputs.push(FieldDigit::Two); is_pressed = true; },
                    KeyCode::Digit3 => { digit_inputs.push(FieldDigit::Three); is_pressed = true; },
                    KeyCode::Digit4 => { digit_inputs.push(FieldDigit::Four); is_pressed = true; },
                    KeyCode::Digit5 => { digit_inputs.push(FieldDigit::Five); is_pressed = true; },
                    KeyCode::Digit6 => { digit_inputs.push(FieldDigit::Six); is_pressed = true; },
                    KeyCode::Digit7 => { digit_inputs.push(FieldDigit::Seven); is_pressed = true; },
                    KeyCode::Digit8 => { digit_inputs.push(FieldDigit::Eight); is_pressed = true; },
                    KeyCode::Digit9 => { digit_inputs.push(FieldDigit::Nine); is_pressed = true; },
                    
                    KeyCode::Backspace => { is_deleted = true; is_pressed = true; },
                    
                    KeyCode::ArrowLeft => { new_cursor = new_cursor.saturating_sub(1); is_pressed = true; },
                    KeyCode::ArrowRight => { new_cursor = (new_cursor + 1).min(N-1); is_pressed = true; },
                    KeyCode::Period => { if let Some(decimal_position) = decimal_position {
                        new_cursor = decimal_position + 1; is_pressed = true
                    } },
                    _ => { }
                }
            }

            if !is_pressed { continue; }

            info!("pressed");

            let mut new_digits = None;

            if is_deleted {
                let digits = delete_digit(new_digits.unwrap_or(prev_digits.clone()), new_cursor);
                if new_cursor > 0 { new_cursor -= 1; }
                new_digits = Some(digits);
            }

            if !digit_inputs.is_empty() {
                let mut digits = new_digits.unwrap_or(prev_digits.clone());
                for digit in digit_inputs {
                    digits = add_digit(digits, new_cursor, digit);
                    if new_cursor < N-1 { new_cursor += 1; }
                }
                new_digits = Some(clamp_digits(digits, min_digits, max_digits));
            }

            let digits_changed = new_digits.map_or(
                false,
                |digits| !equal_digits(prev_digits, &digits)
            );
            let cursor_changed = *prev_cursor != new_cursor;

            let prev_data = digit_field.data_mut();

            if digits_changed { prev_data.digits = new_digits.unwrap(); }
            if cursor_changed { prev_data.cursor = new_cursor; }
        }
    }
}