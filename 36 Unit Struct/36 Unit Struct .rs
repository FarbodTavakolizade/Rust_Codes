struct Marker;
struct Logger;

struct Admin;

trait CanAccessPanel {
    fn access(&self);
}

impl CanAccessPanel for Admin{
    fn  access(&self){
        println!("you can access the panel");
    }
}

fn log_message(_: Logger){
    println!("message created");
}

fn main(){
  let _m=Marker;
  println!(" MARKER created");

  let x= Logger;
  log_message(x);

  let a =Admin;
  a.access();
}
