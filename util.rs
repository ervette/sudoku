use crossterm::{Result, event};
use box_drawing::light;
use rudoku_core::sudoku::sudoku_value::SudokuValue;
use std::convert::TryFrom;

pub fn read_key_code() -> Result<event::KeyCode> {
    loop {
        if let Ok(event::Event::Key(event::KeyEvent { code: k, .. })) = event::read() {
            return Ok(k);
        }
    }
}

pub fn is_up(kc: event::KeyCode) -> bool {
    kc == event::KeyCode::Up || kc == event::KeyCode::Char('w') || kc == event::KeyCode::Char('k')
}

pub fn is_left(kc: event::KeyCode) -> bool {
    kc == event::KeyCode::Left || kc == event::KeyCode::Char('a') || kc == event::KeyCode::Char('h')
}

pub fn is_down(kc: event::KeyCode) -> bool {
    kc == event::KeyCode::Down || kc == event::KeyCode::Char('s') || kc == event::KeyCode::Char('j')
}

pub fn is_right(kc: event::KeyCode) -> bool {
    kc == event::KeyCode::Right || kc == event::KeyCode::Char('d') || kc == event::KeyCode::Char('l')
}

pub fn key_code_to_sudoku_value(kc: event::KeyCode) -> Option<SudokuValue> {
    match kc {
        event::KeyCode::Char(x) if ('0'..='9').contains(&x) => SudokuValue::try_from(x.to_digit(10).unwrap() as i32).ok(),
        event::KeyCode::Delete => Some(SudokuValue::Empty),
        event::KeyCode::Backspace => Some(SudokuValue::Empty),
        _ => None
    }
}

pub fn top_bar() -> String {
    bar(light::DOWN_RIGHT, light::DOWN_LEFT, light::DOWN_HORIZONTAL, light::HORIZONTAL)
}

pub fn bot_bar() -> String {
    bar(light::UP_RIGHT, light::UP_LEFT, light::UP_HORIZONTAL, light::HORIZONTAL)
}

pub fn div_bar() -> String {
    bar(light::VERTICAL_RIGHT, light::VERTICAL_LEFT, light::VERTICAL_HORIZONTAL, light::HORIZONTAL)
}

pub fn num_bar() -> String {
    bar(light::VERTICAL, light::VERTICAL, light::VERTICAL, " ")
}

pub fn bar(left: &str, right: &str, divider: &str, spacing: &str) -> String {
    let mut bar = "".to_string();
    for i in -12..=12 {
        bar.push(match i {
            -12    => left,
            12     => right,
            -4 | 4 => divider,
            _      => spacing
        }.chars().next().unwrap());
    }
    bar
}

pub fn col_number_offset(c: i32) -> i32 {
    c*2 + (c/3)*2 - 10
}

pub fn row_number_offset(r: i32) -> i32 {
    r + (r/3) - 5
}

pub fn time_top_bar() -> String {
    time_bar(light::DOWN_RIGHT, light::DOWN_LEFT, light::HORIZONTAL)
}

pub fn time_bot_bar() -> String {
    time_bar(light::UP_RIGHT, light::UP_LEFT, light::HORIZONTAL)
}

pub fn time_bet_bar() -> String {
    time_bar(light::VERTICAL, light::VERTICAL, " ")
}

pub fn time_bar(left: &str, right: &str, between: &str) -> String {
    let mut bar = "".to_string();
    for i in -3..=3 {
        bar.push(match i {
            -3    => left,
            3     => right,
            _      => between
        }.chars().next().unwrap());
    }
    bar
}
