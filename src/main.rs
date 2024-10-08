use std::env;
use std::fmt::Error;
use std::fs::OpenOptions;

fn main() {
    // initialization of .suchi
    match init() {
        Ok(_) => println!(".suchi file created at $HOME"),
        Err(e) => println!("Error occured: {}", e),
    };

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command = &args[1];
        match &command[..] {
            "show" => println!("show todos, {:?}", &args[2..]),
            "add" => println!("add todo, {:?}", &args[2..]),
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

fn init() -> Result<(), Error> {
    let home: String = env::var("HOME").expect("HOME directory not found");
    let suchi_path = format!("{home}/.suchi");
	// if .suchi doesn't exits then create.
    let _file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .create(true)
        .open(suchi_path);
    Ok(())
}

const HELP: &str = r#"
Usage: suchi [COMMAND] [OPTIONS]

suchi is your fast, simple, and efficient task organizer written in Rust!

### Quick Start Example:
    suchi show

### Available Commands:

- add [TASK]
    Add a new task to your list.
    Example: suchi add "Take a break and stretch"

- edit [INDEX] [UPDATED TASK]
    Update an existing task by its index.
    Example: suchi edit 1 "Skip the break, let's push forward!"

- show
    Display all your tasks.
    Example: suchi show

- done [INDEX]
    Mark a task as complete by its index.
    Example: suchi done 1 (marks the first task as completed)

- delete [INDEX]
    Remove a task by its index.
    Example: suchi delete 4 (removes the fourth task)

- clear
    Remove all tasks in one go.
    Example: suchi clear

Pro Tip: Keep your tasks organized and stay productive with `suchi`!
"#;

fn help() {
    println!("{}", HELP);
}
