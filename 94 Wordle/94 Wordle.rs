/* cargo.toml file should be like this
[dependencies]
colored ="3.0.0" 
*/
use colored::{self,Colorize};
use std::io::{self,Write};
fn main() {

    println!("{}","Welcome To GuessGame  please provide your characters in consecutive order ".on_bright_red());

    let word ="hello";

    let input= io::stdin();

    for _ in 1..=6{
        let mut user_input =String::new();

        input.read_line(&mut user_input).expect("Failed");

        for (word_character , user_character) in word.chars().zip(user_input.trim().chars().take(5)){

            if word_character==user_character{
                println!("{}|",format!("{user_character}").on_green());
            }else if word.contains(user_character) {
                println!("{}|",format!("{user_character}").on_yellow());
            }else {
                println!("{}|",format!("{user_character}").on_bright_black());
            }

            io::stdout().flush().unwrap();
        }
        println!();

        if word==user_input.trim(){
            println!("you guessed the correct word . the word is {}",word.on_red());
            break;
        }

    }
    
}