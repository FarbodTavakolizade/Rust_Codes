
fn main() {
    let mut arr=[1,2,3];
    {
        let element =&mut arr[1];
        *element *=4;
    }
    println!("{:?}", arr);
    // The above code will print [1, 8, 3]


    let mut arr2=[2,4,6,8];
    change_elem(&mut arr2[2]);
    println!("{:?}", arr2);

}

fn change_elem(elem:&mut i32){
    *elem *= 2 ;
}







