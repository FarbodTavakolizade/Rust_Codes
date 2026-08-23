
fn main() {

//String ==> heap_allocated , growable string , owned  mutable

let mut s =String::from("hello world");

s.push_str("!");

//----------------------------------------------------------------

//&str (string slice)==>stack or inside String reference to string , borrowed it is not an owner , immutable 

let s2:&str="hello world";












//---------------------------------------------------------

//&str(string slice) ==>String

let s_slice :&str="salam";

let s_string: String=s_slice.to_string();// String::from("....");

println!("&str :{s_slice}");

println!("String :{s_string}");



//-----------------------------------------------------------

//String ==>&str(string slice)


let s_string2:String=String::from("hello");

let s_slice2:&str=&s_string2; // or &s_string2.as_str();

println!("String :{s_string2}");

println!("&str : {s_slice2}");



















}







