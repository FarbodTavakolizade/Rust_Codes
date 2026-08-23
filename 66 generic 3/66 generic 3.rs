// struct Pair<T ,U>{
//     first:T,
//     second:U,
// }

// impl Pair<i32 ,i32> {
//     fn add(&self) ->i32{
//         self.first +self.second
//     }
// }

// impl Pair<String ,String> {
//     fn join(&self)->String{
//         format!("{}{}",self.first,self.second)
//     }
// }

// fn swap<T, U>(a:T , b:U) ->Pair<U, T>{
//     Pair { first: b, second: a }
// }
// fn main() {
//     let p1 =Pair{first: 3, second:7};
//     println!(" sum is : {}",p1.add());

//     let p2 =Pair{
//         first:String::from("hello"),
//         second:String::from("world"),
//     };
//     println!("joined ={}",p2.join());

//     let swapped =swap(100, String::from("salam"));


//     println!("swapped.first ={}, swapped.second ={}",swapped.first ,swapped.second);
    
// }
//*************************************************
// #[derive(Debug)]
// enum Option_i32{
//     Some(i32),
//     None,
// }
// #[derive(Debug)]
// enum Option_f64{
//     Some(f64),
//     None,
// }
// fn main() {
//     let int =Option_i32::Some(10);
//     let float =Option_f64::Some(5.0);

//     println!("{:?} {:?}",int ,float);
// }
//*************************************************
// fn largest_i32(list :&[i32]) ->&i32{
//     let mut largest =&list[0];
//     for item in list{
//         if item > largest{
//             largest =item;
//         }
//     }
//     largest
// }

// fn largest_char(list:&[char]) ->&char{
//       let mut largest =&list[0];
//     for item in list{
//         if item > largest{
//             largest =item;
//         }
//     }
//     largest

// }
// fn main() {

//     let number_list =vec![34,50,25,100,65];

//     let result =largest_i32(&number_list);
//     println!(" largest number is {result}");

//       let char_list =vec!['y' ,'m' ,'a', 'q'];

//     let result =largest_char(& char_list);
//     println!(" largest char is {result}");
// }
//*************************************************
fn largest<T:PartialOrd>(list:&[T]) ->&T{
       let mut largest =&list[0];
     for item in list{
        if item > largest{
             largest =item;
         }
   }
   largest
}
  fn main(){
    let number_list =vec![34,50,25,100,65];
    let result =largest(&number_list);
    println!(" largest number is {result}");

    let char_list =vec!['y' ,'m' ,'a', 'q'];

     let result =largest(& char_list);
    println!(" largest char is {result}");

}