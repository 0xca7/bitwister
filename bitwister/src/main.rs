use bitwister::evaluate;

use std::io::{self, Write};

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

fn main() {

    logo();

    loop {

        print!("[bt]> ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();

        match io::stdin().read_line(&mut buffer) {
            Ok(_s) => (),
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        } // match 

        let result = evaluate(&buffer);
        if result.is_some() {
            // SAFETY: checked above
            println!(">>>>> {}", result.unwrap());
        } else {
            println!("[bt]> failed to evaluate expression");
        }

    } // loop

}