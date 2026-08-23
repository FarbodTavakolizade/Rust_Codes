// /* async  runtime   works on another task
// non async  does nothing  
// state =   done   , pending
// cpu bounded tasks 
// i/o bounded tasks

// async    await(changes state from pending to done)   tokio
// #[tokio::main]
// async fn main(){
//     body
// }
// //tasks=> a lightweight , non blocking unit of execution 

//  */ 

// use tokio::task::JoinHandle;

// async fn hello(name :&str) ->String{
//     format!("hello {}",name)
// }

// async fn hello_world() ->String{
//     "helloworld".to_string()
// }
// #[tokio::main]
// async fn main(){
//     let value =hello_world().await;
//     println!("{}",value);

//     let join_hadle=tokio::spawn(hello("Farbod"));
//     let value2 =join_hadle.await.unwrap();

//     println!("{}", value2);

// }

 // sync   async  thread
// use std::thread;
// use std::time::Duration;
 use tokio::time::{sleep,Duration};
//  fn main(){
//     for i in 1..=10{
//         println!("number in sync {}",i);
//     }

//     let mut handles =vec![];
//     for i in 1..=10{
//         let handle =thread::spawn(move || {
//             println!(" number in threads is {}",i);
//             thread::sleep(Duration::from_millis(20));
//         });
//         handles.push(handle);
//     }
//     for h in handles{
//         h.join().unwrap();
//     }
//  }

 #[tokio::main]
 async fn main() {
    let mut tasks=vec![];
    for i in 1..=10{
        let task = tokio::spawn(async move {
            println!("number isn async[tokio] is {}",i);
            sleep(Duration::from_millis(200)).await;
        });
        tasks.push(task);
    }
     for t in tasks{
        t.await.unwrap();
     }
 }

