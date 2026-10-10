use std::path::Path;
use std::fs;
use std::env;
use std::io::ErrorKind;
use std::io::Error;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get the arguments
    let args: Vec<String> = env::args().collect();

    // check the length
    if args.len() < 2 {
        println!("Usage: cargo run -- filename");
        std::process::exit(1)
    }

    for i in 1..args.len() {

        // check if path is existante
        if !Path::new(&args[i]).exists() {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("file {} doesnt exists", args[i])
            )
            .into()); // convert the error to box
        }

        let extension;

        // There is a .
        if args[1].contains(".") {
            // splits for checking the extension
            let splits: Vec<&str> = args[i].split(".").collect();

            extension = splits[1].to_string();

        } else { // there are no .
            extension = args[1].to_string();
        }


        let folder = format!("./organize/{}", extension);

        let paths = Path::new(&folder);

        // if it doesnt existe
        if !paths.exists() {
            fs::create_dir_all(&folder)?;
        }

        // make variable for moving
        let new_file = format!("{}/{}", &folder, args[i]);

        dbg!(&new_file);

        // the compiler is too good
        fs::rename(<String as Clone>::clone(&args[i]), new_file)?;
    }

    Ok(())
}
