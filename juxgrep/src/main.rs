use std::env;
use std::process;

use juxgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); // Collects args in the same format as bash, $0 is the binary path and $1 is the first arg

    let config: Config = Config::new(&args)
    .unwrap_or_else(|err| {
        eprintln!("Error: Failed to parse arguments - {}", err);
        process::exit(1);
    });
    println!("Searching for {} in {}", config.query, config.filename);

    // eprintln prints to stderr
    if let Err(e) = juxgrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
