use rand::distributions::{Distribution, Standard};
use std::convert::TryFrom;
//use super::Sudoku;

/// [Sudoku] is filled with SudokuValue
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub enum SudokuValue {
    Empty = 0,
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

impl SudokuValue {
    pub fn get_number_array() -> [SudokuValue; 9] {
        [SudokuValue::One, SudokuValue::Two, SudokuValue::Three, SudokuValue::Four, SudokuValue::Five, SudokuValue::Six, SudokuValue::Seven, SudokuValue::Eight, SudokuValue::Nine]
    }

    pub fn get_number_vec() -> Vec<SudokuValue> {
        vec![SudokuValue::One, SudokuValue::Two, SudokuValue::Three, SudokuValue::Four, SudokuValue::Five, SudokuValue::Six, SudokuValue::Seven, SudokuValue::Eight, SudokuValue::Nine]
    }
}

impl std::fmt::Display for SudokuValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SudokuValue::Empty => write!(f, " "),
            num => write!(f, "{}", (*num as i32))
        }
    }
}

impl Iterator for SudokuValue {
    type Item = SudokuValue;
    fn next(&mut self) -> Option<SudokuValue> {
        match self {
            SudokuValue::Empty  => None,
            SudokuValue::Nine   => None,
            _ => {
                let num = *self as i32 + 1;
                *self = SudokuValue::try_from(num).unwrap();
                Some(*self)
            }
        }
    }
}

/// Only generates number values, not [SudokuValue::Empty].
impl Distribution<SudokuValue> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> SudokuValue {
        SudokuValue::try_from(rng.gen_range(1..=9)).unwrap()
    }
}

impl TryFrom<i32> for SudokuValue {
    type Error = &'static str;

    /// 0 corresponds to [SudokuValue::Empty], the other numbers 1..=9 correspond to their SudokuValue.
    fn try_from(num: i32) -> Result<Self, Self::Error> {
        match num {
            0 => Ok(SudokuValue::Empty),
            1 => Ok(SudokuValue::One),
            2 => Ok(SudokuValue::Two),
            3 => Ok(SudokuValue::Three),
            4 => Ok(SudokuValue::Four),
            5 => Ok(SudokuValue::Five),
            6 => Ok(SudokuValue::Six),
            7 => Ok(SudokuValue::Seven),
            8 => Ok(SudokuValue::Eight),
            9 => Ok(SudokuValue::Nine),
            _ => Err("SudokuValue has to be in the range 0..=9")
        }
    }
}
