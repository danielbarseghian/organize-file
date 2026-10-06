use std::path::Path;
use std::fs;
use std::env;
use std::io::ErrorKind;
use std::io::Error;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get the arguments
    let args: Vec<String> = env::args().collect();

    // check the length
    if args.len() != 2 {
        println!("Usage: cargo run -- filename");
        std::process::exit(1)
    }

    // check if path is existante
    if !Path::new(&args[1]).exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            "file doesnt exists"
        )
        .into()); // convert the error to box
    }

    // splits for checking the extension
    let splits: Vec<&str> = args[1].split(".").collect();

    let folder = format!("./organize/{}", splits[1]);

    let paths = Path::new(&folder);

    // if it doesnt existe
    if !paths.exists() {
        fs::create_dir_all(&folder)?;
    }

    // make variable for moving
    let new_file = format!("{}/{}", &folder, args[1]);

    dbg!(&new_file);

    // the compiler is too good
    fs::rename(<String as Clone>::clone(&args[1]), new_file)?;

    Ok(())
}
