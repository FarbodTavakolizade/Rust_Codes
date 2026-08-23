
/*
variable shadowing

fn main(){
    let x = 10;
    let x = 15;      //overshadow first x
    println!("value of x is {}",x);
}
----------------------------------------------------------------
     Scope   
    
    fn main(){
    let x=5;

    {
        let y =15;
       
    }
    println!("value of x is {x} and value of y is {y}");
    }
    */
    




fn main(){
// variable shadowing and scopes mixture
    let x=8;

    {
        let y =5;
        let x=3;
        println!("the value of new x is {x}"); //over shadowing for x value
        println!("the value of new y is {y}");

       
    }
    println!("value of x is {x}");
}
