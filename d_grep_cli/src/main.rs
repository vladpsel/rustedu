use d_grep_cli::Config;
use std::{env, process};

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let app = Config::build(&arguments).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(0);
    });

    let finded = app.find();
    println!("{:?}", finded);
}
