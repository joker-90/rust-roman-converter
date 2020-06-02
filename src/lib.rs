#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::RomanDigit::{C, D, I, L, M, V, X};

#[derive(Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Copy, Clone)]
enum RomanDigit {
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

impl fmt::Display for RomanDigit {
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

impl FromStr for RomanDigit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = &s.to_uppercase();
        match s.as_str() {
            "I" => Ok(I),
            "V" => Ok(V),
            "X" => Ok(X),
            "L" => Ok(L),
            "C" => Ok(C),
            "D" => Ok(D),
            "M" => Ok(M),
            _ => Err(())
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

#[derive(Debug)]
#[derive(Eq, PartialEq)]
struct RomanNumber {
    roman_digits: Vec<RomanDigit>
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

        RomanNumber { roman_digits: result }
    }

    fn convert_thousands_digit(integer: usize) -> Vec<RomanDigit> {
        vec![M; get_digit_at(integer, 1000)]
    }

    fn convert_digit(integer: usize, place: usize, first: RomanDigit, second: RomanDigit, third: RomanDigit) -> Vec<RomanDigit> {
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
            _ => Vec::new()
        }
    }

    pub fn to_decimal(&self) -> usize {
        let (total, current): (usize, usize) = self.roman_digits.iter()
            .fold((0, 0), |acc, elem| {
                let elem_int = elem.to_int();

                match acc {
                    (total, 0) => (total, elem_int),
                    (total, partial) if partial < elem_int => (total + elem_int - partial, 0),
                    (total, partial) => (total + partial, elem_int)
                }
            });

        total + current
    }
}

impl Display for RomanNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = self.roman_digits.iter()
            .map(RomanDigit::to_string)
            .collect::<Vec<String>>()
            .join("");

        write!(f, "{}", s)
    }
}

impl FromStr for RomanNumber {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c: Vec<RomanDigit> = s.chars()
            .map(|c| RomanDigit::from_str(c.to_string().as_str()))
            .collect::<Result<Vec<RomanDigit>, Self::Err>>()?;

        Ok(RomanNumber::new(c))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_i() {
        let result = RomanDigit::from_str("I").unwrap();

        assert_eq!(result, I)
    }

    #[test]
    fn has_max_3_i() {
        for int in 0..4000 {
            assert!(max_repetition_of(&RomanNumber::from_decimal(int), I, 3), "property test failed for int: {}", int)
        }
    }

    #[test]
    fn has_max_4_x() {
        for int in 0..4000 {
            assert!(max_repetition_of(&RomanNumber::from_decimal(int), X, 4), "property test failed for int: {}", int)
        }
    }

    #[test]
    fn has_max_4_c() {
        for int in 0..4000 {
            assert!(max_repetition_of(&RomanNumber::from_decimal(int), C, 4), "property test failed for int: {}", int)
        }
    }

    #[test]
    fn has_max_1_v() {
        for int in 0..4000 {
            assert!(max_repetition_of(&RomanNumber::from_decimal(int), V, 1), "property test failed for int: {}", int)
        }
    }

    #[test]
    fn has_max_1_l() {
        for int in 0..4000 {
            assert!(max_repetition_of(&RomanNumber::from_decimal(int), L, 1), "property test failed for int: {}", int)
        }
    }

    #[test]
    fn has_max_1_d() {
        for int in 0..4000 {
            assert!(max_repetition_of(&RomanNumber::from_decimal(int), D, 1), "property test failed for int: {}", int)
        }
    }

    #[test]
    fn int_has_4_ones_roman_has_iv() {
        for int in 0..4000 {
            let roman = RomanNumber::from_decimal(int);

            four_property(int, &roman, 1, I, V)
        }
    }

    #[test]
    fn int_has_4_tens_roman_has_xl() {
        for int in 0..4000 {
            let roman = RomanNumber::from_decimal(int);

            four_property(int, &roman, 10, X, L)
        }
    }

    #[test]
    fn int_has_4_hundreds_roman_has_cd() {
        for int in 0..4000 {
            let roman = RomanNumber::from_decimal(int);

            four_property(int, &roman, 100, C, D)
        }
    }

    #[test]
    fn int_has_9_ones_roman_has_ix() {
        for int in 0..4000 {
            let roman = RomanNumber::from_decimal(int);

            nine_property(int, &roman, 1, I, X)
        }
    }

    #[test]
    fn int_has_9_tens_roman_has_xc() {
        for int in 0..4000 {
            let roman = RomanNumber::from_decimal(int);

            nine_property(int, &roman, 10, X, C)
        }
    }

    #[test]
    fn int_has_9_hundreds_roman_has_cm() {
        for int in 0..4000 {
            let roman = RomanNumber::from_decimal(int);

            nine_property(int, &roman, 100, C, M)
        }
    }

    fn max_repetition_of(rn: &RomanNumber, rd: RomanDigit, max: usize) -> bool {
        rn.roman_digits.iter()
            .filter(|&&d| d == rd)
            .count() <= max
    }

    fn four_property(int: usize, roman: &RomanNumber, place: usize, unit_digit: RomanDigit, five_digit: RomanDigit) {
        let exist = roman.roman_digits.as_slice().windows(2)
            .any(|chunk|
                match chunk {
                    [stl, last] if last == &five_digit && stl == &unit_digit => true,
                    _ => false
                });

        if exist {
            assert_eq!(get_digit_at(int, place), 4, "unexpected match with {}{} in int: {}, was: {}", unit_digit, five_digit, int, roman)
        } else {
            assert_ne!(get_digit_at(int, place), 4, "missing {}{} in int: {}, was: {}", unit_digit, five_digit, int, roman)
        }
    }

    fn nine_property(int: usize, roman: &RomanNumber, place: usize, unit_digit: RomanDigit, tens_digit: RomanDigit) {
        let exist = roman.roman_digits.as_slice().windows(2)
            .any(|chunk| {
                match chunk {
                    [stl, last] if last == &tens_digit && stl == &unit_digit => true,
                    _ => false
                }
            });

        if exist {
            assert_eq!(get_digit_at(int, place), 9, "unexpected match with {}{} in int: {}, was: {}", unit_digit, tens_digit, int, roman)
        } else {
            assert_ne!(get_digit_at(int, place), 9, "missing {}{} in int: {}, was: {}", unit_digit, tens_digit, int, roman)
        }
    }

    #[test]
    fn test_convert_int_to_roman_to_int_return_the_same() {
        for int in 0..4000 {
            let roman = RomanNumber::from_decimal(int);
            let result = roman.to_decimal();
            assert_eq!(result, int)
        }
    }

    // #[test]
    // fn test_convert_i_to_1() {
    //     let result = RomanNumber::new(vec![I]).to_decimal();
    //
    //     assert_eq!(result, 1)
    // }
    //
    // #[test]
    // fn test_convert_xix_to_19() {
    //     let result = RomanNumber::new(vec![X, I, X]).to_decimal();
    //
    //     assert_eq!(result, 19)
    // }
    //
    // #[test]
    // fn test_from_string_cv_to_cv() {
    //     let result = RomanNumber::from_str("cv").unwrap();
    //
    //     assert_eq!(result, RomanNumber::new(vec![C, V]))
    // }
}