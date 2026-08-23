fn main() {
    //structure ==>struct
    
    struct Car{
        price:f64,
        name:String,
        is_safe:bool,
        logo:char,
    }

    let mut bmw=Car{
        name:String::from("bmw"),
        price:4.5,
        is_safe:true,
        logo:'B',
    };
    bmw.name=String::from("benz");
    bmw.price=5.56;
    bmw.logo='f';


    println!(
        "name is {} \n and price is {} \n and is it safe?{} \n and logo is {}",
        bmw.name,bmw.price,bmw.is_safe,bmw.logo
    );
    

}


























