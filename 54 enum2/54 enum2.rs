enum DiskSize{
    Kb(u32),
    Mb(u32),
    Gb(u32),
}


enum DiskType {
    SSD,
    HDD,
}

fn main() {

     let disk_type =DiskType::SSD;

    // if disk_type == DiskType::SSD{
    //     println!("ssd");
    // } else {
    //     println!("hdd");
    // }

    match  disk_type{
        DiskType::SSD =>println!("ssd"),
        DiskType::HDD =>println!("hdd"),
    }



    #[derive(Debug)]

    enum Car{
        Benz(String,f64),
        Bmw(String,f64),
    }
    //initialize mutable enum variant with values

    let mut bmw = Car::Bmw(String::from("x4"), 20.0);

    //initialize non_mutable enum variant with values

    let benz=Car::Benz(String::from("c200"), 30.0);

    println!(" bmw before = {:?}",bmw);
    println!(" benz before = {:?}",benz);


    bmw =Car::Bmw(String::from("x5"), 36.0);
   



    println!(" bmw after = {:?}",bmw);
    println!(" benz after = {:?}",benz);




}