#[macro_use]
extern crate lazy_static;
mod dict;
use dict::{mult, sngl};
use std::io;
use console::Term;
fn main() {
    let term:console::Term = Term::stdout(); 
    println!("*****MAIN MENU******");
    println!("a) Start game");
    println!("b) Multiplayer Game");
    println!("c) Quit Game");
    println!("********************");

    let mut choice = get_input();
        match  &*choice {
            "a" => {
                println!("ENTER GAME LOOP");
                term.clear_screen().unwrap(); 
                sngl::sng_main();
            }
            "b" => {
                println!("ENTER MULTIPLAYER GAME");
                term.clear_screen().unwrap(); 
                mult::mult_main();
            }
            "c" => {
                println!("QUIT GAME");
            }
            _ => {
                println!("INVALID CHOICE");
            }
        }
        choice.clear();
}

fn get_input() -> String {
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => break,
            Err(_) => {
                input.clear();
                continue;
            }
        };
    }
    input.trim().to_string()
}
