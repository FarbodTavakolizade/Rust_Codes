
struct Book{
    title:String,
    author:String,
}
impl Book{
    fn describe(&self){ //(self:&Self) 
        println!("book {} is written by {}",self.title,self.author)
    }

    

}
fn main(){
    let book=Book{
        title:"rust".to_string(),
        author:"reza".to_string(),
    };
    book.describe(); //immutable reference

}
