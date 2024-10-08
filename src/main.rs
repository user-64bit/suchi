use chrono::{Datelike, Timelike};
use std::fmt::Error;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::{env, process};

fn main() {
    let home: String = env::var("HOME").expect("HOME directory not found");
    let suchi_path = format!("{home}/.suchi");

    // initialization of .suchi
    match init(&suchi_path) {
        Ok(_) => println!("initializing...."),
        Err(e) => println!("Error occured: {}", e),
    };

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command = &args[1];
        match &command[..] {
            "show" => println!("show todos, {:?}", &args[2..]),
            "add" => add(&suchi_path, &args[2..]),
            "done" => println!("done todo, {:?}", &args[2..]),
            "delete" => println!("delete todo, {:?}", &args[2..]),
            "edit" => println!("edit todo, {:?}", &args[2..]),
            "clear" => println!("clear todos, {:?}", &args[2..]),
            "--help" | "help" | "-h" | _ => help(),
        }
    } else {
        help();
    }
}

fn init(suchi_path: &String) -> Result<(), Error> {
    // if .suchi doesn't exits then create.
    let _file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .create(true)
        .open(&suchi_path);
    Ok(())
}

fn add(suchi_path: &String, args: &[String]) {
    if args.is_empty() {
        println!("suchi add command takes atleast 1 argument.");
        process::exit(1);
    }

    // if there isn't .suchi file it will create directly
    let suchi_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&suchi_path)
        .expect("Couldn't able to perform action.");

    let now = chrono::Local::now();
    let (hours, minutes, seconds) = (now.hour(), now.minute(), now.second());
    let (year, month, day) = (now.year(), now.month(), now.day());

    let mut buffer = BufWriter::new(suchi_file);
    for arg in args {
        if arg.trim().is_empty() {
            continue;
        }
        let line = format!(
            "[{}-{}-{} {}:{}:{}] {}\n",
            year, month, day, hours, minutes, seconds, arg
        );
        buffer
            .write_all(line.as_bytes())
            .expect("Couldn't able to write")
    }
}

const HELP: &str = r#"
Usage: suchi [COMMAND] [OPTIONS]

suchi is your fast, simple, and efficient task organizer written in Rust!

### Quick Start Example:
    suchi show

### Available Commands:

- add [TASKs]
    Add a new task/tasks to your list.
    Examples: 
	suchi add "Take a break and stretch" "Watch Next Episode of One Peace"
			OR
	suchi add bat ball cat apple

- edit [INDEX] [UPDATED TASK]
    Update an existing task by its index.
    Example: suchi edit 1 "Skip the break, let's push forward!"

- show
    Display all your tasks.
    Example: suchi show

- done [INDEXs]
    Mark a task/tasks as complete by its index.
    Example: suchi done 1 2 (marks the first and second tasks as completed)

- delete [INDEXs]
    Remove a task/tasks by its index.
    Example: suchi delete 4 5 (removes the fourth and fifth task)

- clear
    Remove all tasks in one go.
    Example: suchi clear

Pro Tip: Keep your tasks organized and stay productive with `suchi`!
"#;

fn help() {
    println!("{}", HELP);
}
