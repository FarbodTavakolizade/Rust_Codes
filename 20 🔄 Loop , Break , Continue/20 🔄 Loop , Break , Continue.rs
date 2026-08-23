fn main() {//loop
   /*  loop {
        println!("hello world");
    }
   */

let mut counter =0;
loop{
    counter+=1;
    println!("number :{counter}");
    if counter==5{
        break;
    }
}

let mut counter2 =0;

loop {
    counter2+=1;
    if counter2==6{
        continue;
    }
    println!("number:{counter2}");

    if counter2 ==7{
        break;
    }
}



}
