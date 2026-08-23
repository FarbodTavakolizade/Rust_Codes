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
impl Player {
    fn show_state(&self) {
        match &self.status {
            Status::Alive(score) => {
                println!(
                    "player {} is alive with amount of {} score",
                    self.name, score
                );
            }
            Status::Dead => {
                println!("player {} is dead", self.name);
            }

            Status::Paused =>{
                println!("game for player {} is paused",self.name);
            }
        }
    }


    fn die(& mut self){
        self.status =Status::Dead;
    }
    fn add_score(&mut self , points: i32){
        if let Status::Alive(ref mut score) =self.status{
            *score +=points;
        }
    }
}

fn main() {
    let mut p =Player{
        name: String::from("farbod"),
        status: Status::Alive(100),
    };

    p.show_state();
    p.add_score(100);
    p.show_state();
}
