mod lox;

use std::{env, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lox = lox::Lox::new();

    if args.len() == 1 {
        lox.run_prompt();
    } else if args.len() == 2 {
        lox.run_file(path::PathBuf::from(&args[1]));
    } else {
        println!("usage: lox [filename.lox]")
    }
}
