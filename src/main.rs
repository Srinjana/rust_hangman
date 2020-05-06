//generate random numbers
extern crate rand;
use rand::Rng;

//imported for file handling
use std::fs::File;
use std::io::prelude::*;

//imported code for user input
use std::io;

const ALLOWED_ATTEMPTS: u8 = 10;

struct Letter
{
    character: char,
    revealed: bool
}

enum GameProgress {
    InProgress,
    Won,
    Lost
}

fn select_word() -> String
{
//open file
    let mut file = File::open("words.txt").expect(":/ File not Found!");

//load contents into var
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
    .expect("An error occured while reading file.");

    // get individual words
    let available_words: Vec<&str> = file_contents.trim().split(',').collect();

    //generate random index
    let random_index = rand::thread_rng().gen_range(0, available_words.len());

    return String::from(available_words[random_index]);  //returns the string literal as return type
}


fn create_letters(word: &String) -> Vec<Letter>
{
    //Create empty vector
    let mut letters: Vec<Letter> = Vec::new();
    
    //wrapping chars in letters struct
    for c in word.chars(){
        letters.push(Letter {
            character: c,
            revealed: false
        });
    }

    return letters;
}


fn main() 
{
    let mut turns_left = ALLOWED_ATTEMPTS;
    let selected_word = select_word();
    let mut letters = create_letters(&selected_word);

    println!("\n");
    println!("----Welcome to the Game of Country Hangman.----\n");

    loop {
            println!("You have {} turns left.", turns_left);
            display_progress(&letters);

            println!("Enter a letter to make a guess: ");
            let user_char = read_user_input_character();

            //exit if an * is returned.
            if user_char == '*' {
                break;
            }
         
            //after each round update the revealed state, if no match then the user loses one turn, else the correct alphabets are revealed.

            let mut atleast_one_letter_revealed = false;

            for letter in letters.iter_mut() {
                if letter.character == user_char {
                    letter.revealed = true;
                    atleast_one_letter_revealed = true;
                }
            }

            //lose turn for wrong guess 

            if !atleast_one_letter_revealed {
                turns_left -= 1;
            }

            match check_progress(turns_left, &letters)
            {

                GameProgress::InProgress => continue,
                GameProgress::Won => {
                println!("\nCongratulations!!!! You won :D ");
                println!("\nThe word was: {}", selected_word);
                break;
                }
                GameProgress::Lost => {
                println!("\nSorry! You lost :'( ");
                println!("\nThe correct word was: {}", selected_word);
                break;
                }
            }
        }
    
    println!("\n ----Thank You for Playing. Goodbye!!!---- ");
   
}


//display progress of the game based in vector(Letter) eg: a _ b _ g
fn display_progress(letters: &Vec<Letter>)
{
    let mut display_string = String::from("Progress");

    for letter in letters 
    {
        display_string.push(' ');

        if letter.revealed{
            display_string.push(letter.character);
        }
        else{
            display_string.push('_');
        }

        display_string.push(' ');

    }

    println!("{}", display_string);
}


///read input from user. if multiple chars are given only the first index is taken.
fn read_user_input_character() -> char
{
    //get user input
    let mut user_input = String::new();
    match io::stdin().read_line(&mut user_input)  //if this is ok we match user input 
    {
        Ok(_) => {
            match user_input.chars().next() {
                Some(c) => { return c; }   //everything successful
                None => { return '*'; }    //getting next fails 
            }
        }

        Err(_) => { return '*'; }   //if anything goes wrong return *
    }

}


fn check_progress(turns_left: u8, letters: &Vec<Letter>) -> GameProgress
{
    //see if all letters have been revealed
    let mut all_revealed = true;
    for letter in letters{

        if !letter.revealed {
            all_revealed = false;
        }
    }

    if all_revealed 
    {
    return GameProgress::Won;
    }

    //game not over more turns left.
    if turns_left > 0 
    {
        return GameProgress::InProgress;
    }

    return GameProgress::Lost;
}
