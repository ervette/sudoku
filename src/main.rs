use std::{io, cmp, thread, time};
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use crossterm::{execute, queue, event::KeyCode, cursor, style, ErrorKind};
use style::Attribute::{Underlined, NoUnderline};
use rand::seq::SliceRandom;

mod util;
mod difficulty;
mod ui;
mod sudoku;
mod value;

use util::*;
use difficulty::*;
use sudoku::Sudoku;
use value::SudokuValue;
use std::collections::HashSet;

const MENU: &str = r#"
        Welcome to the Sudoku Game in Rust!
                by Talalaiko Kiril 
                    <40618094>
                    
                      q : quit
                 up / w : up
               left / a : left
               down / s : down
              right / d : right
                    0-9 : 0-9
                      c : check
                      t : tip
                  Enter : Select
                 Escape : Main Menu

                 Select Difficulty
"#;
const CHECK_KEY: char = 'c';
const HINTS_KEY: char = 't';

mod color {
    use crossterm::style;
    pub const GIVEN_NUMBER: style::Color    = style::Color::Blue;
    pub const INSERTED_NUMBER: style::Color = style::Color::Yellow;
    pub const WRONG_NUMBER: style::Color    = style::Color::Red;
}

fn main() -> Result<(), ErrorKind> {
    let stdout = Arc::new(Mutex::new(io::stdout()));
    run(stdout)
}

fn run<W: 'static + io::Write + Send>(w: Arc<Mutex<W>>) -> Result<(), ErrorKind> {
    // setup
    crossterm::terminal::enable_raw_mode()?;
    queue!(w.lock().unwrap(), crossterm::terminal::EnterAlternateScreen, cursor::Hide)?;
    let mut selected = Difficulty::Medium;
    let size = crossterm::terminal::size()?;
    let term = Arc::new(Mutex::new(ui::Terminal::new(size.0, size.1)));
    loop {
        let size = crossterm::terminal::size()?;
        let mut t_lock = term.lock().unwrap();
        t_lock.set_size(size.0, size.1);
        let mut w_lock = w.lock().unwrap();
        queue!(w_lock, crossterm::terminal::Clear(crossterm::terminal::ClearType::All))?;
        for (n, line) in MENU.split('\n').enumerate() {
            queue!(w_lock, style::Print(line), style::Print('\n'),cursor::MoveTo(t_lock.h_center()-25, t_lock.v_center_str(MENU)+n as u16))?;
        }
        queue!(w_lock, cursor::MoveTo(t_lock.h_center_str(&Difficulty::Hard.to_string()), t_lock.v_center() + 7), style::Print(
            format!("{}{}{}", match selected {Difficulty::Hard => Underlined, _ => NoUnderline} , Difficulty::Hard, NoUnderline)),
                  cursor::MoveTo(t_lock.h_center_str(&Difficulty::Medium.to_string()), t_lock.v_center() + 8), style::Print(
            format!("{}{}{}", match selected {Difficulty::Medium => Underlined, _ => NoUnderline} , Difficulty::Medium, NoUnderline)),
                  cursor::MoveTo(t_lock.h_center_str(&Difficulty::Easy.to_string()), t_lock.v_center() + 9), style::Print(
            format!("{}{}{}", match selected {Difficulty::Easy => Underlined, _ => NoUnderline} , Difficulty::Easy, NoUnderline)))?;
        w_lock.flush()?;
        drop(w_lock);
        drop(t_lock);

        let key_code = read_key_code()?;
        if key_code == KeyCode::Char('q') {
            break;
        } else if is_up(key_code) {
            selected = selected.up();
        } else if is_down(key_code) {
            selected = selected.down();
        } else if key_code == KeyCode::Enter {
            let w = Arc::clone(&w);
            let term = Arc::clone(&term);
            game(w, term, selected)?;
        }
    };

    // end
    crossterm::terminal::disable_raw_mode()?;
    let (_, rows) = crossterm::terminal::size()?;
    execute!(w.lock().unwrap(), style::ResetColor, cursor::Show, cursor::MoveTo(0, rows))?;
    Ok(())
}

fn game<W: 'static + io::Write + Send>(w: Arc<Mutex<W>>, term: Arc<Mutex<ui::Terminal>>, diff: Difficulty) -> Result<(), ErrorKind> {
    // setup
    // clear
    let mut w_lock = w.lock().unwrap();
    queue!(w_lock, cursor::Hide, crossterm::terminal::Clear(crossterm::terminal::ClearType::All), cursor::MoveTo(0, 0))?;

    // draw Sudoku Lines
    draw_sudoku_lines(&mut *w_lock, &*term.lock().unwrap())?;
    drop(w_lock);

    // create Sudokus
    let (given, solution) = Sudoku::new(diff as usize);
    let mut current = given;

    let mut selected = (4, 4);
    let mut changed = true;
    let mut win = false;
    let mut wrong_values: HashSet<(usize, usize)> = HashSet::new();

    // timer
    let timer_stop = Arc::new(AtomicBool::new(false));
    let timer = Arc::new(time::Instant::now());

    // timer thread
    let timer_handle = {
        let timer_stop = Arc::clone(&timer_stop);
        let timer = Arc::clone(&timer);
        let w = Arc::clone(&w);
        let term = Arc::clone(&term);
        thread::spawn(move || {
            let mut last_time = std::u64::MAX;
            while timer_stop.load(Ordering::SeqCst) == false {
                let now = timer.elapsed().as_secs();
                if last_time != now {
                    last_time = now;
                    // draw new time
                    let mut w_lock = w.lock().unwrap();
                    let t_lock = term.lock().unwrap();
                    draw_time(&mut *w_lock, &*t_lock, now);
                    drop(w_lock);
                    drop(t_lock);
                    thread::sleep(time::Duration::from_millis(990));
                } else {
                    thread::sleep(time::Duration::from_millis(1));
                }
            }
        })
    };

    // game loop
    loop {
        let new_size = crossterm::terminal::size()?;
        let mut t_lock = term.lock().unwrap();
        if t_lock.set_size(new_size.0, new_size.1) {
            changed = true;
            // redraw lines
            let mut w_lock = w.lock().unwrap();
            queue!(w_lock, crossterm::terminal::Clear(crossterm::terminal::ClearType::All))?;
            draw_sudoku_lines(&mut *w_lock, &*t_lock)?;
            draw_time(&mut *w_lock, &* t_lock, timer.elapsed().as_secs());
        }
        if changed {
            let mut w_lock = w.lock().unwrap();
            queue!(w_lock, cursor::MoveTo(t_lock.h_center()+13, t_lock.v_center()-5), style::Print(time_top_bar()))?;
            queue!(w_lock, cursor::MoveTo(t_lock.h_center()+13, t_lock.v_center()-4), style::Print(time_bet_bar()))?;
            queue!(w_lock, cursor::MoveTo(t_lock.h_center()+13, t_lock.v_center()-3), style::Print(time_bot_bar()))?;
            if current == solution {
                win = true;
                timer_stop.store(true, Ordering::SeqCst);
                queue!(w_lock, cursor::MoveTo(t_lock.h_center()+14, t_lock.v_center()-4), style::Print("Done!"))?;
            } else {
                let count = current.count(SudokuValue::Empty);
                queue!(w_lock, cursor::MoveTo(t_lock.h_center()+15, t_lock.v_center()-4), style::Print(format!("{:2}", count)))?;
            }
            draw_sudoku_values(&mut *w_lock, &*t_lock, &current, &given, selected, win, &wrong_values)?;
            changed = false;
        }
        drop(t_lock);

        let key_code = read_key_code()?;
        let typed_sudoku_value = key_code_to_sudoku_value(key_code);
        if key_code == KeyCode::Esc || key_code == KeyCode::Char('q') {
            timer_stop.store(true, Ordering::SeqCst);
            timer_handle.join().unwrap();
            break;
        } else if is_up(key_code) {
            selected.0 = selected.0.saturating_sub(1);
            changed = true;
        } else if is_left(key_code) {
            selected.1 = selected.1.saturating_sub(1);
            changed = true;
        } else if is_down(key_code) {
            selected.0 = cmp::min(selected.0 + 1, 8);
            changed = true;
        } else if is_right(key_code) {
            selected.1 = cmp::min(selected.1 + 1, 8);
            changed = true;
        } else if typed_sudoku_value != None && given.get(selected.0, selected.1) == Some(&SudokuValue::Empty) {
            current.set(selected.0, selected.1, typed_sudoku_value.unwrap());
            wrong_values.remove(&(selected.0, selected.1));
            changed = true;
        } else if key_code == KeyCode::Char(CHECK_KEY) {
            for r in 0..9 {
                for c in 0..9 {
                    if current.get(r, c) != Some(&SudokuValue::Empty) && current.get(r, c) != solution.get(r, c) {
                        if wrong_values.insert((r, c)) {
                            changed = true;
                        }
                    }
                }
            }
        } else if key_code == KeyCode::Char(HINTS_KEY) {
            let mut empty_values = Vec::new();
            for r in 0..9 {
                for c in 0..9 {
                    if current.get(r, c) == Some(&SudokuValue::Empty) {
                        empty_values.push((r, c));
                    }
                }
            }
            if let Some((chosen_r, chosen_c)) = empty_values.choose(&mut rand::thread_rng()) {
                let right_value = *solution.get(*chosen_r, *chosen_c).unwrap();
                current.set(*chosen_r, *chosen_c, right_value);
                selected = (*chosen_r, *chosen_c);
                changed = true;
            }
        }
    };

    // end
    execute!(&mut *w.lock().unwrap(), style::ResetColor, crossterm::terminal::Clear(crossterm::terminal::ClearType::All))?;
    Ok(())
}

fn draw_sudoku_lines<W: io::Write>(w: &mut W, term: &ui::Terminal) -> crossterm::Result<()>{
    for i in 0..=12 {
        let this_bar = match i {
            0 => top_bar(),
            12 => bot_bar(),
            4 | 8 => div_bar(),
            _ => num_bar()
        };
        queue!(w, cursor::MoveTo(term.h_center()-12, term.v_center()-6+i), style::Print(this_bar))?;
    }
    w.flush()?;
    Ok(())
}

fn draw_sudoku_values<W: io::Write>(w: &mut W, term: &ui::Terminal, sud: &Sudoku, given: &Sudoku, selected: (usize, usize), win: bool, wrong_values: &HashSet<(usize, usize)>) -> crossterm::Result<()> {
    for r in 0_usize..9 {
        for c in 0_usize..9 {
            queue!(w, cursor::MoveTo((term.h_center() as i32 + col_number_offset(c as i32)) as u16, (term.v_center() as i32 + row_number_offset(r as i32)) as u16))?;
            let sud_val = sud.get(r, c).unwrap();
            let underline = if !win && selected == (r, c) { Underlined } else { NoUnderline };
            let color = match (given.get(r, c), wrong_values.get(&(r, c))) {
                (_, Some(_))                    => color::WRONG_NUMBER,
                (Some(&SudokuValue::Empty), _)  => color::INSERTED_NUMBER,
                _                               => color::GIVEN_NUMBER
            };

            queue!(w, style::SetForegroundColor(color), style::Print(format!("{}{}{}", underline, sud_val, NoUnderline)), style::ResetColor)?;
        }
    }
    w.flush()?;
    Ok(())
}

pub fn draw_time<W: io::Write>(w: &mut W, term: &ui::Terminal, seconds: u64) {
    queue!(w, cursor::MoveTo(term.h_center() + 13, term.v_center()-1), style::Print(time_top_bar())).unwrap();
    queue!(w, cursor::MoveTo(term.h_center() + 13, term.v_center()),   style::Print(time_bet_bar())).unwrap();
    queue!(w, cursor::MoveTo(term.h_center() + 13, term.v_center()+1), style::Print(time_bot_bar())).unwrap();
    let mins = seconds / 60;
    let secs = seconds % 60;
    let timer_time = format!("{:02}:{:02}", mins, secs);
    queue!(w, cursor::MoveTo(term.h_center() + 14, term.v_center()), style::Print(timer_time)).unwrap();
    w.flush().unwrap();
}
