//function runtime   macro compile time
//(....)=>{....}  expr:expression
macro_rules! add {
    ($a:expr ,$b:expr) => {
        $a +$b 
    };
}

macro_rules! square {
    ($x:expr) => {
        $x *$x
    };
}

fn add(a:i32 , b:i32) ->i32{
    a+b
}

fn main() {
    println!("Hello, world!");
    
    let result=add(10, 20);
    println!("{}",result);


    let result2=add!(5,4);
    println!("{}",result2);

    let square_of_num = square!(10);

    println!("{}",square_of_num);

    
}
