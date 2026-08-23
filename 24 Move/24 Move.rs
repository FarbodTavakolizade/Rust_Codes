fn main() {
   //move ==> transfering ownership
    let name1 = String::from("reza");
    let name2=name1; // name1 is moved to name2
    println!("name2: {}", name2);
    // println!("name1: {}", name1); // this will cause an error because name1 is no longer valid
    // name1 is no longer valid after being moved to name2
    // name2 is now the owner of the String
}





