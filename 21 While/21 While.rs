use std::io;
fn main() {//while

let mut counter=1;

while counter <=5 {
    println!("number is {counter}");
    counter+=1;
}
//----------------------------------------------
/* 
let mut number=0;

while number < 100 {
    number +=1;

    if number %2 != 0 {
        continue;
    }
    println!("number is {number}");


    if number >= 100 {
        break;
    }
*/

let mut x =0;
let mut y=0;
 
 while x<5 && y<5{

    println!("x is {x} and y is {y}");
    x+=1;
    y+=1;
 }

 let mut input =String::new();
 while input.trim()!="exit" {
    println!("enter sth:");
    input.clear();
    io::stdin().read_line(&mut input).expect("error in recieving input");
    input =input.trim().to_string();
    
 }

 println!("finished");




    
}





