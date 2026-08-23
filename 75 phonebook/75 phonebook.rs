
use std::io;
use colored::*;

#[derive(Clone, Debug)]
struct Contact {
    name: String,
    phone: String,
}

impl Contact {
    fn new(name: &str, phone: &str) -> Self {
        Self {
            name: name.to_string(),
            phone: phone.to_string(),
        }
    }
}

struct PhoneBook {
    contacts: Vec<Contact>,
}


impl PhoneBook {
    fn new() -> Self {
        Self { contacts: Vec::new() }
    }

    fn add_contact(&mut self, name: &str, phone: &str) {
        if self.contacts.iter().any(|c| c.name == name) {
            println!("{}", format!("❌ Contact '{}' already exists!", name).red());
        } else {
            self.contacts.push(Contact::new(name, phone));
            println!("{}", "✅ Contact added successfully!".green());
        }
    }

    fn delete_contact(&mut self, name: &str) {
        let original_len = self.contacts.len();
        self.contacts.retain(|c| c.name != name);
        if self.contacts.len() < original_len {
            println!("{}", format!("🗑️ Contact '{}' deleted!", name).yellow());
        } else {
            println!("{}", format!("❌ Contact '{}' not found!", name).red());
        }
    }

    fn edit_contact(&mut self, name: &str, new_phone: &str) {
        match self.contacts.iter_mut().find(|c| c.name == name) {
            Some(contact) => {
                contact.phone = new_phone.to_string();
                println!("{}", format!("✏️ Contact '{}' updated!", name).blue());
            }
            None => println!("{}", format!("❌ Contact '{}' not found!", name).red()),
        }
    }

    fn search_contact(&self, name: &str) {
        match self.contacts.iter().find(|c| c.name == name) {
            Some(c) => println!("{}", format!("🔍 Found: {} -> {}", c.name, c.phone).green()),
            None => println!("{}", format!("❌ Contact '{}' not found!", name).red()),
        }
    }

    fn list_contacts(&self) {
        if self.contacts.is_empty() {
            println!("{}", "📭 No contacts available.".yellow());
        } else {
            println!("{}", "📇 Contacts:".cyan().bold());
            for contact in &self.contacts {
                println!("{} -> {}", contact.name.green(), contact.phone.blue());
            }
        }
    }
}


enum Command {
    Add,
    Delete,
    Edit,
    Search,
    List,
    Exit,
    Invalid,
}

impl Command {
    fn from_input(choice: &str) -> Self {
        match choice {
            "1" => Command::Add,
            "2" => Command::Delete,
            "3" => Command::Edit,
            "4" => Command::Search,
            "5" => Command::List,
            "6" => Command::Exit,
            _ => Command::Invalid,
        }
    }
}

fn input(prompt: &str) -> String {
    let mut line = String::new();
    print!("{}", format!("{} ", prompt).magenta().bold());
    io::Write::flush(&mut io::stdout()).expect("Flush failed");
    io::stdin().read_line(&mut line).expect("Failed to read line");
    line.trim().to_string()
}

fn main() {
    let mut phonebook = PhoneBook::new();

    loop {
        println!("{}", "\n================ PhoneBook Menu =================".bright_blue().bold());
        println!("{}", "1️⃣  Add Contact".cyan());
        println!("{}", "2️⃣  Delete Contact".red());
        println!("{}", "3️⃣  Edit Contact".blue());
        println!("{}", "4️⃣  Search Contact".green());
        println!("{}", "5️⃣  List Contacts".yellow());
        println!("{}", "6️⃣  Exit".bright_red());
        println!("{}", "================================================".bright_blue().bold());

        let choice = input("Enter your choice:");
        let command = Command::from_input(&choice);

        match command {
            Command::Add => {
                let name = input("Enter contact name:");
                let phone = input("Enter contact phone:");
                phonebook.add_contact(&name, &phone);
            }
            Command::Delete => {
                let name = input("Enter contact name to delete:");
                phonebook.delete_contact(&name);
            }
            Command::Edit => {
                let name = input("Enter contact name to edit:");
                let phone = input("Enter new phone number:");
                phonebook.edit_contact(&name, &phone);
            }
            Command::Search => {
                let name = input("Enter contact name to search:");
                phonebook.search_contact(&name);
            }
            Command::List => phonebook.list_contacts(),
            Command::Exit => {
                println!("{}", "👋 Exiting PhoneBook. Goodbye!".bright_magenta().bold());
                break;
            }
            Command::Invalid => {
                println!("{}", "⚠️ Invalid choice, please try again.".red());
            }
        }
    }
}