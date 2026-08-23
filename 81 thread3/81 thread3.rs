use std::thread;
// use std::sync::mpsc;
// use std::sync::Mutex;  // mutex = mutual exclusion
// use std::rc::Rc;
// use std::sync::Arc;
use std::sync::{Arc,Mutex};
// fn main() {
//     //spawn a new thread

//     let handle =thread::spawn(||{
//         println!("hello from spawn thread");
//     });
//     handle.join().unwrap();

//     println!("hello from main thread");

// }
//------------------------------------------------------------------------------

// fn main() {
//     //create a channel  (tx =sender  , rx=reciever)   message=msg

//     let (tx,rx)=mpsc::channel::<String>();

//     let handle =thread::spawn(move||{
//     //recieve msg (blocking)
//     let msg =rx.recv().unwrap();
//     println!("{msg}");
//     });

//     tx.send("salam".to_string()).unwrap();
//     handle.join().unwrap();

// }

//------------------------------------------------------------------------------
// fn main() {
//     let (tx,rx)=mpsc::channel::<String>();

//     //clone the sender

//     let tx2=tx.clone();

//     let h1=thread::spawn(move||{
//         tx.send("message from thread 1".to_string()).unwrap();
//     });

//     let h2=thread::spawn(move||{
//         tx2.send("message from thread 2".to_string()).unwrap();
//     });

//     //recieve two messages

//     for _ in 0..2{
//         let msg =rx.recv().unwrap();
//         println!("reciever got :{msg}");
//     }

//     h1.join().unwrap();
//     h2.join().unwrap();
// }
//------------------------------------------------------------------------------
                 //Mutex
// fn main() {
//     let counter =Mutex::new(0);
//     {
//         //lock -> returns mutex guard
//         let mut num =counter.lock().unwrap();
//         *num+=1;

//         //lock automatically release   when variable (num) goes out of scope

//     }

//     println!("counter ={}",*counter.lock().unwrap());
// }
//------------------------------------------------------------------------------
// fn main() {
//     let data = Rc::new(String::from("Hello Rc"));

//     let a= Rc::clone(&data);   
//     let b= Rc::clone(&data);

// multiple variables share ownership of same data
//     println!("data:{data}");
//     println!("a:{a}");
//     println!("b:{b}");
//     println!("strong_count={}",Rc::strong_count(&data));   number of pointers
// }
// rc is for single thread
//------------------------------------------------------------------------------
// fn main() {
//     let shared =Arc::new(vec![10,20,30]);

//     let s1  = Arc::clone(&shared);
//     let s2=Arc::clone(&shared);


//     let h1=thread::spawn(move||{
//         println!("thread 1: length ={}",s1.len());
//     });

//     let h2=thread::spawn(move||{
//         println!("thread 2:first element ={}",s2[0]);
//     });

//     h1.join().unwrap();
//     h2.join().unwrap();

    
// }
//------------------------------------------------------------------------------

fn main() {
    let counter =Arc::new(Mutex::new(1));

    let mut handles =Vec::new();

    for _ in 0..10{
        let c = Arc::clone(&counter);

        handles.push(thread::spawn(move||{
            let mut num = c.lock().unwrap();
            *num*=2;
        }));
    }

    for h in handles{
        h.join().unwrap();
    }

    println!("result = {}",*counter.lock().unwrap());
}