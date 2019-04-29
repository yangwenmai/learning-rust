use std::env;
use std::process;
use std::fs;
use std::error::Error;

use minigrep;
use minigrep::Config;

// step 0, 1, 2: read file
//fn main() {
//    let args: Vec<String> = env::args().collect();
//    println!("{:?}", args);
//
//    let query = &args[1];
//    let filename = &args[2];
//
//    print!("Searching for \"{}\" ", query);
//    println!("in file: {}",filename);
//
//    let contents = fs::read_to_string(filename)
//        .expect("Something went wrong reading the file");
//
//    println!("With text:\n{}", contents)
//}

// step 3: use struct
//fn main() {
//    let args: Vec<String> = env::args().collect();
//    let (query, filename) = parse_config(&args);
//    println!("{} {}", query, filename)
//}
//
//struct Config {
//    query: String,
//    filename: String,
//}
//
//fn parse_config(args: &[String]) -> (Config) {
//    let query = args[1].clone();
//    let filename= args[2].clone();
//    Config{query, filename}
//}

// step 4:
//fn main() {
//    let args: Vec<String> = env::args().collect();
//
//    let config = Config::new(&args).unwrap_or_else(|err| {
//        println!("Problem parsing arguments: {}", err);
//        process::exit(1);
//    });
//
//    run(config);
//    println!("end.");
//}
//
//fn run(config: Config) {
//    let contents = fs::read_to_string(config.filename)
//        .expect("something went wrong reading the file");
//    println!("With text:\n{}", contents)
//}
//
//struct Config {
//    query: String,
//    filename: String,
//}
//
//impl Config {
//    fn new(args: &[String]) -> Result<Config,&'static str> {
//        if args.len()<3 {
//            return Err("not enough arguments");
//        }
//        let query = args[1].clone();
//        let filename = args[2].clone();
//        Ok(Config { query, filename })
//    }
//}

// step 5:
//fn main() {
//    let args: Vec<String> = env::args().collect();
//
//    let config = Config::new(&args).unwrap_or_else(|err| {
//        println!("Problem parsing arguments: {}", err);
//        process::exit(1);
//    });
//
//    print!("Searching for {}", config.query);
//    println!("in file: {}", config.filename);
//
//    // run(config); // warning: unused `std::result::Result` that must be used
//    if let Err(e) = run(config) {
//        println!("Application error: {}", e);
//        process::exit(1);
//    }
//    println!("end.");
//}
//
//fn run(config: Config) -> Result<(), Box<dyn Error>> {
//    let contents = fs::read_to_string(config.filename)?;
//    println!("With text:\n{}", contents);
//    Ok(())
//}
//
//struct Config {
//    query: String,
//    filename: String,
//}
//
//impl Config {
//    fn new(args: &[String]) -> Result<Config,&'static str> {
//        if args.len()<3 {
//            return Err("not enough arguments");
//        }
//        let query = args[1].clone();
//        let filename = args[2].clone();
//        Ok(Config { query, filename })
//    }
//}

// step 6:
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    print!("Searching for {}", config.query);
    println!("in file: {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
    println!("end.");
}