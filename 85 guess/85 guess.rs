use rand::Rng;
use std::io;
use std::io::Write;

fn main(){
    println!("welcome to the number Guessing Game!");
    let secret_number =rand::thread_rng().gen_range(1..=100);

    let mut guess=String::new();
    let mut guess_count=0;

    loop {
        guess_count+=1;
        print!("Guess the number: ");
        io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut guess).expect("faild to get input from client");

        let guess_num:i32 = match guess.trim().parse() 
        {
            Ok(num) =>num,
            Err(_)=>{
                println!("please provide a valid number: ");
                guess=String::new();
                continue;
            }
        };

        match guess_num.cmp(&secret_number) {
            std::cmp::Ordering::Less =>{
                println!("too small");
            }
            std::cmp::Ordering::Greater =>{
                println!("too big");
            }
            std::cmp::Ordering::Equal =>{
                println!("you guessed the right number in {} tries",guess_count);
                break;
            }
        }
        

        guess = String::new();
    }
}
