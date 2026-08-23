#[derive(Debug)]
struct Book{
   title:String,
   author:String,
   page:u32,
}
impl Book{
   fn new() ->Self{
      Self{
         title: String::new(),
         author: String::new(),
         page: 0,   
      }
   }
   fn title(mut self , t:&str) ->Self{
      self.title=t.to_string();
      self
   }
   fn author(mut self , a:&str) ->Self{
      self.author=a.to_string();
      self
   }
   fn page(mut self , p:u32) ->Self{
      self.page=p;
      self
   }
   
   
}
fn main() {
    let book =Book::new()
    .title("rust")
    .author("ali")
    .page(100);

    println!("{:#?}", book);
}