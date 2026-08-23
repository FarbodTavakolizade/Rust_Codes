struct Book<'a>{
    title :&'a str,
}

//Lifetime

// fn longest(a:&str , b:&str) ->&str{
//     if a.len() > b.len(){
//         a
//     } else {
//         b
//     }

// }

fn longest <'a>(a: &'a str , b:&'a str) -> &'a str{
    if a.len() > b.len(){
        a
    } else {
        b
    }
}
fn first<'a>(s:&'a str ) ->&'a str{
    &s[0..1]
}

fn main() {


// let s=String::from("salam");
// let r =&s;

// println!("reference :{}",r);
// println!(" s is still valid :{}",s);

//borrow checker    in this example  compiler returns error
// let r2=&mut s;

// println!("{} {}",r,r2);




// lifetime annotation '

 let x ="rust";
 let y ="lifetime";
 let result =longest(x,y);
 println!("{}",result);

let word =String::from("salam");
let c =first(&word);
println!(" first word is {}",c);


let name =String::from("Rust course");

let book=Book{title :&name};
println!("book is {}",book.title);

}
