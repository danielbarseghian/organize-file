use std::env;

fn main() {
    // get the arguments
    let args: Vec<String> = env::args().collect();

    // check the length
    if args.len() != 2 {
        println!("Usage: cargo run -- filename");
        std::process::exit(0)
    }

    dbg!(&args); // i need to borrow it hear or else i can't use it afterweards!'

    println!("{}", args[1]);

    // splits for checking the extension
    let splits: Vec<&str> = args[1].split(".").collect();

    println!("{}", splits[1]);
}
