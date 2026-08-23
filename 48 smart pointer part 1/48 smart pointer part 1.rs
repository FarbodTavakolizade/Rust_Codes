use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;
use std::thread;
fn main() {
    let x1=10; //stack

    let y1 =&x1;

    println!("y is {}",y1);

    let mut x2=5;

    let y2 =&mut x2;
    *y2+=1;

    println!("x2 is {}",x2);

    //Box  heap  1 owner

    let b1=Box::new(5);//smart pointer

    println!("box value is {}",b1);

    let b2=b1;

    println!("{}",b2);//owner
    //println!("{}",b1); error  b2 is owner

    //Rc thread  read  multiple occasions for accessing data


    let a =Rc::new(11);
    let b =Rc::clone(&a);
    let c =Rc::clone(&a);

    println!("a is {}   b is {}   c is {}",a,b,c);
    println!("{}",Rc::strong_count(&a));

//RefCell 

let data=RefCell::new(12);
*data.borrow_mut()+=10;

println!("value is {}",data.borrow());

//rc and refcell mixture
let data2=Rc::new(RefCell::new(6));


let d1=Rc::clone(&data2);
let d2=Rc::clone(&data2);

*d1.borrow_mut()-=5;
println!("value is {}",d2.borrow());

//Arc = Atomic reference count 

//arc mutex





    
}
