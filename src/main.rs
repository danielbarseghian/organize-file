use std::env;

fn main() {
    // get the arguments
    let args: Vec<String> = env::args().collect();

    // check the length
    if args.len() != 2 {
        println!("Usage: cargo run -- filename");
        std::process::exit(0)
    }

    dbg!(args);
}
