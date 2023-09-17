// FIXME: remove all panic!("error") in lib.rs, replace with "return None;"

use bitwister::{
    evaluate,
    show_help
};

use std::env;

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

    let mut args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        logo();
        eprintln!("bitwister: not enough args, run [bitwister h] for help");
        std::process::exit(1);
    }

    if args[1] == "h" {
        logo();
        show_help();
        std::process::exit(1);
    }

    args.remove(0);

    let expr = args.join(" ");

    let result = evaluate(&expr);
    if result.is_some() {
        // SAFETY: checked above
        let (num, overflow) = result.unwrap();
        println!("[expr]> {expr}");
        println!("[eval]> {} {}", num, overflow);
    } else {
        println!("> failed to evaluate expression: {expr}");
    }
    
}