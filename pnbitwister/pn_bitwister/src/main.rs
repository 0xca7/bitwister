use rustyline::{
    DefaultEditor, 
    Result,
    error::ReadlineError,
};

use pncalc::{
    Tokenizer,
    Calculation,
    CalculationResult, 
    Bits
};

fn eval(line: &str) -> Option<CalculationResult> {

    let expr: Vec<&str> = line
        .split(' ')
        .collect();

    let (mut calc, expr) = match expr[0] {
        "u8" => (Calculation::new(Bits::U8), line.strip_prefix("u8 ")
            .unwrap()),
        "u16" => (Calculation::new(Bits::U16), line.strip_prefix("u16 ")
            .unwrap()),
        "u32" => (Calculation::new(Bits::U32), line.strip_prefix("u32 ")
            .unwrap()),
        "u64" => (Calculation::new(Bits::U64), line.strip_prefix("u64 ")
            .unwrap()),
        _ => (Calculation::new(Bits::U32), line)
    };

    let tok = Tokenizer::new();
    let tokens = tok.tokenize(expr);

    match tokens {
        Some(mut toks) => calc.calculate(&mut toks),
        None => None,
    }

}

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

fn usage() {
    println!("[examples]");
    println!("+ 2 2 == 2 + 2");
    println!("* 2 + 2 2 == 2*(2+2)\n\n");
}

fn main() -> Result<()> {

    logo();
    usage();

    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline("⮕  ");
        match readline {
            Ok(line) => {
                match rl.add_history_entry(line.as_str()) {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("[bt]> error adding to history: {e}");
                    },
                }
                println!("⚙ evaluating... { }", line);
                match eval(&line) {
                    Some(value) => println!("✓ {}", value),
                    None => println!("✗ error in expression"),
                }
            },
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }

    println!(">> goodbye! see you next time :^)\n");
    Ok(())
}
