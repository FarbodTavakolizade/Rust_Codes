//mutable & immutable ownership
//fn function_name([mut]x: T)

struct Tea{
    price:f64,
    name:String,
    is_hot:bool,
}

fn make_tea(name:String ,price :f64, is_hot:bool)->Tea{
    Tea {
        name,
        price,
        is_hot,
    }
}
fn main(){
    let green_tea:Tea=make_tea(String::from("black tea"), 5.00, true);
    drink_tea(green_tea);

   // println!("{}",green_tea.name);

}

fn drink_tea(mut tea:Tea){
    println!("i want to order {}",tea.name);
}
