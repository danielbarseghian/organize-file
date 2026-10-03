use std::path::Path;
use std::fs;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get the arguments
    let args: Vec<String> = env::args().collect();

    // check the length
    if args.len() != 2 {
        println!("Usage: cargo run -- filename");
        std::process::exit(0)
    }

    dbg!(&args); // i need to borrow it here or else i can't use it afterweards!'

    println!("{}", args[1]);

    // splits for checking the extension
    let splits: Vec<&str> = args[1].split(".").collect();

    let p = format!("./organize/{}", splits[1]);

    let paths = Path::new(&p);

    // if it doesnt existe
    if !paths.exists() {
        fs::create_dir_all(p)?;
    }

    Ok(())
}
