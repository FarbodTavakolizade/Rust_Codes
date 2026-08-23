//Error Handling
//Recoverable  Result<Ok  , Err> [Err:= panic! unwrap , expect]  , Unrecoverable     panic   [panic! , unwrap , expect]  
//---------------------------------------------------------
//Unrecoverable     panic!(msg)  
// fn main(){

//     println!("hello");

//     panic!("Crash");

//     let numbers=[1,2,3];

//     println!("unknown index ={}",numbers[3]);
// }
//---------------------------------------------------------
/* Recoverable  Result
enum Result<T, E>{
    Ok(T),
    Err(E),
}
*/

// fn devide(a:i32 , b:i32) ->Result<i32 ,String>{
//     if b == 0{
//         Err(String::from(" error "))
//     }else {
//         Ok(a/b)
//     }
// }

// fn main() {
//     let result =devide(10, 0);
//     match result {
//         Ok(value) => println!("result is {}",value),
//         Err(msg) =>println!("error is {}",msg),
//     }
// }
//---------------------------------------------------------
// use std::fs::File;
// use std::io::{self , Read};

// fn read_file_content(path :& str)->Result<String , io::Error>{
//     let mut file =File::open(path)?;

//     let mut content =String::new();

//     file.read_to_string(& mut content)?;

//     Ok(content)
// }
// fn main() {
//     match read_file_content("data.txt") {
//         Ok(text) =>println!("content is {}",text),
//         Err(e) => println!("Error reading file : {}",e),
        
//     }   
// }
//---------------------------------------------------------
// use std::fs::File;
// use std::io::Read;

// fn main() {

//     let file1 =File::open("data.txt");

//     match file1 {
//         Ok(mut f) =>{
//             let mut content =String::new();
//             f.read_to_string(& mut content).unwrap();
//             println!(" File1  read :{}",content);
//         }
//         Err(e) =>{
//             println!(" Error reading File1 :{}",e);
//         }
//     }

//     let mut file2 =File::open("data2.txt").expect(" nashod file 2 ro barat baz konam");

//     let mut content2 =String::new();
//     file2.read_to_string(&mut content2).expect("Nashod barat file2 ro bekhoonam");
//     println!("File2 ro khondam:{}",content2);
    
// }
//---------------------------------------------------------
// use  std::fs::File;
// fn main() {
    
//     let file_result = File::open("hello.txt");

//     let a =match file_result {
//         Ok(file) =>file,
//         Err(e) =>panic!("problem opening file {e:?}"),
//     };
// }
//---------------------------------------------------------
//use std::fs::File;
//unwrap = program panic and return default msg
//expect = simillar to unwrap ,program panic and return written msg
// fn main() {
//     // let file = File::open("a.txt").unwrap();
//     // println!("fie open successfully:{:?}",file);


//     let file2=File::open("b.txt").expect("failed to open b.txt");
//     println!("fie open successfully:{:?}",file2);

// }
//---------------------------------------------------------
use std::fs::File;
use std::io::{Read ,Write};


fn main() {
    match File::create("file1.txt") {

        Ok(mut file1) =>{
            file1.write(b"Ali").unwrap();
            println!("File1 written : Ali");
        }
        Err(e) =>{
            println!("Error creating file1.txt :{:?}",e);
        }
    }
    //-------------------------------------------------

    let mut file2=File::create("file2.txt").unwrap();
    file2.write_all(b"Reza").unwrap();
    println!("File2 written : Reza");

    //-------------------------------------------------


    let mut file3=File::create("file3.txt").expect("Expect : Failed to create file3.txt");
    file3.write_all(b"Farbod").expect("Expect : Failed to write to file3.txt");
    println!("File3 written : Farbod");

    //-------------------------------------------------

    match File::open("file4.txt") {
        Ok(_) => println!("file 4 open successfully"),
        Err(_) => panic!("cant  open file4.txt"),
        
    }
    
}


    
