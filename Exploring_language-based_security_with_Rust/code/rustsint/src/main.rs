mod sortedcontainer;

use sortedcontainer::SortedContainer;
use std::io::{self, Write};
use std::cmp::Ordering;
use std::fmt;



#[derive(Debug)]
enum Command {
    Insert{age: u32, name: String},
    Erase{age: u32, name: String},
    Contains{age: u32, name: String},
    Print,
    Exit,
    Error(String)
}


#[derive(Debug)]
struct Data {
   age: u32,
   name: String,
}


// This is required in order to implement the trait PartialOrd
impl PartialEq for Data {
    fn eq(&self, other: &Data) -> bool {
        self.age  == other.age &&
        self.name == other.name
    }
}


// This is required in order to use the sortedcontainer module
impl PartialOrd for Data {
    fn partial_cmp(&self,other: &Data) -> Option<Ordering> {
        Some(
            self.age.cmp(&other.age).then(
                self.name.cmp(&other.name)
            )
        )
    }
}


// This is required in order to use the sortedcontainer module
// since the data needs to be displayed when using the 'p' command
impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Age: {}, Name: {}", self.age, self.name)
    }
}







fn parse_command(input: String) -> Command {
    let command_items: Vec<&str> = input.split_whitespace().collect();

    // Fix: check if the input is empty first, so we don't panic otherwise
    if command_items.is_empty() { 
        return Command::Error("please insert a command".to_string());
    }

    match (command_items[0], command_items.len()) { 
        ("p", 1) => Command::Print,
        ("x", 1) => Command::Exit,
        ("i", 3) => {
            if let Ok(age) = command_items[1].parse::<u32>() {
                Command::Insert{age: age, name: command_items[2].to_string()}
            } else {
                Command::Error("unable to parse int (age).".to_string())
            }
        },
        ("e", 3) => {
            if let Ok(age) = command_items[1].parse::<u32>() {
                Command::Erase{age: age, name: command_items[2].to_string()}
            } else {
                Command::Error("unable to parse int (age).".to_string())
            }
        },
        ("c", 3) => {
            if let Ok(age) = command_items[1].parse::<u32>() {
                Command::Contains{age: age, name: command_items[2].to_string()}
            } else {
                Command::Error("unable to parse int (age).".to_string())
            }
        },

        (_, _) => Command::Error("invalid command.".to_string())
    }
}


fn main() {

    let mut sc: SortedContainer<Data> = SortedContainer::new();

    loop {
        let mut input = String::new();

        print!("> ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match parse_command(input) {
                    Command::Insert{age, name} => {
                        sc.insert(Data {age: age, name: name});
                    },
                    Command::Erase{age, name} => {
                        sc.erase(Data {age: age, name: name});
                    },
                    Command::Contains{age, name} => {
                        println!("{}",sc.contains(Data {age: age, name: name}));
                    },
                    Command::Print => {
                        sc.print();
                    },
                    Command::Exit => {
                        println!("Exiting...");
                        break;
                    },
                    Command::Error(error) => {
                        println!("Error: {}", error);
                    }
                }
            }
            Err(error) => println!("Error: {}", error),
        }
    }
}
