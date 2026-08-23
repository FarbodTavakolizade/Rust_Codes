fn main() {
    let first_name =String::from("farbod");
    let first_name2=&first_name[0..5];
    print!("{} \n",first_name2.len());
    let food="🍕"; //4byte
    println!("{}",food.len());
    let x=&food[0..4];
    println!("{}",x.len());


    let values=[4,8,15,16,23,42];

    let my_slice=&values[..4];
    println!("{my_slice:?}");

    let my_slice=&values[2..4];
    println!("{my_slice:?}");

    let my_slice=&values[2..];
    println!("{my_slice:?}");

    let my_slice=&values[..];
    println!("{my_slice:?}");

    let my_slice=&values;
    println!("{my_slice:?}");

    let mut my_array=[10,15,20,25,30];
    let my_slice2=&mut my_array[0..3];
    println!("{:?}",my_slice2);
    my_slice2[2]=100;
    println!("{:?}",my_slice2);
    println!("{:?}",my_array);
}







