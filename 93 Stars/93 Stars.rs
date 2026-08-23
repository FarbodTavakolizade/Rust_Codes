fn main(){
    right_aligned_triangle(6);
    triangle(7);
    diamond(7);
    hollow_square(7);
    pascal(10);
}

fn right_aligned_triangle(n:u32){
    for i in 1..=n{
        //printing spaces
        for _ in 0..(n-i){
            print!(" ");
        }
        for _ in 0..i{
            print!("*");
        }
        println!();
    }
}


fn triangle(n:u32){
    for i in 1..=n{
        //printing spaces
        for _ in 0..(n-i){
            print!(" ");
        }
        //printing stars
        for _ in 0..(2*i-1){
            print!("*");
        }
        println!();
    }
}

fn diamond(n:u32){
    //up
    for i in 0..n{
        //space
        for _ in 0..(n-i-1){
            print!(" ");
        }

        //stars
        for _ in 0..(2*i+1){
            print!("*");
        }
        println!();
    }


        //down

        for i in (0..n-1).rev(){
            //space
            for _ in 0..(n-i-1){
                print!(" ");
            }
            //stars
            for _ in 0..(2*i+1){
                print!("*");
            }
            println!()
        }
    }

    fn hollow_square(n:u32){
        for i in 0..n{
            for j in 0 ..n{
                if i==0||i==n-1||j==0||j==n-1{
                    print!("* ");
                }else {
                    print!("  ");
                }
            }
            println!();
        }

            
    }
fn pascal(n:u32){
    for i in 0..n{
        for _ in 0..(n-i-1){
            print!(" ");
        }
        let mut value=1;
        for j in 0..=i{
            print!("{:4}",value);
            value=value*(i-j)/(j+1);

        }
        println!();
    }
}





