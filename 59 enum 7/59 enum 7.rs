#[derive(Clone, Copy)]
enum CreatureType{
    Human,
    Monster,
    Robot,
}

struct Creature {
    name: String,
    health : i32,  //hp
    ctype :CreatureType,
}

impl Creature{
    fn new(name: &str , health: i32 ,ctype:CreatureType) ->Self{
        Self { 
                name: name.to_string(),
                health,
                ctype,
         }
    }

    fn describe(&self){
        match &self.ctype{
            CreatureType::Human => println!(" {} is a human with {} hp",self.name,self.health),
            CreatureType::Monster => println!("{} is a monster with {} hp",self.name, self.health),
            CreatureType::Robot => println!("{} is a robot with {} hp",self.name, self.health),
        }
    }

    fn attack(&self , target :&mut Creature){
        let damage = match self.ctype{
            CreatureType::Human =>5,
            CreatureType::Monster => 10,
            CreatureType::Robot =>7,
        };

        println!("{} attacks {} for {} damage!",self.name , target.name , damage);
        target.take_damage(damage);
    }

    fn take_damage(&mut self , amount:i32){
        self.health -= amount;
        if self.health <= 0 {
            println!(" {} has been defeated",self.name);
        }else {
            println!("{} now has {} HP  left",self.name,self.health)
        }
    }
}


fn main() {
    let mut reza =Creature::new("Reza", 100, CreatureType::Human);

    let mut monster =Creature::new("monster", 150,CreatureType::Monster);

    let mut bot =Creature::new("bot", 125,CreatureType::Robot);

    reza.describe();
    monster.describe();
    bot.describe();



    println!("-----Battle Start!-----");

    reza.attack(& mut monster);
    monster.attack(& mut reza);
    bot.attack(&mut monster);
    
}