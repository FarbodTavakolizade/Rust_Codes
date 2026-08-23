use std::io;

fn main(){
    println!("please provide a number: ");

    let mut input=String::new();

    io::stdin().read_line(&mut input).expect("eror");

    let limit :u32=input.trim().parse().expect("enter valid number");

    println!("prime number up to {}:",limit);


    if limit>=2{
        println!("2");
    }

    for num in (3..=limit).step_by(2){
        let mut is_prime=true;
        let sqrt=(num as f64).sqrt() as u32;

        for i in (3..=sqrt).step_by(2){
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









