#[derive(Debug)]
enum Gender {
    Male,
    Female,
}
#[derive(Debug)]
enum Ip {
    V4,
    V6,
}
#[derive(Debug)]
enum Weekday {
    Saturday,
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}
#[derive(Debug)]
struct Usage {
    gender: Gender,
    ip: Ip,
    weekday: Option<Weekday>,
}
impl Usage {
    fn new(gender: Gender, ip: Ip, weekday: Option<Weekday>) -> Self {
        Self {
            gender,
            ip,
            weekday,
        }
    }

    fn describe_usage(&self) {
        match (&self.gender, &self.ip, &self.weekday) {
            (Gender::Male, Ip::V6, Some(day)) => {
                println!("male in  {:?} use ipv6", day);
            }
            (Gender::Male, Ip::V4, Some(day)) => {
                println!("male in  {:?} use ipv4", day);
            }

            (Gender::Female, Ip::V4, Some(day)) => {
                println!("female in  {:?} use ipv4", day);
            }
            (Gender::Female, Ip::V6, Some(day)) => {
                println!("female in  {:?} use ipv6", day);
            }
            (_, _, None) => {
                println!(
                    "day is unknown but gender is {:?} and ip is {:?}",
                    self.gender, self.ip
                );
            }
        }
    }
}

fn check_if_ipv6(usage: Usage){
    if let Ip::V6 =usage.ip{
        println!("{:?} user use IPv6 in day: {:?}",usage.gender , usage.weekday);
    }
}



fn main() {
    let user_one = Usage::new(Gender::Male, Ip::V4, Some(Weekday::Monday));

    let user_two =Usage::new(Gender::Female, Ip::V6, Some(Weekday::Friday));

    let user_three =Usage::new(Gender::Male, Ip::V6, None);

    user_one.describe_usage();
    user_two.describe_usage();
    user_three.describe_usage();


    check_if_ipv6(user_one);
    check_if_ipv6(user_three);
    check_if_ipv6(user_two);
    


}
