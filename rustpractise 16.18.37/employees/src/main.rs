use std::collections::HashMap;
use std::io::{self, Write};
use std::str::SplitWhitespace;

fn main() {
    println!("Welcome to the department interface. Please run a command:");
    help();

    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        print_promt();
        let command = get_command();
        let mut command_iter = command.split_whitespace();

        match command_iter.next() {
            Some("add") => match process_add(&mut command_iter) {
                Ok((employee, department)) => {
                    let mut employees = departments.entry(department).or_insert(Vec::new());
                    employees.push(employee);
                }
                Err(err) => println!("Error: {}", err),
            },
            Some("help") => help(),
            Some("list") => println!("{:#?}", &departments),
            Some("exit") => break,
            Some(_) => println!("{}: Command unknown. Try `help`", command),
            None => panic!("We shouldn't be here"),
        }
    }
}

fn help() {
    println!("help: show this help.");
    println!("exit: exits this program.");
    println!("show: shows the deparment structure.");
    println!("add: adds an employee to a department");
    println!("     - usage: `add <employee name> to <department_name>`");
}

fn print_promt() {
    print!("> ");
    io::stdout().flush().ok().expect("Couldn't flush stdout!");
}

fn get_command() -> String {
    let mut command = String::new();
    io::stdin()
        .read_line(&mut command)
        .expect("Couldn't read command!");
    command
}

fn process_add<'a>(command: &mut SplitWhitespace) -> Result<(String, String), &'a str> {
    let mut employee = String::new();
    let mut department = String::new();
    while let Some(word) = command.next() {
        if word == "to" {
            break;
        }

        if employee.len() > 0 {
            employee.push_str(" ");
        }
        employee.push_str(word)
    }

    for word in command {
        if department.len() > 0 {
            department.push_str(" ");
        }
        department.push_str(word);
    }

    if employee.len() > 0 && department.len() > 0 {
        Ok((employee, department))
    } else {
        Err("Could not parse command. Try `add <employee name> to <department name>")
    }
}
