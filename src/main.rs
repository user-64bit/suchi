use clap::Parser;

#[derive(Parser)]
struct Args {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();
    let content = std::fs::read_to_string(&args.path);
    match content {
        Ok(data) => {
            println!("file content {}", data);
        }
        Err(err) => {
            println!("Error, {}", err);
        }
    }
}
