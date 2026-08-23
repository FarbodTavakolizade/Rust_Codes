//usize  unsigned size  u32  u6
 // array  index array   byte of memory
 //len()  array || vector

 struct Book{
    title:String,
    page:usize,
 }

 impl Book{
    fn turn_page(&mut self){  //mutable reference
        self.page+=1;
        println!("turned page to{}",self.page);
    }
 }
fn main(){
    let array=[1,2,3,4];
    let index:usize=2;
    println!("value at index {} is {}",index,array[index]);
 


 let mut book =Book{
    title: "rust".to_string(),
    page :1,
 };

 book.turn_page();//2
 book.turn_page();//3
 book.turn_page();//4
}
