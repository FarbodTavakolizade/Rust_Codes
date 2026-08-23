// trait     debug display copy clone  

//compiler directive || attribute

#[derive(Debug)] //debug trait 
struct Book{
    title:String,
    author:String,
    pages:u32,
    is_available:bool,
}
fn main(){

    let my_book= Book{
        title:String::from("a"),
        author:String::from("b"),
        pages:100,
        is_available:true,
    };

    println!("{:?}",my_book);
    println!("{:#?}",my_book);
    
/* manual
use std::fmt 

impl fmt::Debug for Book{
    fn fmt(&self, f:&mut fmt::Formatter)->fmt::Result {
        write!(
            f,
            "Book{{title:{},author:{},pages:{},is_available:{}}}",
            self.title,
            self.author,
            self.pages,
            self.is_available
        )
    }
}
*/

}
