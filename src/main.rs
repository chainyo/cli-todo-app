fn main() {
    loop {
        println!("To-Do List Menu:");
        println!("1. Add a task");
        println!("2. List all tasks");
        println!("3. Mark a task as done");
        println!("4. Delete a task");
        println!("5. Save tasks to a file");
        println!("6. Load tasks from a file");
        println!("7. Exit");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();

        // Attempt to parse the input as a u8, if it fails, print an error message and continue
        let choice = match input.parse::<u8>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please try again.");
                continue;
            },
        };

        match choice {
            1 => {
                println!("Enter a task:");
                let mut task = String::new();
                std::io::stdin().read_line(&mut task).expect("Failed to read input");
                println!("Added task: {}", task);
            },
            2 => println!("List all tasks"),
            3 => println!("Mark a task as done"),
            4 => println!("Delete a task"),
            5 => println!("Save tasks to a file"),
            6 => println!("Load tasks from a file"),
            7 => break,
            _ => println!("Please enter a valid number"),
        }
    }
}
