fn hello(n:&String){
    println!("hello {}", n);
}


fn main() {
    //borrowing & reference
    let s1 = String::from("hello");
    let s2=&s1;
    println!("s1: {}, s2: {}", s1, s2);



    let name=String::from("ali");
    hello(&name);
    println!("name: {}", name);

}






