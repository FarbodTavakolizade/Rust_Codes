/*Cargo.toml should be like this
[package]
name = "manage"
version = "0.1.0"
edition = "2024"

[dependencies]
colored ="2.0.0" */


use std::io;
use colored::*;

#[derive(Debug, Clone)]
struct MenuItem {
    id: u32,
    name: String,
    price: f32,
}

struct Restaurant {
    menu: Vec<MenuItem>,
    next_id: u32,
}

impl Restaurant {
    fn new() -> Self {
        Self {
            menu: Vec::new(),
            next_id: 1,
        }
    }

    fn add_item(&mut self, name: String, price: f32) {
        let item = MenuItem {
            id: self.next_id,
            name,
            price,
        };
        self.menu.push(item);
        self.next_id += 1;
        println!("{}", "✔ Item added successfully!".green());
    }

    fn edit_item(&mut self, id: u32, name: String, price: f32) {
        for item in &mut self.menu {
            if item.id == id {
                item.name = name;
                item.price = price;
                println!("{}", "✏ Item edited successfully!".green());
                return;
            }
        }
        println!("{}", "❌ Item not found with this ID.".red());
    }

    fn delete_item(&mut self, id: u32) {
        if let Some(pos) = self.menu.iter().position(|x| x.id == id) {
            self.menu.remove(pos);
            println!("{}", "🗑 Item deleted successfully!".bright_red());
        } else {
            println!("{}", "❌ No item exists with this ID.".red());
        }
    }

    fn list_items(&self) {
        println!("\n{}", "📋 Menu Items List:".blue());
        for item in &self.menu {
            println!("ID: {} | Name: {} | Price: {:.2}", 
                   item.id.to_string().blue(), 
                   item.name.cyan(), 
                   item.price.to_string().blue());
        }
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt.magenta());
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input");
    input.trim().to_string()
}

fn main() {
    let mut restaurant = Restaurant::new();

    loop {
        println!("\n{}", "--- Restaurant Management Menu ---".bright_blue().bold());
        println!("{}", "1. Add item".green());
        println!("{}", "2. Edit item".green());
        println!("{}", "3. Delete item".red());
        println!("{}", "4. Show menu".blue());
        println!("{}", "5. Exit".green());

        let choice = get_input("Select an option:");

        match choice.as_str() {
            "1" => {
                let name = get_input("Item name:");
                let price_input = get_input("Item price:");
                if let Ok(price) = price_input.parse::<f32>() {
                    restaurant.add_item(name, price);
                } else {
                    println!("{}", "❌ Invalid price.".red());
                }
            }
            "2" => {
                let id_input = get_input("Item ID to edit:");
                let name = get_input("New name:");
                let price_input = get_input("New price:");
                if let (Ok(id), Ok(price)) = (id_input.parse::<u32>(), price_input.parse::<f32>()) {
                    restaurant.edit_item(id, name, price);
                } else {
                    println!("{}", "❌ Invalid input.".red());
                }
            }
            "3" => {
                let id_input = get_input("Item ID to delete:");
                if let Ok(id) = id_input.parse::<u32>() {
                    restaurant.delete_item(id);
                } else {
                    println!("{}", "❌ Invalid ID.".red());
                }
            }
            "4" => {
                restaurant.list_items();
            }
            "5" => {
                println!("{}", "👋 Goodbye!".green());
                break;
            }
            _ => println!("{}", "❌ Invalid option.".red()),
        }
    }
}
