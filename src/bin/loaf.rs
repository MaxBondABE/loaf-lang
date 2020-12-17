use std::{env, fs::read_to_string, process::exit};

use loaf_lang::lang::parse::parse;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2{
        eprintln!("loaf <file.loaf>");
        exit(1);
    }
    let filename = &args[1];
    let code = read_to_string(filename).expect("Filename should be valid");
    let program_builder = parse(&code);
    if let Err(e) = program_builder {
        eprintln!("{:?}", e);
        exit(1);
    }
    let mut program = program_builder.unwrap().build();
    program.run(4);
}
