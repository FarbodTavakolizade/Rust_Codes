use std::io;
fn main(){
    println!("please provide a number: ");
    let mut input =String::new();
    io::stdin().read_line(&mut input).expect("error");

    let number:u32=input.trim().parse().expect("error");
    let factorial =calculate_factorial(number);
    println!("factorial of input number is {}",factorial);
}
fn calculate_factorial(n:u32) ->u32{
    if n==0 ||n==1{
        1
    }else {
        n*calculate_factorial(n-1)
    }
}
