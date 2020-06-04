use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::{error, fmt};

use RomanDigit::{C, D, I, L, M, V, X};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum RomanDigit {
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

impl Display for RomanDigit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            I => "I",
            V => "V",
            X => "X",
            L => "L",
            C => "C",
            D => "D",
            M => "M",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
pub struct ParseRomanDigitError {
    wrong_string: String,
}

impl fmt::Display for ParseRomanDigitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid characters: {}", self.wrong_string)
    }
}

impl error::Error for ParseRomanDigitError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl FromStr for RomanDigit {
    type Err = ParseRomanDigitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_uppercase();
        match s.as_str() {
            "I" => Ok(I),
            "V" => Ok(V),
            "X" => Ok(X),
            "L" => Ok(L),
            "C" => Ok(C),
            "D" => Ok(D),
            "M" => Ok(M),
            _ => Err(ParseRomanDigitError { wrong_string: s }),
        }
    }
}

impl RomanDigit {
    pub fn to_int(&self) -> usize {
        match self {
            I => 1,
            V => 5,
            X => 10,
            L => 50,
            C => 100,
            D => 500,
            M => 1000,
        }
    }
}

fn get_digit_at(integer: usize, place: usize) -> usize {
    integer % (10 * place) / place
}

#[derive(Debug, Eq, PartialEq)]
pub struct RomanNumber {
    roman_digits: Vec<RomanDigit>,
}

impl RomanNumber {
    pub fn new(roman_digits: Vec<RomanDigit>) -> RomanNumber {
        RomanNumber { roman_digits }
    }

    pub fn from_decimal(integer: usize) -> RomanNumber {
        let mut result = Vec::new();

        result.extend(RomanNumber::convert_thousands_digit(integer));
        result.extend(RomanNumber::convert_digit(integer, 100, C, D, M));
        result.extend(RomanNumber::convert_digit(integer, 10, X, L, C));
        result.extend(RomanNumber::convert_digit(integer, 1, I, V, X));

        RomanNumber {
            roman_digits: result,
        }
    }

    fn convert_thousands_digit(integer: usize) -> Vec<RomanDigit> {
        vec![M; get_digit_at(integer, 1000)]
    }

    fn convert_digit(
        integer: usize,
        place: usize,
        first: RomanDigit,
        second: RomanDigit,
        third: RomanDigit,
    ) -> Vec<RomanDigit> {
        let digit = get_digit_at(integer, place);

        match digit {
            0 => Vec::new(),
            1 | 2 | 3 => vec![first; digit],
            4 => vec![first, second],
            5 => vec![second],
            6 => vec![second, first],
            7 => vec![second, first, first],
            8 => vec![second, first, first, first],
            9 => vec![first, third],
            _ => Vec::new(),
        }
    }

    pub fn to_decimal(&self) -> usize {
        let (total, current): (usize, usize) =
            self.roman_digits.iter().fold((0, 0), |acc, elem| {
                let elem_int = elem.to_int();

                match acc {
                    (total, 0) => (total, elem_int),
                    (total, partial) if partial < elem_int => (total + elem_int - partial, 0),
                    (total, partial) => (total + partial, elem_int),
                }
            });

        total + current
    }
}

impl Display for RomanNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = self
            .roman_digits
            .iter()
            .map(RomanDigit::to_string)
            .collect::<Vec<String>>()
            .join("");

        write!(f, "{}", s)
    }
}

impl FromStr for RomanNumber {
    type Err = ParseRomanDigitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let roman_digits: Vec<RomanDigit> = s
            .chars()
            .map(|c| RomanDigit::from_str(c.to_string().as_str()))
            .collect::<Result<Vec<RomanDigit>, Self::Err>>()?;

        Ok(RomanNumber::new(roman_digits))
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use super::*;

    const INT_RANGE: Range<usize> = 0..5000;

    const ONES: usize = 1;
    const TENS: usize = 10;
    const HUNDREDS: usize = 100;

    #[test]
    fn has_max_3_i() {
        test_property("Roman has max 3 I", |int| {
            max_repetition_of(&RomanNumber::from_decimal(int), I, 3)
        })
    }

    #[test]
    fn has_max_4_x() {
        test_property("Roman has max 4 X", |int| {
            max_repetition_of(&RomanNumber::from_decimal(int), X, 4)
        })
    }

    #[test]
    fn has_max_4_c() {
        test_property("Roman has max 4 C", |int| {
            max_repetition_of(&RomanNumber::from_decimal(int), C, 4)
        })
    }

    #[test]
    fn has_max_1_v() {
        test_property("Roman has max 1 V", |int| {
            max_repetition_of(&RomanNumber::from_decimal(int), V, 1)
        })
    }

    #[test]
    fn has_max_1_l() {
        test_property("Roman has max 1 L", |int| {
            max_repetition_of(&RomanNumber::from_decimal(int), L, 1)
        })
    }

    #[test]
    fn has_max_1_d() {
        test_property("Roman has max 1 D", |int| {
            max_repetition_of(&RomanNumber::from_decimal(int), D, 1)
        })
    }

    #[test]
    fn has_m_if_upper_to_1000() {
        test_property(
            "Roman has at least one M if int is greater than 1000",
            |int| {
                let roman = RomanNumber::from_decimal(int);
                match roman.roman_digits.as_slice() {
                    [M, ..] => int >= 1000,
                    _ => int <= 1000,
                }
            },
        )
    }

    #[test]
    fn int_has_4_ones_roman_has_iv() {
        test_property("Roman has IV if int has 4 ones", |int| {
            let roman = RomanNumber::from_decimal(int);

            four_property(int, &roman, ONES, I, V)
        })
    }

    #[test]
    fn int_has_4_tens_roman_has_xl() {
        test_property("Roman has XL if int has 4 tens", |int| {
            let roman = RomanNumber::from_decimal(int);

            four_property(int, &roman, TENS, X, L)
        })
    }

    #[test]
    fn int_has_4_hundreds_roman_has_cd() {
        test_property("Roman has CD if int has 4 hundreds", |int| {
            let roman = RomanNumber::from_decimal(int);

            four_property(int, &roman, HUNDREDS, C, D)
        })
    }

    #[test]
    fn int_has_9_ones_roman_has_ix() {
        test_property("Roman has IX if int has 9 ones", |int| {
            let roman = RomanNumber::from_decimal(int);

            nine_property(int, &roman, ONES, I, X)
        })
    }

    #[test]
    fn int_has_9_tens_roman_has_xc() {
        test_property("Roman has XC if int has 9 tens", |int| {
            let roman = RomanNumber::from_decimal(int);

            nine_property(int, &roman, TENS, X, C)
        })
    }

    #[test]
    fn int_has_9_hundreds_roman_has_cm() {
        test_property("Roman has CM if int has 9 hundreds", |int| {
            let roman = RomanNumber::from_decimal(int);

            nine_property(int, &roman, HUNDREDS, C, M)
        })
    }

    fn test_property<P>(name: &str, p: P)
    where
        P: Fn(usize) -> bool,
    {
        for int in INT_RANGE {
            assert!(p(int), "Property {} failed for int: {}", name, int);
        }
    }

    fn max_repetition_of(roman: &RomanNumber, rd: RomanDigit, max: usize) -> bool {
        roman.roman_digits.iter().filter(|&&d| d == rd).count() <= max
    }

    fn four_property(
        int: usize,
        roman: &RomanNumber,
        place: usize,
        unit_digit: RomanDigit,
        second_digit: RomanDigit,
    ) -> bool {
        let exist = has_pattern(roman, &[unit_digit, second_digit]);

        if exist {
            get_digit_at(int, place) == 4
        } else {
            get_digit_at(int, place) != 4
        }
    }

    fn nine_property(
        int: usize,
        roman: &RomanNumber,
        place: usize,
        unit_digit: RomanDigit,
        tens_digit: RomanDigit,
    ) -> bool {
        let exist = has_pattern(roman, &[unit_digit, tens_digit]);

        if exist {
            get_digit_at(int, place) == 9
        } else {
            get_digit_at(int, place) != 9
        }
    }

    fn has_pattern(roman: &RomanNumber, pattern: &[RomanDigit]) -> bool {
        roman
            .roman_digits
            .as_slice()
            .windows(pattern.len())
            .any(|chunk| chunk == pattern)
    }

    #[test]
    fn test_convert_int_to_roman_to_int_return_the_same() {
        test_property(
            "Convert int to roman and back to int should be the same",
            |int| {
                let roman = RomanNumber::from_decimal(int);
                let result = roman.to_decimal();

                result == int
            },
        )
    }

    #[test]
    fn test_from_string_cv_to_cv() {
        let result = RomanNumber::from_str("cv").unwrap();

        assert_eq!(result, RomanNumber::new(vec![C, V]))
    }
}
