use crate::mod_files::mod_test::MessageMod;
use rand::Rng;
use std::cmp::Ordering;
use std::{io, process::exit};

pub mod mod_files;

fn guess_number_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn ownership_tuto() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here
    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
    let s1: String = String::from("coucou");

    let mut len = calculate_length(&s1);

    println!("The lenght of '{}' is {}", s1, len);

    let mut s2 = String::from("hello");
    len = calculate_length(&s2);

    println!("The lenght of '{}' is {}", s2, len);

    change(&mut s2);
    len = calculate_length(&s2);

    println!("The lenght of '{}' is {}", s2, len);
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

#[derive(Debug)]
struct Message {
    id: u32,
    content: String,
}

impl Message {
    /// Constructor of Message
    fn new(id: u32, content: String) -> Message {
        Self { id, content }
    }

    fn to_string(&self) -> String {
        format!("Message {{ id: {}, content: {} }}", self.id, self.content)
    }
}

fn test_struct() {
    // Create a new object Message like in object oriented programming
    let mut message = Message::new(1, String::from("Hello world!"));
    println!("Message: {}", message.to_string());
    message.content = String::from("Hello world 2!");
    println!("Message: {}", message.to_string());
}

fn test_mod() {
    let message: MessageMod = MessageMod::new(String::from("moi"), String::from("coucou"));
    println!("message ==> {}", message.to_string());
    println!("author: {}", message.author)
}

fn main() {
    loop {
        println!("<=================== Menu ====================>");
        println!("1. Guess number game");
        println!("2. Ownership tutorial");
        println!("3. Test of struct");
        println!("4. Test of mod");
        println!("99. Exit");
        println!("Choose function to run:");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Please type a number!");
        match choice {
            1 => {
                println!("<=================== Guess number game Z====================>");
                guess_number_game();
            }
            2 => {
                println!("<=================== Ownership ====================>");
                ownership_tuto();
            }
            3 => {
                println!("<=================== Test of struct ====================>");
                test_struct();
            }
            4 => {
                println!("<=================== Test of mod ====================>");
                test_mod();
            }
            99 => {
                exit(0);
            }
            _ => println!("Invalid choice"),
        }
    }
}
