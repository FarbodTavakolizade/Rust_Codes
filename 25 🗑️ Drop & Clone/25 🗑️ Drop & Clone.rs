fn main() {
   //clone  drop
    let x=35;
    let y=x;
    println!("x: {}", x);
    println!("y: {}", y);

    let string1=String::from("hcsbhcb");
    let string2=string1.clone();
    println!("string1: {}", string1);
    println!("string2: {}", string2);


    let s=String::from("hello");
    drop(s);
    print!("{s}");
    // println!("{s}"); // This will cause a compile-time error because s has been dropped

}





