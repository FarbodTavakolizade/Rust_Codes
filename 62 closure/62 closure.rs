use std::result;

//closure  anonymous function lambda ||
fn main() {
    let print_text = ||println!("hello world");
    print_text();

    let add_one =|x:i32| x+1;
    let result =add_one(3);
    println!("{}",result);

    let squared_sum =|x:i32 , y:i32|{
        let mut sum :i32 =x+y;

        let mut result :i32 =sum *sum;
        return result;
    };

    let result =squared_sum(5,3);
    println!("{}",result);


    let num =100;
    let print_number =||println!(" number is {}",num);
    print_number();


    let word =String::from("hello world");

    //immutable closure 1

    let print_string =||{
        println!("word is {}",word);
    };


    //immutable borrow is possible outside closure 
    println!("length of word is {}",word.len());
    print_string();


    let mut word2 =String::from("hello");

    //mutable closure

    let mut print_string2 =||{
        word2.push_str("world!");
        println!(" word is {}",word2);
    };
    print_string2();
    println!("length of word is {}",word2.len());
}