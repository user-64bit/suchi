use chrono::{Datelike, Timelike};
use std::collections::HashSet;
use std::fs::{remove_file, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::{env, process};
#[macro_use]
extern crate prettytable;
use prettytable::Table;

fn main() {
    let home: String = env::var("HOME").expect("HOME directory not found");
    let suchi_path = format!("{home}/.suchi");

    // initialization of .suchi
    init(&suchi_path);

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command = &args[1];
        match &command[..] {
            "show" => show(&suchi_path, &args[2..]),
            "add" => add(&suchi_path, &args[2..]),
            "done" => toggle_done_undone(&suchi_path, &args[2..]),
            "undone" => toggle_done_undone(&suchi_path, &args[2..]),
            "delete" => delete(&suchi_path, &args[2..]),
            "edit" => edit(&suchi_path, &args[2..]),
            "clear" => clear(&suchi_path, &args[2..]),
            "--help" | "help" | "-h" | _ => help(),
        }
    } else {
        help();
    }
}

fn init(suchi_path: &String) {
    // if .suchi doesn't exits then create.
    let _file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .create(true)
        .open(&suchi_path);
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
            "[✗] [{}-{}-{} {}:{}:{}] {}\n",
            year,
            month,
            day,
            hours,
            minutes,
            seconds,
            arg.trim()
        );
        buffer
            .write_all(line.as_bytes())
            .expect("Couldn't able to write")
    }
}

fn show(suchi_path: &String, args: &[String]) {
    if !args.is_empty() {
        println!("suchi show command doesn't recognize: {:?}", &args);
        process::exit(1);
    }
    let mut number = 1;
    let mut table = Table::new();
    let suchi = OpenOptions::new()
        .read(true)
        .open(suchi_path)
        .expect("Couldn't able to perform action.");

    // Headers
    table.add_row(row!["number", "created_on", "task", "progress"]);

    let reader = BufReader::new(&suchi);
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if let Some((progress, others)) = line.split_once("] "){
            if let Some((timestamp, task)) =  others.split_once("] "){
                table.add_row(row![number, &timestamp[1..], task, &progress[1..]]);
                number += 1;
            }
        };
    }
    table.printstd();
}

fn clear(suchi_path: &String, args: &[String]) {
    if !args.is_empty() {
        println!("suchi show command doesn't recognize: {:?}", &args);
        process::exit(1);
    }
    remove_file(suchi_path).expect("Couldn't able to perform action");
    println!("suchi cleared...")
}

fn delete(suchi_path: &String, args: &[String]) {
    // Todo: computation can be reduce

    if args.is_empty() {
        println!("suchi delete command takes atleast 1 argument.");
        process::exit(1);
    }
    // 1. cleaning vector
    let args: Vec<usize> = vector_cleaning(args);

    // 2. Reading file
    let suchi = OpenOptions::new()
        .read(true)
        .open(&suchi_path)
        .expect("Couldn't able to perform action.");
    let reader = BufReader::new(&suchi);
    // 3. convert file to Vector of Strings to removing string become easy
    let mut lines: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .expect("Unable to read file.");

    for arg in args {
        if arg > lines.len() {
            println!("Line number {:?} doesn't exists in file.", &arg);
            process::exit(1);
        }
        // 4. removing line
        lines.remove(arg - 1); // 0 indexing
    }

    // 5. Now overriding in file with updated Vector of strings(same as add() but with truncate=true)
    let suchi = OpenOptions::new()
        .write(true)
        .truncate(true) // it will truncate the file to 0 length
        .open(suchi_path)
        .expect("Couldn't able to perform action.");
    let mut writer = BufWriter::new(&suchi);
    for line in &lines {
        let line = format!("{}\n", line);
        writer.write_all(line.as_bytes()).expect("Deleting failed");
    }
}

fn toggle_done_undone(suchi_path: &String, args: &[String]) {
    if args.is_empty() {
        println!("suchi done command takes atleast 1 argument.");
        process::exit(1);
    }
    // 1. cleaning vector
    let args: Vec<usize> = vector_cleaning(args);

    // 2. Reading file
    let suchi = OpenOptions::new()
        .read(true)
        .open(&suchi_path)
        .expect("Couldn't able to perform action.");
    let reader = BufReader::new(&suchi);
    // 3. convert file to Vector of Strings to updating string become easy
    let mut lines: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .expect("Unable to read file.");

    for arg in args {
        if arg > lines.len() {
            println!("Line number {:?} doesn't exists in file.", &arg);
            process::exit(1);
        }
        // 4. updating line
        let line_to_be_updated: &String = &lines[arg-1]; // 0 indexing
        if let Some((progress, others)) = line_to_be_updated.split_once("] "){
            let mut new_line = "";
            if progress == "[✗" {
                new_line = "[✓] ";
            }
            else if progress == "[✓" {
                new_line = "[✗] ";
            }
            lines[arg-1] = new_line.to_string() + &others;
        }
    }
    // 5. Now overriding in file with updated Vector of strings(same as add() but with truncate=true)
    let suchi = OpenOptions::new()
        .write(true)
        .truncate(true) // it will truncate the file to 0 length
        .open(suchi_path)
        .expect("Couldn't able to perform action.");
    let mut writer = BufWriter::new(&suchi);
    for line in &lines {
        let line = format!("{}\n", line);
        writer.write_all(line.as_bytes()).expect("Deleting failed");
    }
}

fn edit(suchi_path: &String, args: &[String]) {
    // Todo: edit task
    println!("{:?}, {:?}", &suchi_path, &args);
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

- undone [INDEXs]
    Mark a task/tasks as complete by its index.
    Example: suchi undone 1 2 (marks the first and second tasks as completed)

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

// helper functions

fn vector_cleaning(vector: &[String]) -> Vec<usize> {
    // Todo: delete this (this function is worst but it works)

    // converting Vector of Strings to Vector of integer(usize here)
    let mut vector_of_usize = Vec::new();
    for v in vector {
        vector_of_usize.push(v.parse::<usize>().expect("Error"));
    }
    /*
        -> sorting and reversing Vector. but why?
        because suppose someone put suchi delete 2 3 4
        delete() function will delete 2nd line and because of that 4th line has become 3rd line
        and 3rd line has become 2nd line
    */
    vector_of_usize.sort();
    vector_of_usize.reverse();

    // removing duplicates
    let mut final_vector: Vec<usize> = Vec::new();
    let mut map: HashSet<usize> = HashSet::new();

    for v in vector_of_usize {
        if !map.contains(&v) {
            final_vector.push(v);
        }
        map.insert(v);
    }
    final_vector
}
