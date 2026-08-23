//Associated functions  =>   String::from()  String::new()
struct Rectangle{
   width:u32,
   height:u32,
}
impl Rectangle{
   fn new(width:u32 , height:u32) ->Self{
      Self { width , height }
   }



   fn square(size:u32) ->Self{
      Self{
         width:size,
         height:size,
      }
   }


   fn area(&self)->u32{
      self.width *self.height
   }
}
fn main(){
   let rectangle =Rectangle::new(10,20);
   let square=Rectangle::square(20);

   println!("area of rectangle is {} and area of square is {}",rectangle.area(),square.area())

}
//--------------------------------------------------
/* 
struct Person {
   name:String
}


impl Person{
   fn new(name:&str)->Self{
   Self{
         name:name.to_string(),
      }
   }

fn salam(&self){
   println("{} says salam",self.name );
   }
}

   fn main(){
   
   let a =Person::new("ali");

    a.salam();
   
   
   
   }

   */
