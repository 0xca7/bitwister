/// repl for bitwister
/// 0xca7

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use bitwister::{
    evaluate
};

pub fn repl() -> Result<()> {

    let mut rl = DefaultEditor::new()?;
    #[cfg(feature = "with-file-history")]
    if rl.load_history("/tmp/history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline("[bt]> ");
        match readline {
            Ok(line) => {
                match rl.add_history_entry(line.as_str()) {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("[bt]> error adding to history: {e}");
                    },
                }
                let result = evaluate(&line);
                if result.is_some() {
                    // SAFETY: checked above
                    let (num, overflow) = result.unwrap();
                    println!("[expr]> {line}");
                    println!("[eval]> {} {}", num, overflow);
                } else {
                    println!("> failed to evaluate expression: {line}");
                }
            },
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("[bt]> shutting down, goodbye :^)");
                break
            },
            Err(err) => {
                println!("[bt]> error: {:?}", err);
                break
            }
        }
    } // loop
    #[cfg(feature = "with-file-history")]
    rl.save_history("/tmp/history.txt");
    Ok(())

}