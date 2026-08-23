//palindrome string ==>"aaabbaaa" or "aaabaaa"
use std::{io, num::ParseIntError};

/*
fn main(){

   println!("please provide a string: ");

   let mut input=String::new();

   io::stdin().read_line(&mut input).expect("error");

   let x: Vec<char> = input
   .trim()
   .to_lowercase()
   .chars()
   .filter(|c|c.is_alphanumeric())
   .collect();

   let mut is_palindrome=true;

   let len =x.len();
   for i in 0..len/2{

      if x[i] !=x[len - i- 1]{
         is_palindrome=false;
         break;
      }
   }
   if is_palindrome{
      println!("string is palindrome")
   }else {
      println!("string is not palindrome")
   }
}
   */

fn main(){
    println!("please provide a string: ");

   let mut input=String::new();

   io::stdin().read_line(&mut input).expect("error");

   let cleaned_string : String =input.trim()
   .to_lowercase()
   .chars()
   .filter(|c|c.is_alphanumeric())
   .collect();

   let reversed: String=cleaned_string.chars().rev().collect();

   if cleaned_string == reversed{
      println!("string is palindrome");
   }else {
       println!("string is not palindrome");
   }

}
