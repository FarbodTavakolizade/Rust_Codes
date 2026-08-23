/*
thread::spawn(||{
    code
});
*/
use std::thread;
use std::time::Duration;
// fn main() {
//     //create a thread
//     thread::spawn(||{
// //everything in here runs in separate thread
//         for i in 0..10{
//             println!("{} from the spawn thread",i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     for i in 0..5{
//           println!("{} from the main thread",i);
//             thread::sleep(Duration::from_millis(2));
//     }
    
// }
//------------------------------------------------------------
// fn main() {
//     //create thread and save the handle to a variable
//     let handle=thread::spawn(||{
//         //everything in here runs in a separate thread
//         for i in 0..10{
//                  println!("{} from the spawn thread",i);
//              thread::sleep(Duration::from_millis(2));
//         }
//     });

//     //wait for the separate threads to complete
//    // handle.join().unwrap();

//     //main thread

//      for i in 0..5{
//            println!("{} from the main thread",i);
//              thread::sleep(Duration::from_millis(2));
//      }
//     handle.join().unwrap();
// }
// //----------------------------------------------------------------------------
fn main() {
    //main thread starts here
    let message=String::from("hello world");


    //move the message value into separate thread
    let handle =thread::spawn(move||{
        println!("{}",message);
    });

    //wait for the thread to finish
    handle.join().unwrap();
    
}