//  add rand = "0.8" below [dependencies] in Cargo.toml
use rand::Rng;
fn main(){
   let mut rng= rand::thread_rng();

   let x:u32=rng.gen_range(1..=2000000);

   print!("random number is :{x}");

}









