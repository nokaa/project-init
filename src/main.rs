#[macro_use]
extern crate clap;

use clap::{App, Arg};

use std::{env, fs, process};
use std::io::{self, Read, Write};

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

    let config = config_dir();
    let mut license_path = config.clone();
    license_path.push_str(license);

    if !file_exists(&license_path[..]) {
        println!("License \"{}\" does not exist!", license);
        process::exit(1);
    }
    
    // We make sure that the License exists before we create the
    // directory
    if !dir && !cargo { // Create directory manually
        fs::create_dir_all(name).unwrap();
        change_dir(name).unwrap();
        fs::create_dir_all("src").unwrap();
    } else if !dir && cargo { // using cargo
        let cmd = process::Command::new("cargo")
            .arg("new")
            .arg("--bin")
            .arg(name)
            .output()
            .expect("failed to execute cargo");

        if !cmd.status.success() {
            println!("stderr: {}", String::from_utf8_lossy(&cmd.stderr));
            process::exit(1);
        }

        change_dir(name).unwrap();
    } else if cargo { // use current directory and cargo
        let cmd = process::Command::new("cargo")
            .arg("init")
            .arg("--bin")
            .arg(name)
            .output()
            .expect("failed to execute cargo");

        if !cmd.status.success() {
            println!("stderr: {}", String::from_utf8_lossy(&cmd.stderr));
            process::exit(1);
        }
    } else { // Use current directory manually
        fs::create_dir_all("src").unwrap();
    }

    project_files(name, &license_path[..]);
}

fn project_files(name: &str, license_path: &str) {
    copy_file(&license_path[..], "LICENSE").unwrap();
    write_file("README.md", name.as_bytes()).unwrap();
}

fn read_file(filename: &str) -> Result<Vec<u8>, io::Error> {
    let mut f = try!(fs::File::open(filename));
    let mut buf: Vec<u8> = vec![];
    try!(f.read_to_end(&mut buf));

    Ok(buf)
}

fn write_file(filename: &str, data: &[u8]) -> Result<(), io::Error> {
    let mut file = try!(fs::File::create(filename));
    try!(file.write_all(data));
    Ok(())
}

fn copy_file(filename: &str, dest: &str) -> Result<(), io::Error> {
    let f = read_file(filename).unwrap();
    try!(write_file(dest, &f[..]));
    Ok(())
}

fn file_exists(filename: &str) -> bool {
    match fs::File::open(filename) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn config_dir() -> String {
    match env::var("XDG_CONFIG_HOME") {
        Ok(mut s) => {
            if s.is_empty() {
                s = get_home_dir();
                s.push_str("/.config/");
            } else if !s.ends_with('/') {
                s.push('/');
            }
            s.push_str("license-add/");
            s
        }
        Err(_) => {
            let mut home = get_home_dir();
            home.push_str("/.config/license-add/");
            home
        }
    }
}

fn get_home_dir() -> String {
    env::home_dir().unwrap().to_str().unwrap().to_string()
}

fn change_dir(dir: &str) -> io::Result<()> {
    let mut current_dir = try!(env::current_dir());
    current_dir.push(dir);
    try!(env::set_current_dir(current_dir));
    Ok(())
}
