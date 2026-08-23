fn add_five(num:&mut i32){
    *num+=5;
}
fn get_mutable_ref(num2:&mut i32)->&mut i32{
    num2
}

fn main() {
   //mutable borrowing
   let mut x=10;
   {
         let y= &mut x;
         *y+=1;
        
   }
   println!("x is {}",x);


   let mut num = 10;
   add_five(&mut num);
   println!("num is {}",num);


   let mut v=20;
   let v2=get_mutable_ref(&mut v);
   *v2+=10;
   println!("v is {}",v);
}





