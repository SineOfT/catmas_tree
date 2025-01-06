use bevy::prelude::*;

use super::{ECursorChanged, EDigitsChanged, CDigitField};

use super::super::{
    c_selectable_ui::CSelectableUI,
    m_selectable_ui::MSelectedUI
};

use super::super::super::s_field_digit::{FieldDigit, FieldDigits};



#[derive(Component)]
pub struct CKeyInputField;


pub mod update {
    use super::*;


    fn add_digit<const N: usize, const D: usize>(
        mut digits: FieldDigits<N, D>, cursor: usize, digit: FieldDigit
    ) -> FieldDigits<N, D> {

        for i in ((cursor+1)..N).rev() {
            digits[i] = digits[i - 1];
        }
        digits[cursor] = digit;
        return digits;
    } 

    fn delete_digit<const N: usize, const D: usize>(
        mut digits: FieldDigits<N, D>, cursor: usize
    ) -> FieldDigits<N, D> {

        for i in cursor..(N-1) {
            digits[i] = digits[i + 1];
        }
        digits[N-1] = FieldDigit::Zero;
        return digits;
    }

    fn equal_digits<const N: usize, const D: usize>(
        prev_digits: &FieldDigits<N, D>, new_digits: &FieldDigits<N, D>
    ) -> bool {

        prev_digits.iter().zip(new_digits.iter()).all(|(prev, new)| prev == new)
    }

    fn clamp_digits<const N: usize, const D: usize>(
        mut digits: FieldDigits<N, D>, min_digits: &FieldDigits<N, D>, max_digits: &FieldDigits<N, D>
    ) -> FieldDigits<N, D> {

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

    pub fn update_selected_key_input_field<const N: usize, const D: usize>(
        mut commands: Commands,
        keys: Res<ButtonInput<KeyCode>>,
        selected_digit_field_query: Query<(Entity, &CDigitField<N, D>, &CKeyInputField), With<MSelectedUI>>
    ) {

        for (entity, digit_field, key_input) in selected_digit_field_query.iter() {                        
            let prev_cursor = digit_field.cursor();
            let mut new_cursor = prev_cursor as usize;
            let mut digit_inputs = Vec::new();
            let mut is_deleted = false;
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
                    KeyCode::Period => { if D < N-1 { new_cursor = D + 1; is_pressed = true } },
                    _ => { }
                }
            }

            if !is_pressed { continue; }

            info!("pressed");

            let min_digits = digit_field.min_digits();
            let max_digits = digit_field.max_digits();
            let prev_digits = digit_field.digits();
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

            if let Some(new_digits) = new_digits {
                if !equal_digits(prev_digits, &new_digits) {
                    commands.trigger_targets(EDigitsChanged { digits: new_digits, tag: digit_field.tag() }, entity);
                }
            }

            if prev_cursor != new_cursor {
                commands.trigger_targets(ECursorChanged { cursor: new_cursor, tag: digit_field.tag() }, entity);
            }
        }
    }
}