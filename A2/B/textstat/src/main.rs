use std::env;
use std::process;
use std::fs::File;
use std::error::Error;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: {} <filename>",args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let f = match File::open(&filename) {
        Ok(f)  => f,
        Err(e) => {
            eprintln!("Cannot open file: {}", e.description());
            process::exit(1);
        }
    };

}
