use std::sync::mpsc;
use std::thread;
fn main() {
    //main thread starts here
    //create a new channel   mspsc  multiple producer single consumer

    let (sender , reciever )=mpsc::channel();

    //spawn a new thread 

    let handle =thread::spawn(move||{

        //recieve message from channel

        let message =reciever.recv().unwrap();
        println!("Recieved message :{}",message);

        
    });

    let message=String::from("hello world");
    //send message to channel
    sender.send(message).unwrap();


    handle.join().unwrap();
}