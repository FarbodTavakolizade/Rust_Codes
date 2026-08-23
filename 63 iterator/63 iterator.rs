fn main() {
    let mut numbers =[2,1,17,99,34,56];
    //iterator

    let number_iterator = numbers.iter();

    for number in number_iterator {
        println!("{}", number);
    }



    let mut colors =vec!["red" ,"yellow ", "green "];
    let mut colors_iterator =colors.iter();
    println!("colors iterator is = {:?}",colors_iterator);

    println!("{:?}" ,colors_iterator.next());
    println!("{:?}" ,colors_iterator.next());
    println!("{:?}" ,colors_iterator.next());
    println!("{:?}" ,colors_iterator.next());

    //iter() method
    // for color in colors.iter() {
    //     println!("{}",color);
    // }
    println!("colors ={:?}",colors);




    //into_iter()  simillar to normal for loop
    // for color in colors.into_iter(){
    //     println!("{}",color);
    //  }
   // wrong form  println!("colors ={:?}",colors);


   //iter_mut()

   for color in colors.iter_mut(){
    *color ="black";
    println!("{}" ,color);
   }

    println!("colors ={:?}",colors);





    //----------------------------------------


    let number2 :Vec<i32> =vec![1,2,3];

    //map iterator adapter

   let even_numbers:Vec<i32>=number2.iter().map(|i|i*2).collect();

   println!("numbers ={:?}",number2);
   println!("even_numbers ={:?}",even_numbers);




 
}