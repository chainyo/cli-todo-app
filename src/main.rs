fn main() {

    struct Task {
        name: String,
        done: bool,
    }

    loop {
        println!("To-Do List Menu:");
        println!("1. Add a task");
        println!("2. List all tasks");
        println!("3. Mark a task as done");
        println!("4. Delete a task");
        println!("5. Save tasks to a file");
        println!("6. Load tasks from a file");
        println!("7. Exit");

        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let input: &str = input.trim();

        // Attempt to parse the input as a u8, if it fails, print an error message and continue
        let choice = match input.parse::<u8>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please try again.");
                continue;
            },
        };

        let mut tasks: Vec<Task> = Vec::new();

        match choice {
            1 => {
                println!("Enter a task:");
                let mut task_text: String = String::new();
                std::io::stdin().read_line(&mut task_text).expect("Failed to read input");
                let task_text = task_text.trim().to_string();
                let task = Task {
                    name: task_text,
                    done: false,
                };
                tasks.push(task);
                println!("Added task: {}", tasks.last().unwrap().name);
            },
            2 => {
                println!("Tasks:");
                for (index, task) in tasks.iter().enumerate() {
                    println!("{}. {} [{}]", index + 1, task.name, if task.done { "✅" } else { "⭕" });
                };
            }
            3 => println!("Mark a task as done"),
            4 => println!("Delete a task"),
            5 => println!("Save tasks to a file"),
            6 => println!("Load tasks from a file"),
            7 => break,
            _ => println!("Please enter a valid number"),
        }
    }
}
