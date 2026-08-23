struct Book{
   title:String,
}
impl Book {
    fn take_ownership(self:Self){ //(self)
      println!("taking ownership of {}",self.title);
    }
}

fn main(){
   let book =Book{
      title:"rust".to_string(),
   };

   book.take_ownership();

}
