use std::io;

fn main(){
    println!("please provide a number: ");

    let mut input=String::new();

    io::stdin().read_line(&mut input).expect("eror");

    let limit :u32=input.trim().parse().expect("enter valid number");

    println!("prime number up to {}:",limit);

    for num in 2..=limit{
        let mut is_prime=true;

        for i in 2..num{
            if num % i==0{
                is_prime=false;
                break;
            }
        }
        if is_prime{
            println!("{}",num);
        }

    }

}









