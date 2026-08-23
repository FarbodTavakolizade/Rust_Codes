use std::default;
use ::std::fmt::Debug;
//wherre clause usage
fn process_pair<T ,U>(x:T , y:U) ->T
where 
T:Clone+Debug,
U:Debug,
{
    println!("y is {:?}",y);
    x.clone()
}
    

fn main(){
    let a =10;
    let b="salam";

    let result =process_pair(a, b);

    println!("result is {}",result);
    
}