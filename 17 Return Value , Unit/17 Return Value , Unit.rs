fn main() {
   let result=square_of_number(5);
   println!("result is {result}");
   let b=a();//unit : an empty tuple
}
//return value
fn square_of_number(number:i32) ->i32{
     number*number

}

fn a(){}
