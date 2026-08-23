// fn get_biggest_number<T :PartialOrd>(a:T , b:T) ->T{
//     if a>b{
//         a
//     }else {
//         b
//     }
// }
// fn main() {
//     println!("the biggest number is {}",get_biggest_number(1.0,3.14));
    
// }
//----------------------------------------------------------------
// fn make_pair<T ,U>(x:T , y:U) ->(T ,U){
//     (x ,y)
// }
// fn main() {

//     let pair1 =make_pair(10,20 );

//     let pair2 =make_pair("helloworld!", 3.14);

//     println!("pair one is : ({} ,{})",pair1.0,pair1.1);
//     println!("pair two is : ({} ,{})",pair2.0,pair2.1);
// }
//----------------------------------------------------------------
struct Point<T>{
    x:T,
    y:T
}
impl Point<i32> {
    fn sum(&self) ->i32{
        self.x +self.y
    }
}

impl Point<&'static str> {
    fn concat(&self) ->String{
        self.x.to_string() +self.y
    }
}


fn main() {
    let p1 =Point{ x:5 ,y:7};
    let p2 =Point{x:"hello " , y:"world!"};
    println!("sum is : {}",p1.sum());
    println!("concat is : {}",p2.concat());

}