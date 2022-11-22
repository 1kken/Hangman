use console::Style;
use console::Term;
use rand::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

///////////////////////////////
////////LAZY STATIC///////////
/////////////////////////////

lazy_static! {
    static ref TERM: console::Term = Term::stdout();
    static ref RED: console::Style = Style::new().red();
    static ref GREEN: console::Style = Style::new().green();
}

///////////////////////////////////
//////GET WORD FROM THE FILE//////
/////////////////////////////////
fn get_word(path: &str) -> String {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut rng = thread_rng();
    reader.lines().map(|d| d.unwrap()).choose(&mut rng).unwrap()
}

////////////////////////////////////
////////GET CHAR INPUT/////////////
//////////////////////////////////
fn get_input() -> char {
    let mut res: char;
    loop {
        res = TERM.read_char().unwrap();
        match res {
            'A'..='Z' | 'a'..='z' => break,
            _ => continue,
        }
    }
    TERM.clear_screen().unwrap();
    res
}

//Find uniques//
    fn find_uniques(st: Vec<char>) -> i8 {
        let mut state = st;
        state.sort_unstable();
        state.dedup();
        state.len() as i8
    }

enum GameState {
    Win,
    Alive,
    Dead,
}
struct State {
    guess_remaining: i8,
    to_guess: String, // Random english string
    uniques: i8,
    guessed: Vec<char>, // charrified string BAABHAHAHAHAHAHAH *charrified*
}

impl State {
    fn reveal(&self, right_choice: &Vec<char>) {
        for ch in self.guessed.clone() {
            if right_choice.contains(&ch) {
                print!("{} ", ch);
            } else {
                print!("_ ");
            }
        }
    }

    fn update(&mut self, right_choice: &Vec<char>) -> GameState {
        self.guess_remaining -= 1;
        if self.guess_remaining < 1 {
            GameState::Dead
        } else {
            if self.uniques == right_choice.len() as i8 {
                GameState::Win
            } else {
                GameState::Alive
            }
        }
    }
}

///////////////////////////////
///*****SINGLE PLAYER******////
///////////////////////////////
pub mod sngl {
    use super::*;
    pub fn sng_main() {
        let mut state = init();
        game_start(&mut state);
    }

    fn init() -> State {
        let to_guess = get_word("resources/source.txt").trim().to_string();
        let guessed: Vec<char> = to_guess.chars().collect();
        let uniques = find_uniques(guessed.clone());
        let state = State {
            guess_remaining: 7,
            to_guess,
            uniques,
            guessed,
        };

        state
    }


    fn game_start(state: &mut State) {
        let mut letters: Vec<char> = ('a'..='z').collect();
        let mut right_guess: Vec<char> = Vec::new();
        //game loop
        loop {
            //print lives remaining
            println!("LIVES REMAINING:{}", RED.apply_to(state.guess_remaining));
            //print selection
            println!("LETTERS TO SELECT");
            for letter in &letters {
                print!("{} ", letter.to_uppercase());
            }
            //print guessed word
            println!("\n");
            print!("WORD TO GUESS\n");
            state.reveal(&right_guess);

            //get input either add it to wrong or right
            println!("\n");
            println!("CHOOSE FROM A_Z");
            let inp = get_input();
            if state.to_guess.contains(inp) {
                right_guess.push(inp);
                if let Ok(index) = letters.binary_search(&inp) {
                    letters.remove(index);
                } else {
                    continue;
                }
            } else {
                if let Ok(index) = letters.binary_search(&inp) {
                    letters.remove(index);
                } else {
                    continue;
                }
            }

            //Check if the player still have a chance or not
            match state.update(&right_guess) {
                GameState::Win => {
                    println!("{}",GREEN.apply_to("YOU WIN CONGRATS"));
                    break;
                }
                GameState::Alive => continue,
                GameState::Dead => {
                    println!("{}",RED.apply_to("THE MAN IS DEAD"));
                    break;
                }
            }
        }
    }
}

pub mod mult {
    use super::*;
    use whatlang::{Detector, Lang};
    /// ENTRY POINT FOR OUR MULTIPLAYER GAME///
    pub fn mult_main() {
        let mut state: State = init();
        game_start(&mut state);
    }

    fn init() -> State {
        let word: String = get_word();
        let guessed: Vec<char> = word.chars().collect();
        let uniques = find_uniques(guessed.clone());
        let state = State {
            guess_remaining: 7,
            to_guess:word,
            uniques,
            guessed,
        };
        state
    }

    /// GET THE WORD TO BE GUESSED//
    fn get_word() -> String {
        let mut word: String = String::new();
        loop {
            println!("MR.WHITE PLEASE PUT A WORD TO BE GUESS");
            match TERM.read_line() {
                Ok(data) if data.len() > 3 && lang_check(&data) => {
                    word = data;
                    break;
                }
                Ok(_) => {
                    word.clear();
                },
                Err(_) => {
                    word.clear();
                },
            }
        }
        TERM.clear_screen().unwrap();
        word
    }

    //CHECK THE LANGUAGE IF ITS ENGLISH
    fn lang_check(word: &str) -> bool {
        let detector: Detector = Detector::with_allowlist(vec![Lang::Eng]);
        match detector.detect_lang(word) {
            Some(data) if data == Lang::Eng => true,
            Some(_) => false,
            None => false,
        }
    }

    //GAME LOOP//
    fn game_start(state: &mut State) {
        let mut letters: Vec<char> = ('a'..='z').collect();
        let mut right_guess: Vec<char> = Vec::new();
        //game loop
        loop {
            //print lives remaining
            println!("LIVES REMAINING:{}", RED.apply_to(state.guess_remaining));
            //print selection
            println!("LETTERS TO SELECT");
            for letter in &letters {
                print!("{} ", letter.to_uppercase());
            }
            //print guessed word
            println!("\n");
            print!("WORD TO GUESS\n");
            state.reveal(&right_guess);

            //get input either add it to wrong or right
            println!("\n");
            println!("CHOOSE FROM A_Z");
            let inp = get_input();
            if state.to_guess.contains(inp) {
                right_guess.push(inp);
                if let Ok(index) = letters.binary_search(&inp) {
                    letters.remove(index);
                } else {
                    continue;
                }
            } else {
                if let Ok(index) = letters.binary_search(&inp) {
                    letters.remove(index);
                } else {
                    continue;
                }
            }

            //Check if the player still have a chance or not
            match state.update(&right_guess) {
                GameState::Win => {
                    println!("{}",GREEN.apply_to("YOU WIN CONGRATS"));
                    break;
                }
                GameState::Alive => continue,
                GameState::Dead => {
                    println!("{}",RED.apply_to("THE MAN IS DEAD"));
                    break;
                }
            }
        }
    }
}
