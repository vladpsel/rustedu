use filestat::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = Config::build(&args).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(0);
    });

    app.analize();
}
