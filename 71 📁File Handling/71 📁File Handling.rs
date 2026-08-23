// //File Handling
// use std::fs::File;
// fn main(){
//     //Open File  in read only mode

//     let data_result =File::open("data.txt");
//     let data_file =match data_result {
//         Ok(file) =>file,
//         Err(error) => panic!("problem opening data file : {:?}",error),
//     };

//     println!("Data file is {:?}" ,data_file);

    
// }
//----------------------------------------------------------------
// use std::fs::File;
// use std::io::Read;
// fn main(){
//     //read a file in local file system
//     let mut data_file =File::open("data.txt").unwrap();

//     //create an empty mutable string

//     let mut file_content =String::new();

//     data_file.read_to_string(&mut file_content).unwrap();

//     println!("file content is :{:?}",file_content);

//}

//----------------------------------------------------------------
// use std::fs::File;
// use std::io::Write;
// fn main() {
//     //create file 

//     let mut data_file =File::create("data.txt").expect("creation failed");


//     //write contents to file
//     data_file.write("hello".as_bytes()).expect("write operation failed");

//     println!("Created a file data.txt");
// }

//----------------------------------------------------------------

//appending content to  file
// use std::fs::OpenOptions;
// use std::io::Write;
// fn main() {

//     let mut data_file =OpenOptions::new()
//         .append(true)
//         .open("data.txt")
//         .expect("cant open file");

//     //write to a file
//     data_file
//         .write("\t my name is farbod".as_bytes())
//         .expect("write operation failed");

//     println!("appended content to a file");   
//}

//----------------------------------------------------------------
//remove 


use std::fs;
fn main(){

    fs::remove_file("data.txt").expect("cant remove the file");
    

}
