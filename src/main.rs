use std::env;

fn main() {
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
      _ => println!("get some help"),
    }
  } else {
    println!("--help to know more...")
  }
}
