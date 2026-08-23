// //trait bound on generic function
// fn larger<T :Ord>(a:T ,b:T) ->T{
//     if  a > b{
//         a
//     }else {
//         b
//     }
// }
// fn main() {
//     let x=10;
//     let y=20;
//     let result =larger(x, y);
//     println!("larger number is {}",result);

// }
//---------------------------------------------------
//trait bound on generic struct
struct Container<T: Ord> {
    value: T,
}
impl<T: Ord> Container<T> {
    fn new(value: T) -> Self {
        Container { value }
    }

    fn bigger<'a>(&'a self, other: &'a T) -> &'a T {
        if &self.value > other {
            &self.value
        } else {
            other
        }
    }
}

fn main() {

    let x1=Container::new(10);
    let x2=Container::new(20);

    println!(" bigger num is {}",x1.bigger(&x2.value));

    let string1 =Container::new("abcd");
    let string2=Container::new("efghi");

    println!("bigger string is {}",string1.bigger(&string2.value));
}
