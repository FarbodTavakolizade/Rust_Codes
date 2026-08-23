use std::string;

//Enum (enumeration) variant [enum value]
enum Direction{
    Up,  //variant
    Down,
    Left,
    Right,
}


fn main() {
    let dir =Direction::Left;
    match dir{
        Direction::Up => println!("up"),
        Direction::Down => println!("down"),
        Direction::Left => println!("left "),
        Direction::Right => println!("right"),
    }

//----------------------------------------------------------------------
    #[derive(Debug)]
    enum Geo_Direction{
        North,
        South,
        East,
        West,
    }
    //initialize and access enum variants
    let north =Geo_Direction::North;
    let south =Geo_Direction::South;
    let east =Geo_Direction::East;
    let west =Geo_Direction::West;

    println!("{:?}",north);
    println!("{:?}",south);
    println!("{:?}",east);
    println!("{:?}",west);

// -------------------------------------------------------------------


    #[derive(Debug)]
    enum Result{
        Score(f64),
        Valid(bool),
    }

    let num =Result::Score(3.14);
    let bool =Result::Valid(true);


    println!("num is {:?}",num);
    println!("bool is  {:?}",bool);
// -------------------------------------------------------------------


    #[derive(Debug)]
    enum Game {
        Quit,
        Print(String),
        Position {x:i32 , y:i32},
        ChangeBackground(i32 ,i32 ,i32),
    }

    let quit =Game::Quit;
    let print =Game::Print(String::from("hello world"));
    let position = Game::Position { x: 10, y: 10 };
    let color =Game::ChangeBackground(200, 255, 255);

    println!("quit = {:?}",quit);
    println!("print = {:?}",print);
    println!("position = {:?}",position);
    println!("color = {:?}",color);


    



}