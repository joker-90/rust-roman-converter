use std::error::Error;

use roman_number::RomanNumber;

use crate::Action::{IntToRoman, RomanToInt};

mod roman_number;

pub fn run(action: Action) -> Result<String, Box<dyn Error>> {
    match action {
        RomanToInt(raw) => roman_to_int(raw.as_str()).map(|int| int.to_string()),
        IntToRoman(raw) => int_to_roman(raw.as_str()).map(|rn| rn.to_string()),
    }
}

fn roman_to_int(raw_string: &str) -> Result<usize, Box<dyn Error>> {
    let roman_number = raw_string.parse::<RomanNumber>()?;

    Ok(roman_number.to_decimal())
}

fn int_to_roman(raw_string: &str) -> Result<RomanNumber, Box<dyn Error>> {
    let int = raw_string.parse::<usize>()?;

    Ok(RomanNumber::from_decimal(int))
}

#[derive(Debug)]
pub enum Action {
    RomanToInt(String),
    IntToRoman(String),
}

impl Action {
    pub fn new(mut args: std::env::Args) -> Result<Action, &'static str> {
        args.next(); // Discard program name

        let action_type = args.next().ok_or("Missing action type, it must be either intToRoman or romanToInt")?;
        let to_convert = args.next().ok_or("Missing argument to convert")?;

        match action_type.as_str() {
            "intToRoman" => Ok(IntToRoman(to_convert)),
            "romanToInt" => Ok(RomanToInt(to_convert)),
            _ => Err("Wrong action, it must be either intToRoman or romanToInt")
        }
    }
}

#[cfg(test)]
mod test {
    use crate::run;
    use crate::Action::{RomanToInt, IntToRoman};

    #[test]
    fn run_roman_to_int_should_return_a_int() {
        let result = run(RomanToInt("XC".to_string())).unwrap();

        assert_eq!(result, "90")
    }

    #[test]
    fn run_int_to_roman_should_return_a_roman_number() {
        let result = run(IntToRoman("1142".to_string())).unwrap();

        assert_eq!(result, "MCXLII")
    }
}