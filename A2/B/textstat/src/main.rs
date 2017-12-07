use std::env;
use std::process;
use std::fs::File;
use std::error::Error;
use std::io::Read;
use std::io::BufReader;

fn get_stats(buf: BufReader<File>){

    for r in buf.bytes() {
        let b = r.unwrap() as char;
        //println!("Byte: {}",std::str::from_utf8(&[b]).unwrap());
        println!("Byte: {}",b.is_uppercase());

    }

}

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

   
    let buf = BufReader::new(f);
    get_stats(buf);
}
