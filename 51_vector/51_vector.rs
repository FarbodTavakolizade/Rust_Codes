/*
Array static   compile time 
let array :[i32;3] =[1,2,3];   or let arr =[1,2,3];

slice reference array , vec  
let arr1=[10,20,30,40];
let slice:&[i32]=&arr1[1..3];   output=20,30


vector dynamic array add(push)  remove(pop)  runtime  vec!
*/
fn main() {
    let v1=vec![1,2,3];
    let v2:Vec<u8>=vec![2,4,6];
    println!("v1 is {:?} and v2  is {:#?}",v1,v2);

    let colors =vec!["blue","red","green"];
    
    println!("first color is {}",colors[0]);
    println!("second color is {:?}",colors.get(1));

    let mut v3=vec![10,20,30,40,50];
    println!("v3 is {:?}",v3);

    //push value in vector (عنصر اصافه میشه به اخر داینامیک ارایه)

    v3.push(12);
    v3.push(14);
    v3.push(16);

    println!("changed vector  v3  is {:?}",v3);

    v3.remove(2); //remove specific index from vector
    println!("v3 pop element  is {:?}",v3);

    let mut index =0;
    for color in colors{
        println!("Index: {} --value:{}",index,color);
        index+=1;


        //creating an empty vector
        let mut vector:Vec<i32>=Vec::new();

        vector.push(10);
        vector.push(20);
        vector.push(30);//index is 2
        vector.remove(2);
        println!("vector is {:?}",vector);

        let mut vector2=vec![0;5];
        println!("vector2 is {:?}",vector2);


        //pop عنصر کم مکنی از اخر داینامیک ارایه

        let mut vector3=vec!['a','b','c'];

        let x=vector3.pop();

        println!("{:?}",x);
        println!("{:?}",vector3);

       let mut vector4:Vec<i32>=Vec::new();
       let y=vector4.pop();
        println!("{:?}",y);

        println!("{:?}",vector4);


        let vector5=[1,2,3,4,5];
        println!("length of vector is {}",vector5.len());

        //insert  add element to specific index

        let mut vector6=vec![1,2,3,4,5,6];
        vector6.insert(3, 10);
        println!("{:?}",vector6);


        let vector7=vec![10,20,30];
        for x in &vector7{
            println!("{}",x);
        }
    }
















}