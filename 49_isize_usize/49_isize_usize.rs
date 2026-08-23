//isize (integer size  32:4byte  64 :8byte)  usize(unsigned size  32:4byte   64:8byte)

//usize index of arrays and memory 
//isize pointer difference
fn main() {
    let x:isize=-50;
    println!("{}",x);

    let len_of_array:usize=10;
    println!("{}",len_of_array);

    let arr=[10,20,30,40];
    let index:usize=2;
    println!("value is {}",arr[index]);

    let offset:isize=-1;
    println!("offset ={}",offset);


    let arr2=[10,20,30,40,50];
    let index2:usize=2;
    println!("arr[{}] ={}",index2,arr[index2]);

    let offset2:isize=-1;
    let new_index=index2 as isize+offset;
    println!("new index is {}",new_index);
}