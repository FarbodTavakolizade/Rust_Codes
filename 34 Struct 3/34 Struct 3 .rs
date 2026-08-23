struct User{
    username:String,
    email:String,
    active:bool,
    sign_in_count:u64,
}
fn main(){

    let mut user1=User{
        email : String::from("farbodtavakolizade@gmail.com"),
        username:String::from("Farbod"),
        active:true,
        sign_in_count:2,
    };
    let name=user1.username;
    user1.username=String::from("reza");
    println!("{}",user1.username);

    let user2 =build_user(
    String::from("a@gmail.com"),
    String::from("a"));

    println!("{}",user2.sign_in_count);
    

    let user3=User{
        email:String::from("ksnkcdn@gmail.com"),
        username:String::from("kdnne"),
        ..user2
    };

    println!("{}",user3.active);
}

fn build_user(email:String , username:String) -> User{
    User{
        email:email,
        username,
        active:true,
        sign_in_count:2,
    }
}
