//hashset    simillar to a set
use std::collections::HashSet;
fn main() {
   let mut a:HashSet<String> =HashSet::new();
   println!("hashset is {:?}",a);

   //Add value   insert()

   let mut  colors  :HashSet<&str> =HashSet::new();
    colors.insert("red");
    colors.insert("blue");
    colors.insert("green");
    colors.insert("yellow");
    colors.insert("black");

    //len()
    println!("len is {}",colors.len());

    //is empty
    println!(" empty?  {}",colors.is_empty());

    //clear() removes all elements from hashset
    //  colors.clear();
    //     println!("colors after clear is  {:?}",colors);
    

    println!("colors = {:?}",colors);


    //check value is present in hashset   contains()

    if colors.contains("red"){
        println!("we have red in our hashset");
    }

    //remove values from hashset   remove()
        colors.remove("black");
        println!("colors after remove is  {:?}",colors);

    //iterate over values in hashset


    for x in colors{
        println!("{}",x);
    }


    let numbers =HashSet::from([1,2,3,4]);

    println!("numbers are {:?}",numbers);
    

   
let hashset1 =HashSet::from([2,4,6,7]);
let hashset2 =HashSet::from([3,5,7]);

//union of hashsets

let result:HashSet<_> =hashset1.union(&hashset2).collect();

println!("hashset1 is {:?}",hashset1);
println!("hashset2 is {:?}",hashset2);
println!("union of hashsets are {:?}",result);

//differenece 

let result2:HashSet<_>=hashset2.difference(&hashset1).collect();
println!("hashset1 is {:?}",hashset1);
println!("hashset2 is {:?}",hashset2);
println!("difference of hashsets are {:?}",result2);

//symmetric_differenece 


let hashset_a=HashSet::from([2,7,8]);
let hashset_b=HashSet::from([1,2,7,9]);

let result3 :HashSet<_>=hashset_a.symmetric_difference(&hashset_b).collect();

println!("hashset_a is {:?}",hashset_a);
println!("hashset_b is {:?}",hashset_b);
println!("symmetric_difference of hashsets are {:?}",result3);



//intersection between two hashsets

let hashset_c=HashSet::from([2,7,8]);
let hashset_d=HashSet::from([1,2,7]);

let result4: HashSet<_>=hashset_c.intersection(&hashset_d).collect();

println!("hashset_c is {:?}",hashset_c);
println!("hashset_d is {:?}",hashset_d);
println!("intersection of hashsets are {:?}",result4);





    



}