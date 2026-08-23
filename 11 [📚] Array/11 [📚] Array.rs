fn main() {
   //array compound data data
   let numbers:[i32;6]=[4, 5, 6, 7, 8, 9];
   let names:[&str;3]=["ali", "reza", "mahdi"];

   //len method for length of array

   println!("length of array is {}",names.len());
   let first_index:&str=names[1];  //accessing chosen index
   println!("{first_index}");

//use mut keyboard for replace

   let mut numbers2:[f64;3]=[15.0, 16.0, 8.5];
   println!("{}",numbers2[0]);
   numbers2[0]=16.5;
   println!("{}",numbers2[0]);


}
