#[macro_use]
extern crate clap;

use clap::{App, Arg};

use std::{env, fs, process};

fn main() {
    let matches = App::new("project-init")
        .version(crate_version!())
        .author("nokaa <nokaa@cock.li>")
        .about("A project initializer")
        .arg(Arg::with_name("directory")
             .short("d")
             .long("directory")
             .value_name("DIR")
             .help("Initialize project in the current directory")
             .takes_value(false))
        .arg(Arg::with_name("cargo")
             .short("c")
             .long("cargo")
             .value_name("CARGO")
             .help("Create new project with cargo")
             .takes_value(false))
        .arg(Arg::with_name("PROJECT NAME")
             .help("Sets the name of the new project")
             .required(true)
             .index(1))
        .arg(Arg::with_name("LICENSE")
             .help("Sets the license of the new project")
             .required(true)
             .index(2))
        .get_matches();

    let name = matches.value_of("PROJECT NAME").unwrap();
    let license = matches.value_of("LICENSE").unwrap();
    let dir = matches.is_present("directory");
    let cargo = matches.is_present("cargo");

    println!("name: {}\tlicense: {}", name, license);
    println!("dir: {}\tcargo: {}", dir, cargo);

    let config = config_dir();

    //home.push(license);
    let license_path = "";//home.to_str().unwrap();
    if !file_exists(license_path) {
        println!("License \"{}\" does not exist!", license);
        process::exit(1);
    }
    println!("license exists!");
}

fn file_exists(filename: &str) -> bool {
    match fs::File::open(filename) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn config_dir() -> String {
    match env::var("XDG_CONFIG_HOME") {
        Ok(s) => {
            s
        }
        Err(_) => {
            let mut home = env::home_dir().unwrap();
            home.push(".config/");
            home.to_str().unwrap().to_string()
        }
    }
}
