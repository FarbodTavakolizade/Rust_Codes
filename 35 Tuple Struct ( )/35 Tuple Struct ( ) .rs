struct Person(String,u8);
struct Point(i32,i32);
fn main(){
    let mut p1=Person(String::from("alireza"),30);
    p1.1=50;
    println!("Name:{}",p1.0);
    println!("age:{}",p1.1);

    let my_point=Point(3,4);
    let distance=distance_from_origin(my_point);

    println!("distance from origin is {}",distance);

}
fn distance_from_origin(p:Point) ->f64{
    let x=p.0;
    let y=p.1;
    ((x.pow(2)+y.pow(2)) as f64).sqrt()

}

