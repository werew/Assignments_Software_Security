use std::env;
use std::process;
use std::fs::File;
use std::error::Error;
use std::io::Read;
use std::io::BufReader;
use std::io::Bytes;
use std::io::Result;
use std::collections::hash_map::HashMap;



/// Check whether a character has to be
/// considered as part of a word
/// @param c: a reference to the character to check
/// @return true if the char is part of a word
///     false otherwise
fn is_word_char(c: &char) -> bool {
    c.is_alphanumeric() || c.eq(&'\'')
}



/// Reads a single word
/// @param it: an iterator over the bytes to read
/// @return None if no words have been found
///     otherwise a Result instance containing
///     a string with the word read in case of 
///     success or an error otherwise
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


/// Counts the number of occurrences of each word
/// @param buf: a buffer reader
/// @return an HashMap mapping each word (as a String)
///     to the number of occurrences (as u64)
fn gen_wordcount(buf: BufReader<File>) -> HashMap<String,u64> {

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
                // note: we always lowercase the words
                //       so that we don't
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

    hm
}


// TODO meaningful naming
fn print_stats(count: HashMap<String, u64>){

    let mut total = 0;
    let mut total_diff = 0;
    let mut avg_size : f64 = 0.0;
    let mut count_length = HashMap::new();

    for (word, count) in count.iter() {

        // The division is distributed 
        // (avg_size*total + k.len()*v) / new_total
        let new_total = total + count;
        avg_size = (avg_size   as f64 / new_total as f64) *  total as f64 + 
                   (word.len() as f64 / new_total as f64) * *count as f64;

        total = new_total;
        total_diff += 1;

        let counter = count_length.entry(word.len())
                                  .or_insert(0);
        *counter += count;  // TODO Overflow ??

    }

    let mut count_vec: Vec<_> = count.iter().collect();
    count_vec.sort_by(|a,b| b.1.cmp(a.1));
    count_vec.truncate(10); 

    let mut count_len_vec: Vec<_> = count_length.iter().collect();
    count_len_vec.sort_by(|a,b| a.0.cmp(b.0));
    count_len_vec.truncate(10); 

    // TODO maybe improve the style (maybe in a table? check how to format )
    println!("############## STATS ################");
    println!("Total: {}",total);
    println!("Total differents: {}",total_diff);
    println!("Average size: {}",avg_size);

    println!("######### COUNT BY LENGTH ###########");
    for &(l,c) in &count_len_vec { 
        println!("Words of {} characters: {}",l,c); 
    }

    println!("######### TOP 10 MOST USED ###########");
    for &(w,c) in &count_vec { 
        println!("{} (used {} times)",w,c); 
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
    let count = gen_wordcount(buf);
    print_stats(count);
    
}
