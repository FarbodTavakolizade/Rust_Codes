//HashMap  [key , value]  [usernumber , userid]
//[1,"1564432154"]  key  unique  values can be duplicated
use std::collections::HashMap;
fn main() {
    let mut user :HashMap<i32,String> =HashMap::new();
    println!("HashMap is {:?}",user); //output = HashMap is {}

    //Add elements to HashMap

    user.insert(1,String::from("214544251258"));
    user.insert(2,String::from("454845454545"));
 
    

    println!("HashMap is {:?}",user);

    //Access Values in HashMap  with .get(&key) method

    let first_user=user.get(&1);
    let second_user=user.get(&2);
    let third_user=user.get(&3);

    println!("first-user is {:?}",first_user);
    println!("second_user is {:?}",second_user);
    println!("third-user is {:?}",third_user);

    //Remove

    // user.remove(&1);
    // println!("HashMap is {:?}",user);


    // Change/Update elements

    user.insert(1, String::from("21621155"));
        println!("HashMap  after update is is {:?}",user);


     
    println!("number of elements in hash map is {}",user.len());

    let mut x=HashMap::new();
    x.insert("bmw", 1);
    x.insert("benz", 2);

    println!("does bmw exist? {}",x.contains_key("bmw"));


    for (k,v) in x.iter(){
        println!("{} -> {}",k,v);
    }

    for k in x.keys(){
        println!("key :{}",k);
    }

    for v in x.values(){
        println!("value: {}",v);
    }

    let copy=x.clone();
    println!("len is {}",copy.len());

    

    for user in user.keys(){
        println!("{}",user);
    }




}














