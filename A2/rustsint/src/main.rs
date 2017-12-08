mod sortedcontainer;

use sortedcontainer::SortedContainer;
use std::io::{self, Write};
use std::cmp::Ordering;



#[derive(Debug)]
enum Command {
    Insert{age: i32, name: String},
    Erase{age: i32, name: String},
    Contains{age: i32, name: String},
    Print,
    Exit,
    Error(String)
}


struct Data {
   age: u32,
   name: String,
}

impl PartialEq for Data {
    fn eq(&self, other: &Data) -> bool {
        self.age  == other.age &&
        self.name == other.name
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self,other: &Data) -> Option<Ordering> {
        Some(
            self.age.cmp(&other.age).then(
                self.name.cmp(&other.name)
            )
        )
    }
}







fn parse_command(input: String) -> Command {
    let command_items: Vec<&str> = input.split_whitespace().collect();
    match (command_items[0], command_items.len()) {
        ("p", 1) => Command::Print,
        ("x", 1) => Command::Exit,
        ("i", 3) => {
            if let Ok(age) = command_items[1].parse::<i32>() {
                Command::Insert{age: age, name: command_items[2].to_string()}
            } else {
                Command::Error("unable to parse int (age).".to_string())
            }
        },
        ("e", 3) => {
            if let Ok(age) = command_items[1].parse::<i32>() {
                Command::Erase{age: age, name: command_items[2].to_string()}
            } else {
                Command::Error("unable to parse int (age).".to_string())
            }
        },
        ("c", 3) => {
            if let Ok(age) = command_items[1].parse::<i32>() {
                Command::Contains{age: age, name: command_items[2].to_string()}
            } else {
                Command::Error("unable to parse int (age).".to_string())
            }
        },

        (_, _) => Command::Error("invalid command.".to_string())
    }
}

fn main() {

    let sc: SortedContainer<Data> = SortedContainer::new();

    loop {
        let mut input = String::new();

        print!("> ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match parse_command(input) {
                    Command::Insert{age, name} => {
                        unimplemented!();
                    },
                    Command::Erase{age, name} => {
                        unimplemented!();
                    },
                    Command::Contains{age, name} => {
                        unimplemented!();
                                        }
                    Command::Print => {
                        unimplemented!();
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
