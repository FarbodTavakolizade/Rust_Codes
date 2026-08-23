//immutable &T  read

//mutable &mut T  write and read 
struct Tea{
    price:f64,
    name:String,
    is_hot:bool,
}

fn make_tea(name:String, price:f64,is_hot:bool)->Tea{
    Tea{
        name,
        price,
        is_hot,
    }
}
fn main(){
    let mut my_tea=make_tea(String::from("green tea"), 10.0, true);
    
    //immutable reference
    describe_tea(&my_tea);//readable

    //mutable reference 
    increase_price(&mut my_tea);

    println!("info of tea is {} - {}",my_tea.name, my_tea.price);


}
//immutable reference
fn describe_tea(tea:&Tea){
    println!("tea name is {}",tea.name);
    println!("tea price is {}",tea.price);
    println!("is hot {}",tea.is_hot);
}


//mutable reference

fn increase_price(tea:&mut Tea){
    tea.price+=1.0;
    println!("new price is {}",tea.price);
}
