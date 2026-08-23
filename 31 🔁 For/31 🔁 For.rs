fn main() {
    //for

        //copy
    let numbers=[1,2,3,4,5];

    for item in numbers{
        println!("{item}");
    }

    println!("{numbers:#?}");











    for i in 20..31{
        println!("the number is {i}");
    }

    let colors =["red","green","blue"];

    for j in 0..colors.len(){
        println!("color name is {}",colors[j]);
    }
//------------------------------------------------------------
    //moved ownership
let arr =[String::from("a"),String::from("b")];

for s in arr{

    println!("{}",s);
}



let arr2 =[String::from("c"),String::from("d")];

for s2 in arr2.iter(){

    println!("{s2}");
}

println!("{arr2:?}");



let mut x =[1,2,3];

for values in x.iter_mut(){
    *values+=10;
}
println!("{x:?}");

//moved ownership ==>into_iter()
let array=[String::from("hello"),String::from("world")];

for items in array.into_iter(){
    println!("{items}");
}
//---------------------------------------------------------

let slice=&[1,2,3,4,5,6];

for x in slice{
    println!("{x}");
}

println!("{slice:?}");


let slice2=&[String::from("e"),String::from("f")];

for j in slice2.iter(){
    println!("{j}");
}

println!("{slice2:?}");



let mutable_slice =&mut[2,4,6];

for x in mutable_slice.iter_mut(){
    *x+=10;
}
println!("{mutable_slice:?}");




}


























