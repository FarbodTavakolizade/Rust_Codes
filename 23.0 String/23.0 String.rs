
fn main() {
   let mut name="reza"; //string literal  &str  compile time
    name="kndcn";
    println!("{name}");

    //String  runtime

    let mut x=String::new();
    x.push_str("hello world");
    println!("{x}");


    let y=String::from("hello world 2");
    println!("{y}");
}





