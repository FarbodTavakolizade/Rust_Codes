//match statement :switch case
fn main() { 
    let number =30;


    match number {
        1..10 => println!("one"),
        11..20 =>println!("two"),
        21..31 =>println!("three"),
        _ =>(), // println!("string"),
    }

    let alphabet = 'A';

   let result= match alphabet {
        'A'=> "A",
        'B'=> "B",
        _=>"another character",
    };
    println!("{result}");



    let boolean_value =true;

    match boolean_value {
        true =>{
            println!("true");
            println!("!");
        }
        _ =>(),
    }



    let number2 =8;

    match number2 {
        2 | 4 | 6 | 8 =>println!("{number2} is even"),
        3 | 5 | 7 =>println!("{number2} is odd"),
        _ =>println!("adkdkm"),
    }



    let number3 = 5;

    match number3 {
        value if  value % 2==0 =>println!("{value} is even") ,
        x if  x % 2 !=0 =>println!("{x}  is odd"),
        _ => unreachable!(),
        
    }
}
