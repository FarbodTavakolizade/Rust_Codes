use std::io;

fn main(){
println!("please provide number: ");


let mut input=String::new();

io::stdin().read_line(&mut input).expect("failed to read input");

let number:u32=input.trim().parse().expect("please provide valid number");

let mut sum =0;

let mut temp =number;

let num_digits=number.to_string().len() as u32;


while temp >0{
    let digit =temp%10;

    sum+=digit.pow(num_digits);

    temp/=10;
}

if sum==number{
    println!("{} is an armstrong number",number);
}else {
    println!("number is not armstrong");
}




}









