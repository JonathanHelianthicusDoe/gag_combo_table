extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate toml;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use toml::Value;

use failure::Error;


#[derive(Debug, Fail)]
pub enum ArgsError {
    #[fail(display = "Missing argument: {}", arg_name)]
    MissingArg {
        arg_name: &'static str,
    },
}


fn read_toml<P: AsRef<Path>>(toml_filename: P) -> Result<String, Error> {
    let mut toml_file = File::open(toml_filename)?;
    let mut toml_contents = String::new();
    toml_file.read_to_string(&mut toml_contents)?;

    Ok(toml_contents)
}

fn _main() -> Result<(), Error> {
    let toml_filename = env::args().nth(1).ok_or(ArgsError::MissingArg {
        arg_name: "TOML filename"
    })?;
    let toml_contents = read_toml(toml_filename)?;
    let toml_val: Value = toml_contents.parse()?;

    println!("{:?}", toml_val);

    Ok(())
}

fn main() {
    match _main() {
        Err(e) => {
            eprintln!("Something went wrong:");
            eprintln!("  {}.", e);
            std::process::exit(1)
        },
        _ => (),
    }
}
