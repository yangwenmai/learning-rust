use std::fs::File;
use std::fs::OpenOptions;
use std::io::copy;
use std::io::prelude::*;
use std::path::Path;
use clap::{Arg, App, SubCommand};
use colored::*;

fn main() {
    let gitignore_file = ".gitignore";

    let matches = App::new("git-ignore")
        .version("0.0.1")
        .author("yangwenmai <yangwen.yw@gmail.com>")
        .about("Manages gitignore files")
        .subcommand(SubCommand::with_name("init")
            .about("Initializes gitignore file with language")
            .arg(Arg::with_name("language")
                .required(true)
                .index(1))
            .arg(Arg::with_name("overwrite")
                .long("overwrite")
                .help("overwrite .gitignore file"))
            .help("programming language specified gitignore file"))
        .subcommand(SubCommand::with_name("+")
            .about("Add rules to gitignore file")
            .arg(Arg::with_name("node")
                .required(true)
                .index(1))
            .help("file or folder need to be ignored"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("init") {
        let is_overwrite = matches.is_present("overwrite");
        if Path::new(gitignore_file).exists() && !is_overwrite {
            println!("{}: .gitignore already exists", "Warning".bold().red());
            return
        }

        let lang = matches.value_of("language").unwrap();
        let target = format!("https://www.gitignore.io/api/{}", lang);
        let mut response = reqwest::get(target.as_str()).unwrap();

        if response.status().is_success() {
            if is_overwrite {
                let mut file = OpenOptions::new()
                    .write(true)
                    .open(gitignore_file)
                    .unwrap();

                let _ = copy(&mut response, &mut file);
                println!("====> overwrite .gitignore file for {}", lang.bold())
            } else {
                let mut file = File::create(gitignore_file).unwrap();
                let _ = copy(&mut response, &mut file);
                println!("===> .gitignore file for {} initialized", lang.bold())
            }
        }else {
            println!("{}: {}.gitignore not found on gitignore.io", "Warning".bold().red(), lang.bold());
        }
    }
    if let Some(matches) = matches.subcommand_matches("+") {
        if !Path::new(gitignore_file).exists() {
            println!("{}: .gitignore not exists, use `git-ignore init` to initialize ignore file.", "Warning".bold().red());
            return
        }

        let file_or_folder = matches.value_of("node").unwrap();

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(gitignore_file)
            .unwrap();

        if let Err(e) = file.write_fmt(format_args!("{}\n", file_or_folder)) {
            println!("{}: write {} to .gitignore failed, err: {}.", "Warning".bold().red(), file_or_folder, e);
        } else {
            println!("===> {} added to .gitignore file", file_or_folder.bold())
        }
    }
}

