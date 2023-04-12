use crossterm::{
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io::{BufRead, Write};

fn main() {

    struct Task {
        name: String,
        done: bool,
    }

    let mut tasks: Vec<Task> = Vec::new();
    let menu_choices: Vec<&str> = vec![
        "1️⃣ Add a task",
        "2️⃣ Mark a task as done",
        "3️⃣ Delete a task",
        "4️⃣ Save tasks to a file",
        "5️⃣ Load tasks from a file",
        "6️⃣ Exit"
    ];

    fn list_all_tasks(tasks: &Vec<Task>) {
        for task in tasks.iter() {
            println!("* {} {}", task.name, if task.done { "✅" } else { "" });
        };
    }

    fn save_tasks_to_file(tasks: &Vec<Task>, file_name: &str) {
        let file = std::fs::File::create(file_name).expect("Failed to create file");
        let mut writer = std::io::BufWriter::new(file);
        for task in tasks.iter() {
            writer
                .write_all(format!("{} {}\n", task.name, if task.done { "✅" } else { "" })
                .as_bytes())
                .expect("Failed to write to file");
        }
    }

    fn load_tasks_from_file(file_name: &str) -> Vec<Task> {
        // Check if the file exists
        if !std::path::Path::new(file_name).exists() {
            println!("File does not exist, please try again.");
            return Vec::new();
        }
        let file = std::fs::File::open(file_name).expect("Failed to open file");
        let reader = std::io::BufReader::new(file);
        let mut tasks: Vec<Task> = Vec::new();
        for line in reader.lines() {
            let line = line.expect("Failed to read line");
            if "✅" == line.split("✅").next().unwrap() {
                tasks.push(Task {
                    name: line.split("✅").next().unwrap().to_string(),
                    done: true,
                });
            } else {
                tasks.push(Task {
                    name: line,
                    done: false,
                });
            }
        }
        tasks
    }

    fn clear_terminal() {
        let mut stdout = std::io::stdout();
        stdout
            .execute(Clear(ClearType::All))
            .expect("Failed to clear the terminal");
    }
    
    clear_terminal();
    loop {
        println!("[Task Manager - What would you like to do?]\n");
        println!("Current Tasks ({}):", tasks.len());
        if tasks.len() > 0 {
            list_all_tasks(&tasks);
        }
        println!("\n{}\n", menu_choices.join("\n"));
        
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
                clear_terminal();
            },
            2 => {
                println!("Enter the number of the task to mark as done:");
                let mut task_number: String = String::new();
                std::io::stdin().read_line(&mut task_number).expect("Failed to read input");
                let task_number: &str = task_number.trim();
                let task_number = match task_number.parse::<usize>() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input, please try again.");
                        continue;
                    },
                };
                if task_number > tasks.len() {
                    println!("Invalid task number, please try again.");
                    continue;
                }
                tasks[task_number - 1].done = true;
                clear_terminal();
            },
            3 => {
                println!("Enter the number of the task to delete:");
                let mut task_number: String = String::new();
                std::io::stdin().read_line(&mut task_number).expect("Failed to read input");
                let task_number: &str = task_number.trim();
                let task_number = match task_number.parse::<usize>() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input, please try again.");
                        continue;
                    },
                };
                if task_number > tasks.len() {
                    println!("Invalid task number, please try again.");
                    continue;
                }
                tasks.remove(task_number - 1);
                clear_terminal();
            },
            4 => {
                println!("Enter a file name:");
                let mut file_name: String = String::new();
                std::io::stdin().read_line(&mut file_name).expect("Failed to read input");
                let file_name = file_name.trim().to_string();
                save_tasks_to_file(&tasks, &file_name);
                clear_terminal();
            },
            5 => {
                println!("Enter a file name:");
                let mut file_name: String = String::new();
                std::io::stdin().read_line(&mut file_name).expect("Failed to read input");
                let file_name = file_name.trim().to_string();
                tasks = load_tasks_from_file(&file_name);
                clear_terminal();
            },
            6 => {
                clear_terminal();
                println!("Goodbye! 👋");
                break;
            },
            _ => println!("Please enter a valid number"),
        }
    }
}
