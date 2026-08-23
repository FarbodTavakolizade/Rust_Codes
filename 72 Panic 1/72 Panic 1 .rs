   
/*    Recoverable Error Result[panic! , unwrap , expect] manage 
enum Result <T, E>  match 
    Ok(T),
    Err(E),
*/



//  Unrecoverable Error    panic   [panic! , unwrap , expect]
use std::fs::File;
fn main() {
    println!("hello ");

   // panic!("crash");

    println!("salam");

    // let numbers =[1,2,3];
    // println!("{}",numbers[3]);  program will panic


    let data =File::open("data.txt");

    let data_file=match data {
        Ok(file) =>file,
        Err(e) =>panic!("problem opening file : {:?}",e),
    };

    println!("Data file is {:?}",data_file);

    
}


    
