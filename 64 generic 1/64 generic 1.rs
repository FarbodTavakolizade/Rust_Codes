#[derive(Debug)]

struct Rectangle_with_generic<T, U, X>{
    width:T,
    height:U,
    A:X,
}
fn main() {
    let rect =Rectangle_with_generic{
        width:1u8,
        height:3.14f64,
        A:"JBJHBG",
    };
    println!("rect is {:?}",rect);
    


}