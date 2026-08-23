fn hello_world(){//parameter
    println!("helloworld!!");
}

fn main() {
    hello_world();//argument
    student("reza", 15.0);
    
}


fn student(name:&str, score:f64){
    println!("student name is {name} and student score is{score}");
}
