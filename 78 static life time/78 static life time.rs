// struct Book<'a>{
//     title: &'a str,
// }

// fn longer_book<'a>(b1:&'a Book<'a> , b2:&'a Book<'a> ) ->&'a Book<'a>{
//     if b1.title.len() >b2.title.len(){
//         b1
//     } else {
//         b2
//     }
// }

fn pick_first<'a , 'b>(a:&'a str , _b:&'b str) ->&'a str{
    a
}

fn pick_second<'a , 'b>(a:&'a str , b:&'b str) ->&'b str{
    b
}

fn pick_longer<'a , 'b>(a:&'a str , b:&'b str) ->&'b str
where 
    'a : 'b,

{
    if a.len() > b.len(){
        a
    }else {
        b
    }

}

struct User<'a>{
    name:&'a mut String,
}

impl <'a> User<'a> {
    fn change_name(&mut self , new:&str){
        self.name.clear();
        self.name.push_str(new);
    }
}

fn give_static_lifetime()-> &'static str{
    "salam man halam khoobe"
}


struct Message{
    text:&'static str,
}
fn main() {
    // let name1 =String::from("Rust");
    // let name2 =String::from("Advanced Rust");

    // let b1= Book{title:&name1};
    // let b2= Book{title:&name2};

    // let longer =longer_book(&b1, &b2);

    // println!("longer book is {}",longer.title);
    

    let s1=String::from("salam");
    let s2=String::from("chetori?");

    let result = pick_first(&s1, &s2);
    let result2 = pick_second(&s1, &s2);
    let result3 = pick_longer(&s1, &s2);

    println!("result is {}",result);

    println!("result2 is {}",result2);

    println!("result3 is {}",result3);
    

    let mut n = String::from("alireza");
    let mut n2 =User{name : &mut n};

    n2.change_name("farbod");

    println!("new name is {}",n2.name);


    //static life time    string literal

    let x :&'static str="hello world";
    println!("{}",x);

    let msg =give_static_lifetime();
    println!("{}",msg);

    

    let m = Message{text:"kscnjsdbhcjbc"};
    println!("messsge is {}",m.text);

}