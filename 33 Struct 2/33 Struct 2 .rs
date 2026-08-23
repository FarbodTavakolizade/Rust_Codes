struct Person{
    name:String,
    age:i32,
    is_old:bool,
}
fn main(){
    let x=define_human(String::from("reza"), 25, false);

    println!(
        "my name is {} and my age is {} am i old?{}",x.name , x.age , x.is_old
    ) ;
}

fn define_human(name:String, age:i32, is_old:bool) ->Person{
    Person{
        name:name,
        age:age,
        is_old:is_old,
    }
}





















