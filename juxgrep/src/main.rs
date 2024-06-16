use std::env;
use std::process;

use juxgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); // Collects args in the same format as bash, $0 is the binary path and $1 is the first arg

    let config: Config = Config::new(&args)
    .unwrap_or_else(|err| {
        println!("Error: Failed to parse arguments - {}", err);
        process::exit(1);
    });
    println!("Searching for {} in {}", config.query, config.filename);

    if let Err(e) = juxgrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
