use std::collections::HashMap;
use std::io;

enum List {
    Company,
    Department(String),
}

enum Command {
    Add {
        employee: String,
        department: String,
    },
    List(List),
    Stop,
    Err,
}

impl Command {
    fn parse_command(command: &str) -> Self {
        if command.starts_with("Add") {
            let parts = command.split(" ").skip(1).take(3).collect::<Vec<_>>();
            if parts.len() != 3 || parts[1] != "to" {
                return Command::Err;
            };
            Command::Add {
                employee: parts[0].to_owned(),
                department: parts[2].to_owned(),
            }
        } else if command.starts_with("List") {
            let parts = command.split(" ").skip(1).take(2).collect::<Vec<_>>();
            if parts.len() == 1 && parts[0] == "company" {
                Command::List(List::Company)
            } else if parts.len() == 2 && parts[0] == "department" {
                Command::List(List::Department(parts[1].to_owned()))
            } else {
                Command::Err
            }
        } else if command == "Stop" {
            Command::Stop
        } else {
            Command::Err
        }
    }
}

fn main() {
    println!("Welcome to the company management program!");

    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    let stdin = io::stdin();
    let mut command = String::new();
    loop {
        println!("\nGive a command:");
        stdin.read_line(&mut command).expect("Error reading input.");

        match Command::parse_command(command.trim()) {
            Command::Add {
                employee,
                department,
            } => {
                let department = company.entry(department).or_default();
                department.push(employee);
            }
            Command::List(list) => {
                match list {
                    List::Company => {
                        println!("Company:");
                        let mut departments = company.iter_mut().collect::<Vec<_>>();
                        departments.sort_unstable_by(|a, b| a.0.cmp(b.0));
                        for department in departments {
                            list_department(department.0, department.1);
                        }
                    }
                    List::Department(department) => {
                        if let Some(employees) = company.get_mut(&department) {
                            list_department(&department, employees);
                        } else {
                            println!("The department '{}' doesn't exist.", &department);
                        }
                    }
                };
            }
            Command::Stop => {
                println!("\nThanks for using the company management program!");
                break;
            }
            Command::Err => {
                println!(
                    "\nPlease enter a valid command. Syntax:\nAdd <employee> to <department>\nList department <department>\nList company\nStop\n"
                );
            }
        };

        command.clear();
    }
}

fn list_department(name: &str, employees: &mut [String]) {
    // sort() is preferred over sort_unstable() because the vector is likely still partially sorted from the previous time it was sorted.
    employees.sort();
    println!("{}:\n{}\n", name, employees.join("\n"));
}
