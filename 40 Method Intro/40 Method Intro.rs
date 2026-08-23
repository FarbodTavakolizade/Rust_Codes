// fn function_name(){}

//impl

struct Book{
    title:String,
    page:u32,
    author:String,
    is_available:bool,
}

impl Book{
    fn  display(&self){
        println!("Title is {}",self.title);
        println!("pages {}",self.page);
        println!("is available  {}",self.is_available);
        
    }

    fn is_long(&self)->bool{
        self.page>300
    }

    fn borrow(&mut self){
        self.is_available=false;
    }

    fn return_book(&mut self){
        self.is_available=true;
    }

}

fn main(){
    let mut my_book=Book {
        title:String::from("a"),
        page:300,
        author:String::from("b"),
        is_available:true,
    };

    my_book.display();

    if my_book.is_long(){
        println!("this is a long book");
    }else {
        println!("this is a short book");
    }

    my_book.borrow();
    println!("borrowed the book");

    my_book.display();


    my_book.return_book();
    println!("Returned the book");

    my_book.display();

}
