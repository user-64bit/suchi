use chrono;
use std::collections::HashSet;
use std::fs::{remove_file, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::{env, process};
#[macro_use]
extern crate prettytable;
use prettytable::Table;

fn main() {
    let home: String = env::var("HOME").unwrap_or_else(|_| "~".to_string());
    let suchi_path = format!("{home}/.suchi");

    // initialization of .suchi
    init(&suchi_path);

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command = &args[1];
        match &command[..] {
            "show" => show(&suchi_path),
            "add" => add(&suchi_path, &args[2..]),
            "done" => toggle_done_undone(&suchi_path, &args[2..], true),
            "undone" => toggle_done_undone(&suchi_path, &args[2..], false),
            "delete" => delete(&suchi_path, &args[2..]),
            "edit" => edit(&suchi_path, &args[2..]),
            "filter" => filter(&suchi_path, &args[2..]),
            "search" => search(&suchi_path, &args[2..]),
            "clear" => clear(&suchi_path),
            "--help" | "help" | "-h" | _ => help(),
        }
    } else {
        show(&suchi_path);
    }
}

fn init(suchi_path: &str) {
    // if .suchi doesn't exits then create.
    let _file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .create(true)
        .open(&suchi_path);
}

fn add(suchi_path: &str, args: &[String]) {
    if args.is_empty() {
        eprintln!(
            r#"
            ++===============================================++
            ++`suchi add` command takes at least 1 argument. ++
            ++ Use --help to know more about suchi. :)       ++
            ++===============================================++
            "#
        );
        process::exit(1);
    }

    // handles if there is .suchi file already created.
    let suchi_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(suchi_path)
        .expect("Unable to open file.");

    let now = chrono::Local::now();
    let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let mut buffer: BufWriter<std::fs::File> = BufWriter::new(suchi_file);
    for arg in args.iter().filter(|arg| !arg.trim().is_empty()) {
        let line = format!("✗| {}| {}\n", timestamp, arg.trim());
        buffer
            .write_all(line.as_bytes())
            .expect("Couldn't able to write")
    }
    buffer.flush().expect("Failed to flush buffer..");
    show(&suchi_path);
}

fn show(suchi_path: &str) {
    let mut table = Table::new();
    let suchi = OpenOptions::new()
        .read(true)
        .open(suchi_path)
        .expect("Unable open file.");

    let reader = BufReader::new(suchi);
    let mut flag = true;
    let mut index = 1;

    for line in reader.lines() {
        let line = line.expect("unable to read line.");
        if line.trim().is_empty() {
            continue;
        }
        if line.len() > 0 && flag {
            flag = false;
            table.add_row(row!["#", "created_on", "task", "progress"]);
        }
        if let Some((progress, remaining)) = line.split_once("| ") {
            if let Some((timestamp, task)) = remaining.split_once("| ") {
                table.add_row(row![index, timestamp, task, progress]);
                index += 1;
            }
        };
    }
    if !flag {
        table.printstd();
    } else {
        eprintln!(
            r#"
            ++===============================================++
            ++ You haven't added anything to suchi yet.      ++
            ++ Use --help to know more about suchi. :)       ++
            ++===============================================++
            "#
        );
    }
}

fn clear(suchi_path: &str) {
    remove_file(suchi_path).expect("Unable to locate file.");
    println!(
        r#"
        ++===============================================++
        ++ All the data in your suchi has been cleared.  ++
        ++===============================================++
        "#
    );
}

fn delete(suchi_path: &str, args: &[String]) {
    // Todo: computation can be reduce

    if args.is_empty() {
        eprintln!(
            r#"
            ++==================================================++
            ++`suchi delete` command takes at least 1 argument. ++
            ++ Use --help to know more about suchi. :)          ++
            ++==================================================++
            "#
        );
        process::exit(1);
    }

    // 1. cleaning args
    let args: Vec<usize> = args_cleaning(args);

    // 2. Reading file
    let suchi = OpenOptions::new()
        .read(true)
        .open(suchi_path)
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
        .expect("Unable to open file.");
    let mut buffer = BufWriter::new(&suchi);
    for line in &lines {
        if line.trim().is_empty() {
            continue;
        }
        let line = format!("{}\n", line);
        buffer
            .write_all(line.as_bytes())
            .expect("Couldn't able to write")
    }
    buffer.flush().expect("Failed to flush buffer..");
    show(&suchi_path);
}

fn toggle_done_undone(suchi_path: &str, args: &[String], flag: bool) {
    if args.is_empty() {
        eprintln!(
            r#"
            ++==================================================++
            ++`suchi [un]done` command takes at least 1 argument. ++
            ++ Use --help to know more about suchi. :)          ++
            ++==================================================++
            "#
        );
        process::exit(1);
    }
    // 1. cleaning args
    let args: Vec<usize> = args_cleaning(args);

    // 2. Reading file
    let suchi = OpenOptions::new()
        .read(true)
        .open(&suchi_path)
        .expect("Unable to open file.");
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
        let line_to_be_updated: &String = &lines[arg - 1]; // 0 indexing
        if let Some((_progress, others)) = line_to_be_updated.split_once("| ") {
            let mut new_line = "";
            if flag {
                new_line = "✓| ";
            } else if !flag {
                new_line = "✗| ";
            }
            // println!("{new_line}{others}-{arg}");
            lines[arg - 1] = new_line.to_string() + others;
        }
    }
    // 5. Now overriding in file with updated Vector of strings(same as add() but with truncate=true)
    let suchi = OpenOptions::new()
        .write(true)
        .truncate(true) // it will truncate the file to 0 length
        .open(suchi_path)
        .expect("Unable to open file.");
    let mut buffer = BufWriter::new(&suchi);
    for line in &lines {
        if line.trim().is_empty() {
            continue;
        }
        let line = format!("{}\n", line);
        buffer
            .write_all(line.as_bytes())
            .expect("Couldn't able to write")
    }
    buffer.flush().expect("Failed to flush buffer..");
    show(&suchi_path);
}

fn edit(suchi_path: &str, args: &[String]) {
    if args.is_empty() {
        eprintln!(
            r#"
            ++==================================================++
            ++`suchi edit` command takes at least 1 argument.   ++
            ++ Use --help to know more about suchi. :)          ++
            ++==================================================++
            "#
        );
        process::exit(1);
    }

    if args.len() > 2 {
        eprintln!(
            r#"
            ++==================================================++
            ++ Unrecognize commands: {:?}                       ++
            ++ Use --help to know more about suchi. :)          ++
            ++==================================================++
            "#,
            &args[2..]
        );
        process::exit(1);
    }

    let number_of_todo_edit: usize = args[0].parse::<usize>().expect("Error");
    let updated_text = &args[1];

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

    // 4. removing line
    let old_text = &lines[number_of_todo_edit - 1];
    if let Some((progress, others)) = old_text.split_once("| ") {
        if let Some((timestap, _task)) = others.split_once("| ") {
            let new_line = progress.to_string() + "| " + timestap + "| " + updated_text;
            lines[number_of_todo_edit - 1] = new_line;
        }
    }

    // 5. Now overriding in file with updated Vector of strings(same as add() but with truncate=true)
    let suchi = OpenOptions::new()
        .write(true)
        .truncate(true) // it will truncate the file to 0 length
        .open(suchi_path)
        .expect("Unable to open file.");
    let mut buffer = BufWriter::new(&suchi);
    for line in &lines {
        if line.trim().is_empty() {
            continue;
        }
        let line = format!("{}\n", line);
        buffer
            .write_all(line.as_bytes())
            .expect("Couldn't able to write")
    }
    buffer.flush().expect("Failed to flush buffer..");
    show(&suchi_path);
}

fn filter(suchi_path: &str, args: &[String]) {
    if args.is_empty() {
        eprintln!(
            r#"
            ++==================================================++
            ++`suchi filter` command takes at least 1 argument. ++
            ++ Use --help to know more about suchi. :)          ++
            ++==================================================++
            "#
        );
        process::exit(1);
    }
    // Todo: Right Now we only support "done" and "undone" filter (modify below condition if you want to add more)
    if args.len() > 1 {
        eprintln!(
            r#"
            ++==================================================++
            ++ Unrecognize commands: {:?}                       ++
            ++ Use --help to know more about suchi. :)          ++
            ++==================================================++
            "#,
            &args[2..]
        );
        process::exit(1);
    }
    let filter_type = &args[0];
    println!("{}", filter_type);
    if filter_type.trim() != "done" && filter_type.trim() != "undone" {
        eprintln!(
            r#"
            ++==================================================++
            ++ Unrecognize commands: {:?}                       ++
            ++ Use --help to know more about suchi. :)          ++
            ++==================================================++
            "#,
            &args
        );
        process::exit(1);
    };

    let progress_mark: &str = if filter_type == "done" { "✓" } else { "✗" };
    let reader = BufReader::new(
        OpenOptions::new()
            .read(true)
            .open(suchi_path)
            .expect("Couldn't able to perform action."),
    );
    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .expect("Unable to read file.");

    let mut filter_lines: Vec<String> = vec![];
    for line in &lines {
        if let Some((progress, _)) = line.split_once("| ") {
            if progress == progress_mark {
                filter_lines.push(line.to_string());
            }
        }
    }
    print_table(&filter_lines);
}

fn search(suchi_path: &str, args: &[String]) {
    if args.is_empty() {
        eprintln!(
            r#"
            ++==================================================++
            ++`suchi filter` command takes at least 1 argument. ++
            ++ Use --help to know more about suchi. :)          ++
            ++==================================================++
            "#
        );
        process::exit(1);
    }
    // Todo: Right Now we only support single keyword search
    if args.len() > 1 {
        eprintln!(
            r#"
            ++==================================================++
            ++ Unrecognize commands: {:?}                       ++
            ++ Use --help to know more about suchi. :)          ++
            ++==================================================++
            "#,
            &args[2..]
        );
        process::exit(1);
    }

    let keyword = &args[0].to_lowercase();
    let reader = BufReader::new(
        OpenOptions::new()
            .read(true)
            .open(suchi_path)
            .expect("Couldn't able to perform action."),
    );
    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .expect("Unable to read file.");

    let mut filter_lines: Vec<String> = vec![];
    for line in &lines {
        let result = line.to_lowercase().find(keyword);
        if let Some(_) = result {
            filter_lines.push(line.to_string())
        }
    }   
    print_table(&filter_lines);
}

const HELP: &str = r#"
 ___ _   _  ___| |__ (_)
/ __| | | |/ __| '_ \| |
\__ \ |_| | (__| | | | |
|___/\__,_|\___|_| |_|_|

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

fn print_table(lines: &Vec<String>) {
    let mut table = Table::new();
    let mut flag = true;
    let mut index = 1;
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        if line.len() > 0 && flag {
            flag = false;
            table.add_row(row!["#", "created_on", "task", "progress"]);
        }
        if let Some((progress, remaining)) = line.split_once("| ") {
            if let Some((timestamp, task)) = remaining.split_once("| ") {
                table.add_row(row![index, timestamp, task, progress]);
                index += 1;
            }
        };
    }
    if !flag {
        table.printstd();
    } else {
        eprintln!(
            r#"
            ++===============================================++
            ++ unable to find anything related               ++
            ++ Use --help to know more about suchi. :)       ++
            ++===============================================++
            "#
        );
    }
}

fn args_cleaning(vector: &[String]) -> Vec<usize> {
    // converting Vector of Strings to Vector of integer(usize here)
    // Todo: use filter_map()
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
    // Todo: use dedup()
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
