fn main() {
    //reference  &

    //immutable reference
    let x=5;
    let new_x = &x;
    let new_x2=&new_x;
    println!("x: {}, new_x: {}", x, new_x);
    println!("new_x2: {}", new_x2);


    //mutable reference
    let mut y=10;
    let new_y = &mut y;
    *new_y += 5; // dereference to modify the value
    println!("y is {y}");
   
}





