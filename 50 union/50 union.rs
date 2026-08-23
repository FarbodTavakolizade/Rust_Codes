//union

union U {
    i:i32,
    f:f64,
    b:bool,
}

fn main() {
    let mut x=U{i:45};
    unsafe {
        println!("x as i32 is {}",x.i);
        // x.f=3.14;
        println!("x as f64 is {}",x.f);
        println!("x as bool is {}",x.b);
    }
}