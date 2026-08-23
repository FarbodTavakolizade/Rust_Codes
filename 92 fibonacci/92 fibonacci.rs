use std::io;

fn main(){
    println!("please provide a number: ");

    let mut input =String::new();
    io::stdin().read_line(&mut input).expect("error");

    let count :u32=input.trim().parse().expect("error");

        println!("number of fibonacci sequence: {count}");

        fibonacci(count);

}

fn fibonacci(n : u32){
    let mut a =0;
    let mut b =1;

    for i in 0..n{
        if i<=1{
            println!("T{}:{}",i+1,i);
        }else {
            
            let x=a+b;
            println!("T{}:{}",i+1,x);
            a=b;
            b=x;
        }
    }



}
