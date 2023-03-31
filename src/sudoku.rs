#[cfg(test)]


use rand;
use rand::seq::SliceRandom;
use itertools;
use itertools::Itertools;
use super::util::*;
use super::value::SudokuValue;
use std::convert::TryFrom;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct Sudoku {
    board: [[SudokuValue; 9]; 9]
}

impl Sudoku {
    /// Populates a Sudoku board with n hints.
    /// Returns `(unsolved sudoku, solved sudoku)`.
    pub fn new(n: usize) -> (Self, Self) {
        let arr = [[SudokuValue::Empty; 9]; 9];
        let mut sudoku = Sudoku{ board: arr };
        sudoku.fill(0, 0).unwrap();
        let solution = sudoku.clone();
        sudoku.decimate(n).unwrap();

        (sudoku, solution)
    }

    /// Removes elements from a filled sudoku until there
    /// are `n` elements left, while having only 1 solution.
    pub fn decimate(&mut self, n: usize) -> Result<(), String> {
        // < 17 is impossible
        if 17 > n {
            return Err(format!("It is impossible to create a Sudoku with an unique solution, with less than 17 hints. Input was {} hints.", n));
        } else if n > 81 {
            return Err(format!("Impossible to create a Sudoku with {} hints. A Sudoku has just 81 fields.", n));
        } else if n == 81 {
            return Ok(());
        }

        let mut rng = rand::thread_rng();
        // capacity = sudoku_size - number_remaining_elements + 1 (last element doesn't have to be popped, because then the function returns)
        let mut stack: Vec<(SudokuValue, Vec<usize>)> = Vec::with_capacity(9*9 - n + 1);
        let mut all_indices = (0..9).permutations(2).collect_vec();
        loop {
            all_indices.shuffle(&mut rng);
            for index in &all_indices {
                if self.board[index[0]][index[1]] == SudokuValue::Empty {
                    continue;
                }
                // remove from the board
                stack.push((self.board[index[0]][index[1]], index.clone()));
                self.board[index[0]][index[1]] = SudokuValue::Empty;
                if self.solve(0, 0) != 1 {
                    // if there isn't one distinct solution revert
                    match stack.pop() {
                        Some((val, index)) => {
                            self.board[index[0]][index[1]] = val;
                        },
                        None => return Err(format!("Popped with no elements left!"))
                    }
                }
                if self.count(SudokuValue::Empty) == 81-n {
                    return Ok(());
                }
            }
            // if this branch didn't have any unique solutions revert
            match stack.pop() {
                Some((val, index)) => {
                    self.board[index[0]][index[1]] = val;
                },
                None => return Err(format!("Popped with no elements left!"))
            }
        }
    }

    /// Returns the number of different solutions of a given Sudoku
    /// call this function with `sudoku.solve(0, 0)`.
    pub fn solve(&mut self, r: usize, c: usize) -> i32 {
        // code is similar to Sudoku::fill, maybe a helper function can combine some of the code
        let mut counter = 0;
        if self.board[r][c] != SudokuValue::Empty {
            let next_r: usize;
            let next_c: usize;
            match (r, c) {
                // impossible, because then every field, would be filled but it wouldn't be solved
                // (8, 8) => ...
                (_, 8) => {
                    next_r = r + 1;
                    next_c = 0;
                },
                (_, _) => {
                    next_r = r;
                    next_c = c + 1;
                }
            }
            if self.solved() {
                // this recursion reached it's end with a completed Sudoku so return.
                // it isn't possible for another number to fit this r, c value.
                // Mutating the original Sudoku, so it has to be restored
                return 1;
            } else {
                // don't return, other numbers in this field could lead to additional solutions
                return counter + self.solve(next_r, next_c);
                // Mutating the original Sudoku, so it has to be restored
            }
        }
        let sudoku_numbers: [SudokuValue; 9] = SudokuValue::get_number_array();
        for number in &sudoku_numbers {
            self.board[r][c] = *number;
            if self.check(r, c) {
                let next_r: usize;
                let next_c: usize;
                match (r, c) {
                    // impossible, because then every field, would be filled but it wouldn't be solved
                    // (8, 8) => ...
                    (_, 8) => {
                        next_r = r + 1;
                        next_c = 0;
                    },
                    (_, _) => {
                        next_r = r;
                        next_c = c + 1;
                    }
                }
                if self.solved() {
                    // this recursion reached it's end with a completed Sudoku so return.
                    // it isn't possible for another number to fit this r, c value.
                    // Mutating the original Sudoku, so it has to be restored
                    self.board[r][c] = SudokuValue::Empty;
                    return 1;
                } else {
                    // don't return, other numbers in this field could lead to additional solutions
                    counter += self.solve(next_r, next_c);
                    // Mutating the original Sudoku, so it has to be restored
                    self.board[r][c] = SudokuValue::Empty;
                }
            }
        }
        // Mutating the original Sudoku, so it has to be restored
        self.board[r][c] = SudokuValue::Empty;
        counter
    }

    /// Fills an empty sudoku board.
    /// Row and col are the first empty index.
    /// Call this function with `sudoku.fill(0, 0)` to fill an empty board completely.
    pub fn fill(&mut self, r: usize, c: usize) -> Result<(), ()> {
        let mut rng = rand::thread_rng();
        let mut sudoku_numbers: [SudokuValue; 9] = SudokuValue::get_number_array();
        sudoku_numbers.shuffle(&mut rng);
        for number in &sudoku_numbers {
            self.board[r][c] = *number;
            if self.check(r, c) {
                let next_r: usize;
                let next_c: usize;
                match (r,c) {
                    (8, 8) => return Ok(()),
                    (_, 8) => {next_r = r + 1; next_c = 0;},
                    (_, _) => {next_r = r; next_c = c + 1;}
                }
                if self.fill(next_r, next_c).is_ok() {
                    return Ok(());
                } else {
                    self.board[r][c] = SudokuValue::Empty;
                    continue;
                }

            }
        }
        // all numbers failed => impossible to continue from this => return false and make the recursion above try another number
        self.board[r][c] = SudokuValue::Empty;
        Err(())
    }

    /// Returns `true` if the sudoku is completely solved.
    pub fn solved(&self) -> bool {
        if self.count(SudokuValue::Empty) > 0 {
            false
        } else {
            self.check_all()
        }
    }

    /// Checks all rows, cols and squares by checking these indexes.
    /// ```text
    /// X** *** ***
    /// *** X** ***
    /// *** *** X**
    /// *X* *** ***
    /// *** *X* ***  the Xs get checked
    /// *** *** *X*
    /// **X *** ***
    /// *** **X ***
    /// *** *** **X
    /// ```
    pub fn check_all(&self) -> bool {
        let indices = [(0, 0), (1, 3), (2, 6), (3, 1), (4, 4), (5, 7), (6, 2), (7, 5), (8, 8)];
        for (r, c) in &indices {
            if !self.check(*r, *c) {
                return false;
            }
        }
        true
    }

    /// Returns `true` if the row, column and square of the indices have no duplicate SudokuValue.
    pub fn check(&self, r: usize, c: usize) -> bool {
        let mut reference_arr_row: [&SudokuValue; 9] = match self.get_row(r) {
            Some(x) => x,
            None => panic!("IndexError")
        };
        let mut reference_arr_col: [&SudokuValue; 9] = match self.get_column(c) {
            Some(x) => x,
            None => panic!("IndexError")
        };
        let mut reference_arr_square: [&SudokuValue; 9] = match self.get_square(r, c) {
            Some(x) => x,
            None => panic!("IndexError")
        };

        has_only_unique_elements(&mut reference_arr_row, &SudokuValue::Empty)
         && has_only_unique_elements(&mut reference_arr_col, &SudokuValue::Empty)
         && has_only_unique_elements(&mut reference_arr_square, &SudokuValue::Empty)
    }

    /// Returns how often the SudokuValue is on the board.
    pub fn count(&self, val: SudokuValue) -> usize {
        let mut count = 0;
        for r in 0..9 {
            for c in 0..9 {
                if self.board[r][c] == val {
                    count += 1;
                }
            }
        }
        count
    }

    /// Returns the specified row.
    pub fn get_row(&self, r: usize) -> Option<[&SudokuValue; 9]> {
        return Some([
            self.board.get(r)?.get(0)?,
            self.board.get(r)?.get(1)?,
            self.board.get(r)?.get(2)?,
            self.board.get(r)?.get(3)?,
            self.board.get(r)?.get(4)?,
            self.board.get(r)?.get(5)?,
            self.board.get(r)?.get(6)?,
            self.board.get(r)?.get(7)?,
            self.board.get(r)?.get(8)?
        ]);
    }

    /// Returns the specified column.
    pub fn get_column(&self, c: usize) -> Option<[&SudokuValue; 9]> {
        return Some([
            self.board.get(0)?.get(c)?,
            self.board.get(1)?.get(c)?,
            self.board.get(2)?.get(c)?,
            self.board.get(3)?.get(c)?,
            self.board.get(4)?.get(c)?,
            self.board.get(5)?.get(c)?,
            self.board.get(6)?.get(c)?,
            self.board.get(7)?.get(c)?,
            self.board.get(8)?.get(c)?
        ]);
    }

    // Example:
    // Sudoku:
    // 111 222 333
    // 111 222 333
    // 111 222 333
    // 444 555 666
    // 444 555 666  r=4 c=7          -> [&SudokuValue::Eight; 9]
    // 444 555 666
    // 777 888 999
    // 777 888 999
    // 777 888 999
    /// Returns the square in which the indices lie.
    pub fn get_square(&self, r: usize, c: usize) -> Option<[&SudokuValue; 9]> {
        return Some([
            self.board.get((r / 3) * 3 + 0)?.get((c / 3) * 3 + 0)?,
            self.board.get((r / 3) * 3 + 0)?.get((c / 3) * 3 + 1)?,
            self.board.get((r / 3) * 3 + 0)?.get((c / 3) * 3 + 2)?,
            self.board.get((r / 3) * 3 + 1)?.get((c / 3) * 3 + 0)?,
            self.board.get((r / 3) * 3 + 1)?.get((c / 3) * 3 + 1)?,
            self.board.get((r / 3) * 3 + 1)?.get((c / 3) * 3 + 2)?,
            self.board.get((r / 3) * 3 + 2)?.get((c / 3) * 3 + 0)?,
            self.board.get((r / 3) * 3 + 2)?.get((c / 3) * 3 + 1)?,
            self.board.get((r / 3) * 3 + 2)?.get((c / 3) * 3 + 2)?
        ]);
    }

    /// Returns the SudokuValue at the specified indices.
    pub fn get(&self, r: usize, c: usize) -> Option<&SudokuValue> {
        return self.board.get(r)?.get(c);
    }

    /// Sets the value at the specified indices.
    pub fn set(&mut self, r: usize, c: usize, val: SudokuValue) {
        self.board[r][c] = val;
    }
}

impl TryFrom<&str> for Sudoku {
    type Error = String;

    /// The provided `&str` has to have a length of 81 and is only allowed to have the chars in 0-9.
    fn try_from(sud_str: &str) -> Result<Self, Self::Error> {
        if sud_str.len() != 81 {
            return Err(format!("The provided String does not meet the required length of 81, it's length is {}", sud_str.len()));
        }
        let mut sud = Sudoku{board: [[SudokuValue::Empty; 9]; 9]};
        let mut col = 0;
        let mut row = 0;
        let mut err = false;
        for c in sud_str.chars() {
            match c.to_digit(10) {
                Some(x) => sud.board[row][col] = SudokuValue::try_from(i32::try_from(x).unwrap()).unwrap(),
                None => err = true
            }
            match col {
                8 => {col = 0; row += 1}
                _ => col += 1
            }
        }
        match err {
            true => Err(String::from("SudokuValue has to be in the range 0..=9")),
            false => Ok(sud)
        }
    }
}
