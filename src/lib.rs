mod roman_number;

use roman_number::ParseRomanDigitError;
use roman_number::RomanNumber;
use std::error::Error;

pub fn run() {}

pub fn roman_to_int(raw_string: &str) -> Result<usize, ParseRomanDigitError> {
    let roman_number = raw_string.parse::<RomanNumber>()?;

    Ok(roman_number.to_decimal())
}

pub fn int_to_roman(raw_string: &str) -> Result<RomanNumber, Box<dyn Error>> {
	let int = raw_string.parse::<usize>()?;

	Ok(RomanNumber::from_decimal(int))
}

#[derive(Debug)]
enum Action {
	RomanToInt(String),
	IntToRoman(String),
}

impl Action {
	pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {

		

	}
}