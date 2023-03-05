// TODO: implement left and right arrow keys for corrections
// TODO: fix the help screen

use bitwister::{
    evaluate,
    show_help
};

use termion::cursor::DetectCursorPos;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use std::io::{Write, stdout, stdin, Stdout};
use std::collections::VecDeque;


fn logo() {

    let logo = r#"

  ___ _ _____        _    _           
 | _ |_)_   _|_ __ _(_)__| |_ ___ _ _ 
 | _ \ | | | \ V  V / (_-<  _/ -_) '_|
 |___/_| |_|  \_/\_/|_/__/\__\___|_|  
 - the simple bit calculator for your
   bit twisting needs.

    "#;

    println!("{logo}");
}

const MAX_HISTORY: usize = 3;
const PROMPT: &'static str = "[bt]>";

fn save_history(history: &mut VecDeque<String>, line: &String) {
    if history.len() < MAX_HISTORY {
        history.push_front(line.clone());
    } else {
        history.pop_back();
        history.push_front(line.clone());
    }
}

fn clear(stdout: &mut RawTerminal<Stdout>) {

    let (_, y)= stdout.cursor_pos().unwrap();

    write!(stdout,
            "{}{}",
            termion::cursor::Goto(1, y),
            termion::clear::CurrentLine)
            .unwrap();
    stdout.flush().unwrap();
}

fn prompt() {
    print!("{} ", PROMPT);
}

fn input_loop() {

    logo();
    show_help();

    let mut line = String::new();
    let mut history: VecDeque<String> = VecDeque::new();
    let mut history_cursor = 0usize;

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    stdout.flush().unwrap();

    write!(stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::CurrentLine)
            .unwrap();

    prompt();

    for c in stdin.keys() {
            
        match c.unwrap() {
            Key::Char('\n') => {
                // check if there are any characters other than
                // the newline
                let (x, y) = stdout.cursor_pos().unwrap();
                if x == 0 && y == 1 {
                    continue;
                } else {
                    // there is something to process, so get it 
                    if !line.is_empty() {
                        save_history(&mut history, &line);
                        let result = evaluate(&line);
                        if result.is_some() {
                            // SAFETY: checked abovea
                            let (num, overflow) = result.unwrap();
                            clear(&mut stdout);
                            prompt();
                            println!("{line}");
                            clear(&mut stdout);
                            println!(">>> {} {}", num, overflow);
                        } else {
                            println!("[bt]> failed to evaluate expression: {line}");
                        }
                        line.clear();
                    }
                    clear(&mut stdout);
                    prompt();
                }
            },
            Key::Char('q') => break,
            Key::Char('?') => {
                show_help();
            },
            Key::Char(c) => {
                print!("{}", c);
                line.push(c);
            },
            // this enables a history in the application
            Key::Up => {
                if !history.is_empty() {
                    clear(&mut stdout);
                    prompt();
                    print!("{}", history[history_cursor]);
                    line = history[history_cursor].clone();
                    history_cursor = (history_cursor + 1) % history.len();
                }
            },
            Key::Backspace => {
                let (x, y)= stdout.cursor_pos().unwrap();
                line.pop();
                write!(stdout, "{}{}", " ", termion::cursor::Goto(x-1,y))
                    .unwrap();
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }
    
    clear(&mut stdout);
    write!(stdout, "{}", termion::cursor::Show).unwrap();
    println!("[bt]> quitting, see ya next time :^)");

}


fn main() {
    input_loop();
}