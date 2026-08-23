fn main() {
    //boolean :=> true / false
    println!("{}", true);
    println!("{}", false);
    println!("{}", !true); 
    println!("{}", !false);
    //== equal   != not equal
    println!("{}", false==true);
    println!("{}", false!=true);
    println!("{}", "ali"=="ali");

    //>=   <=   >   <  ==  !=
    let age =16;
    let can_get_lisence:bool=age >=18;
    //let cannot_get_lisence:bool=!can_get_lisence;
    println!("my age is {age} can i get lisence?{can_get_lisence}");
    println!("{}",15==15.0 as i32);

    //&& : and           ||:or
    let qualified_in_exam:bool=!false;
    let qualified_as_a_driver:bool=can_get_lisence&&qualified_in_exam;
    println!("{qualified_as_a_driver}");

    let a=false;
    let b=false;
    let c =a||b;
    println!("{c}");
    
    


}
