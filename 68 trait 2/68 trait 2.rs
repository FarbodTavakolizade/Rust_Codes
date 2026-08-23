// //trait bound on generic function
// fn larger<T :Ord>(a:T ,b:T) ->T{
//     if  a > b{
//         a
//     }else {
//         b
//     }
// }
// fn main() {
//     let x=10;
//     let y=20;
//     let result =larger(x, y);
//     println!("larger number is {}",result);

// }
//---------------------------------------------------
//trait bound on generic struct
//---------------------------------------------------------------------
//multiple trait bound
// use std::fmt::Debug; //trait debug

use std::fmt::Debug;

// fn clone_and_print<T: Clone +Debug>(x:T){
//     let y =x.clone();
//     println!("x:{:?} , y:{:?}",x,y);
// }
// fn main() {
//     let number =53;
//     let text ="salam";
//     clone_and_print(number);
//     clone_and_print(text);
//}
//---------------------------------------------------------------------
#[derive(Debug,Clone)]
struct Point{
    x :i32,
    y:i32,
}
fn clone_and_print<T:Clone +Debug>(val:T){
    let copy =val.clone();
    println!("original form is {:?} and copy form is {:?}",val,copy)
}
fn main() {
    let p =Point{

        x:3 ,y:5
    };

    clone_and_print(p);
}
