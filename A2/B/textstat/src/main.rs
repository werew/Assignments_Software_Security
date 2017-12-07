use std::env;
use std::process;
use std::fs::File;
use std::error::Error;
use std::io::Read;
use std::io::BufReader;
use std::io::Bytes;
use std::io::Result;
use std::collections::hash_map::HashMap;


fn is_word_char(c: &char) -> bool {
    c.is_alphanumeric() || c.eq(&'\'')
}


fn read_word(it: &mut Bytes<BufReader<File>>) -> Option<Result<String>> {

    let mut is_reading = false;
    let mut w = String::with_capacity(100);

    loop {
        // Read next byte
        let r = match it.next() {
            Some(r) =>  r,
            None    =>  break, // EOF
        };

        // Handle IO errors
        if r.is_err() { 
            return Some(Err(r.unwrap_err()));
        }

        // Cast result to a char
        let c = r.unwrap() as char;

        // Skip unrelated chars
        if is_word_char(&c) == false && 
           is_reading       == false  { continue; }


        // Start reading the actual word
        is_reading = true;
        if is_word_char(&c) { w.push(c); }
        else { break; }
    }

    if w.is_empty() { None        } 
    else            { Some(Ok(w)) }
}



fn get_stats(buf: BufReader<File>){

    let mut hm = HashMap::new();

    let mut it = buf.bytes();
    loop {

        // Read one word 
        let r = match read_word(&mut it) {
            Some(r) => r,
            None    => break // EOF
        };
  

        // Handle read result
        match r {
            Ok(w)  => {
                // Successful read: increment counter
                let counter = hm.entry(w.to_lowercase())
                                .or_insert(0);
                *counter += 1;
            },

            Err(e) => {
                // Error: quit the application
                eprintln!("Cannot read file: {}",e.description());
                process::exit(1);
            }
        };


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
