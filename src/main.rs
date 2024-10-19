use std::io::{self, Write};

fn main() {
    let mut todo: Vec<String> = Vec::new();

    println!("Welcome to the todo list application!");

    loop {
        println!("\nWhat would you like to do?");
        println!("1. Add an item");
        println!("2. Remove an item");
        println!("3. View the list");
        println!("4. Quit");

        print!("Enter your choice (1-4): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                print!("Enter the item to add: ");
                io::stdout().flush().unwrap();
                let mut item = String::new();
                io::stdin().read_line(&mut item).unwrap();
                add_item(&mut todo, item.trim().to_string());
                println!("Item added successfully!");
            },
            "2" => {
                print!("Enter the index of the item to remove: ");
                io::stdout().flush().unwrap();
                let mut index = String::new();
                io::stdin().read_line(&mut index).unwrap();
                if let Ok(i) = index.trim().parse::<usize>() {
                    remove_item(&mut todo, i - 1);
                    println!("Item removed successfully!");
                } else {
                    println!("Invalid index!");
                }
            },
            "3" => {
                println!("Your current todo list is:");
                get_list(&todo);
            },
            "4" => {
                println!("Thank you for using the todo list application!");
                break;
            },
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn add_item(todo: &mut Vec<String>, item: String) {
    todo.push(item);
}

fn remove_item(todo: &mut Vec<String>, index: usize) {
    if index < todo.len() {
        todo.remove(index);
    }
}

fn get_list(todo: &Vec<String>) {
    for (index, item) in todo.iter().enumerate() {
        println!("{}. {}", index + 1, item);
    }
}
