// /*
// enum Option<T>{
// Some(T)
// None
// }
// */
// fn find_even_number(numbers :Vec<i32>) -> Option<i32>{
//     for num in numbers{
//         if num%2 ==0{
//             return Some(num);
//         }
//     }
//     None
// }
// fn main() {
//     let some_numbers =Some(5);
//     let no_number :Option<i32> =None;

//     println!("some number is {:?}",some_numbers);
//     println!("no  number is {:?}",no_number);

//     let nums=vec![1,3,5,7,8];

//     let result =find_even_number(nums);
//     println!("{:?}",result);

// }

//---------------------------------------------------------------------
// struct Person {
//     name: String,
//     age: Option<u8>,
// }
// impl Person {
//     fn new(name: &str, age: Option<u8>) -> Person {
//         Person {
//             name: name.to_string(),
//             age,
//         }
//     }

//     fn print_info(&self) {
//         match self.age {
//             Some(age) => println!("{} is {} years old", self.name, age),
//             None => println!("{} age is unknown", self.name),
//         }
//     }
// }

// fn main() {
//     let person1 = Person::new("reza", Some(25));
//     let person2 = Person::new("ali", None);
//     person1.print_info();
//     person2.print_info();
// }
//---------------------------------------------------------------------
// fn main() {

//     let x =Some(10);

//     if let Some(num) = x  {
//         println!("found a number: {}",num);
//     }else {
//         println!("no number found");
//     }

//     let numbers =vec![Some(1),None,Some(3)];

//     for i in numbers{
//         if let Some(n) =i {
//             println!("Numbers: {}",n);
//         }else {
//             println!("no number")
//         }
//     }
// }
//---------------------------------------------------------------------
struct Player {
    name: String,
    score: Option<u32>,
}

impl Player {
    fn new(name: &str, score: Option<u32>) -> Player {
        Player {
            name: name.to_string(),
            score,
        }
    }

    fn report(&self) {
        if let Some(s) = self.score {
            println!("{} scored {} goals", self.name, s);
        } else {
            println!("{} has score no points", self.name);
        }
    }
}

fn main() {
    let player_one = Player::new("reza", Some(3));
    let player_two = Player::new("farbod", None);
    player_one.report();
    player_two.report();
}
