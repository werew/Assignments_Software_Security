use std::env;
use std::process;
use std::fs::File;
use std::error::Error;
use std::io::Read;
use std::io::BufReader;
use std::io::Bytes;
use std::io::Result;
use std::collections::hash_map::HashMap;


const MAX_TOPUSAGE_LIST : usize = 10;  // Display ten most used words
const MAX_WLENGTH : usize = 10;        // Display count up to then chars


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



/// Prints some statistics about the text
/// provided an HashMap which contains the
/// words and the number of occurrences
/// @param words_count: an HashMap mapping each 
///     word to the number of occurrences
fn print_stats(words_count: HashMap<String, u64>){

    let mut total_words = 0;        // Amount of words in the text
    let mut total_differents = 0;   // Amount of different words
    let mut sum_sizes : usize = 0;  // Summation of words' sizes
    let mut count_by_length = HashMap::new();  // How many words for each length

    for (word, count) in words_count.iter() {

        let wlen = word.len();

        // Increment all general counters
        sum_sizes        += wlen * (*count as usize);
        total_words      += count;
        total_differents += 1;

        // Increment counter for this specific length
        // initializing counter at zero if this is the
        // first word of this length
        if wlen <= MAX_WLENGTH {
            let counter = count_by_length.entry(wlen)
                                         .or_insert(0); 
            *counter += count;
        }
    }


    // Calculate average size
    let avg_size = sum_sizes as f64 / total_words as f64;

    // List of pairs (length, count) sorted by length
    let mut list_by_length: Vec<_> = count_by_length.iter().collect();
    list_by_length.sort_by(|a,b| a.0.cmp(b.0));

    // List of pairs (word, usage) sorted by usage
    let mut list_by_usage: Vec<_> = words_count.iter().collect();
    list_by_usage.sort_by(|a,b| b.1.cmp(a.1));
    list_by_usage.truncate(MAX_TOPUSAGE_LIST);     


    /************ Display statistics **************/

    println!("############## STATS ################");
    println!("Total: {}",total_words);
    println!("Total differents: {}",total_differents);
    println!("Average size: {}",avg_size);

    println!("######### COUNT BY LENGTH ###########");
    for &(l,c) in &list_by_length{ 
        println!("Words of {} characters: {}",l,c); 
    }

    println!("######### TOP {} MOST USED ###########",MAX_TOPUSAGE_LIST);
    for &(w,c) in &list_by_usage{ 
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
