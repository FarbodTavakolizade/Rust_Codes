use std::io;
fn main() { //control flow: if,else,else if
    /* 
    let number=4;
    if number>5{
        println!("number is greater than 5");
    }else if number<5{
        println!("number is lower  than 5");
    }else {
        println!("number is equal to 5");
    }
*/


let number2=6;
let result = if number2 >5 {"big"} else {"small"};  // boolean condition ? value1 :value 2
println!("{}",result);


let name ="reza".to_string();

if name=="reza"{
    println!("hi reza")
}else if name=="ali" {
    println!("hi ali")
}else{
    println!("i dont know you");
}
/* 
println!("please provide your name");


let mut input=String::new();
io::stdin().read_line(&mut input).expect("input error ");
let name2=input.trim();

if name2=="reza"{
    println!("hi reza")
}else if name2=="ali" {
    println!("hi ali")
}else{
    println!("i dont know you");
}
*/


let name3 ="alireza";

let final_result=if name3=="alireza"{
    "hi alireza"
}else if name3=="reza"  {
    "hi reza"
}else {
    "abcd"
};
println!("{final_result}");




    

}
