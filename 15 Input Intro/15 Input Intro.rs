use std::io;
fn main() {
    println!("please provide your age: ");
    let mut age =String::new();

    io::stdin()
        .read_line(&mut age)
        .expect("error");
    let age:u8= age.trim().parse().expect("please provide number");
    println!("your age is:{}",age);
} 
