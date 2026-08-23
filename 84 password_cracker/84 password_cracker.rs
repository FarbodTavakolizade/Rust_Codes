/*
use cargo add sha2  to import library

add a passwordexample.txt file   with custom passwords could be downloaded in https://github.com/shawntns/top-100-worst-passwords/blob/master/dic.txt


you can search sha256 convertor in your browser in order to test a password hash

*/

#![allow(unused)]
use std::collections::hash_set::ExtractIf;
use std::{env,str,vec};
use std::fs::File;
use sha2::{Sha256,Digest};
use std::process::exit;
use std::io::{BufRead,BufReader};
fn main(){
    let args:Vec<String>=env::args().collect(); // cargo run <hash>

    if args.len() !=2{
        println!("Invalid amount of argument");
        println!("Correct form is cargo run <Hash>");
        exit(1);
    }
    let wanted_hash=&args[1];
    let password_file="src/passwordexample.txt";
    let mut attempts=1;

    println!("Attempting to hack :{}!\n",wanted_hash);
    let password_list=File::open(password_file).unwrap();
    let reader=BufReader::new(password_list);

    for line in reader.lines(){
        let line =line.unwrap();
        let password =line.trim().to_owned().into_bytes();

        let password_hash=format!("{:x}",Sha256::digest(&password)); // "hello" => ajsnjhghs566589debhgud
        println!("[{}] {} =={}",attempts,std::str::from_utf8(&password).unwrap(),password_hash);

        if &password_hash == wanted_hash{
            println!("password is {}",std::str::from_utf8(&password).unwrap());
            exit(0);
        }
        attempts+=1;
    }
    println!("password hash not found");

}