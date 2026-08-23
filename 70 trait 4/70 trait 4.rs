// use std::default;

// //trait defualt
// #[derive(Debug,Default)]
// struct Config{
//     x:u8,
// }
// fn main(){
//     let a =Config::default();
//     println!("{:?}",a);
// }
//----------------------------------------------------
// use std::collections::HashSet;
// #[derive(Debug,Hash,PartialEq, Eq)]
// struct Point(i32 ,i32);
// fn main() {
//     let mut set =HashSet::new();
//     set.insert(Point(1,2));
//     println!("{:?}",set.contains(&Point(1, 2)));
// }
//----------------------------------------------------
// == !=  PartialEq
// #[derive(Debug,PartialEq)]
// struct Person{
//     name:&'static str,
// }
// fn main() {
//     let a =Person{name:"ali"};
//     let b =Person{name:"alireza"};
//     let c =Person{name:"ali"};
//     println!("{}", a==b);
//     println!("{}", a==c); 
// }
//----------------------------------------------------
//trait Eq
// #[derive(Debug,PartialEq, Eq)]
// struct Id(u32);
// fn main() {
//     let x=Id(5);
//     let y =Id(5);
//     println!("{}", x==y);
// }
//----------------------------------------------------
//PrtialOrd  >   <  >=   <=
#[derive(Debug,PartialEq, PartialOrd)]
struct score(i32);
fn main() {
    let a=score(20);
    let b =score(20);

    println!("{}",a<b);
    println!("{:?}",a.partial_cmp(&b));
    
}