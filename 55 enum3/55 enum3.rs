// enum Animal{
//     Dog(String),
//     Cat(String),
//     Bird,
// }

// fn describe_animal(animal:Animal){
//     match animal {
//         Animal::Dog(name) =>println!("{}",name),
//         Animal::Cat(name) =>println!("{}",name),
//         _=> println!("it is a bird"),
//     }
// }
// fn main() {
//     let a =Animal::Dog(String::from("abc"));

//     let b =Animal::Cat(String::from("def"));

//     let c =Animal::Bird;

//     describe_animal(a);
//     describe_animal(b);
//     describe_animal(c);
// }
//------------------------------------------------------------------------------------
#[derive(Debug)]
enum Status {
    Alive(i32),
    Dead,
    Paused,
}

#[derive(Debug)]
struct Player {
    name: String,
    status: Status,
}
fn show_state(player: &Player) {
    match &player.status {
        Status::Alive(score) => println!("player {} is alive with score {}", player.name, score),
        Status::Dead => println!("player {} is dead", player.name),
        Status::Paused => println!("player {} paused the game", player.name),
    }
}

fn kill(player: &mut Player) {
    player.status = Status::Dead;
}

fn main() {
    let mut p = Player {
        name: String::from("reza"),
        status: Status::Alive(10),
    };

    show_state(&p);
    kill(&mut p);
    show_state(&p);
}
