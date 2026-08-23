fn main() {
    let pi:f64=3.141555;
    let pi_2:f32=-3.14;
    println!("value of pi is {pi}");
    println!("{}",pi.floor());
    println!("{}",pi.ceil());
    println!("{}",pi.round());
    println!("{}",pi_2.abs());
    //format specifier for demonstrating digits of floating number

    println!("new value of pi is{pi:.2}");

    //type casting 

    let height=50;
    let height_i8=height as i8;
    let weight =100.25;
    let weight2=weight as f32;
    println!("{height_i8} and {weight2}");

    //math operators

    let addition =8+9;
    let substraction =9-8;
    let multiplication =8*9;
    let division =8.0/5.0;
    let remainder =9%2;
    println!("{division} \t {remainder}");
}
