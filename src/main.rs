#![feature(inclusive_range_syntax)]

extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate toml;

use failure::Error;
use regex::Regex;
use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::{Read, Write};
use std::path::Path;
use toml::Value;


static HTML_HEAD: &'static str =
r#"<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width,initial-scale=1">
    <title>Gaffs</title>
  </head>
  <body>
    <table>
      <colgroup>
        <col span="1" style="background-color: #ccc;">
      </colgroup>
      <tr>
        <th></th>
"#;
static HTML_TAIL: &'static str =
r#"    </table>
  </body>
</html>
"#;


#[derive(Debug, Fail)]
pub enum ArgsError {
    #[fail(display = "Missing argument: {}", arg_name)]
    MissingArg {
        arg_name: &'static str,
    },
}

#[derive(Debug, Fail)]
pub enum ParseError {
    #[fail(display = "TOML parse error: {}", toml_err)]
    TomlError {
        toml_err: &'static str,
    },
    #[fail(display = "TOML key error: got {}, expected {}", key, expected)]
    KeyError {
        key:      String,
        expected: &'static str,
    },
    #[fail(display = "Missing TOML key: {}", key)]
    MissingKey {
        key: &'static str,
    },
}


fn write_html<S, P>(html_filename: P, html_contents: S) -> Result<(), Error>
    where S: Into<String>,
          P: AsRef<Path>
{
    let mut html_out = File::create(html_filename)?;
    html_out.write_all(html_contents.into().as_bytes())?;

    Ok(())
}

fn generate_html(toml_val: Value) -> Result<String, Error> {
    lazy_static! {
        static ref LV_RE: Regex =
            Regex::new(r"^level_([1-9]\d?)(_v2)?$").unwrap();
    }

    let mut html_str = HTML_HEAD.to_string();

    let main_table = toml_val.as_table().ok_or(ParseError::TomlError {
        toml_err: "top-level value is not a Table",
    })?;

    let mut lvs = BTreeMap::new();
    for (lv, lv_data) in main_table {
        let lv_caps = LV_RE.captures(lv).ok_or(ParseError::KeyError {
            key:      lv.clone(),
            expected: "level_#[_v2]",
        })?;

        let level: u8 = lv_caps.get(1).unwrap().as_str().parse().unwrap();
        let v2 = lv_caps.get(2).is_some();
        lvs.insert((v2, level), lv_data);
    }
    for ((v2, level), lv_data) in lvs {
        let nonlured_data =
            lv_data.get("nonlured").ok_or(ParseError::MissingKey {
                key: "nonlured",
            })?;
        let lured_data =
            lv_data.get("lured").ok_or(ParseError::MissingKey {
                key: "lured",
            })?;

        let nonlured_nonorg_data =
            nonlured_data.get("nonorg").ok_or(ParseError::MissingKey {
                key: "nonorg",
            })?;
        let nonlured_org_data =
            nonlured_data.get("org").ok_or(ParseError::MissingKey {
                key: "org",
            })?;
        let lured_nonorg_data =
            lured_data.get("nonorg").ok_or(ParseError::MissingKey {
                key: "nonorg",
            })?;
        let lured_org_data =
            lured_data.get("org").ok_or(ParseError::MissingKey {
                key: "org",
            })?;

        let level_string =
            format!("{}{}", level, if v2 { " v2.0" } else { "" });

        html_str.push_str(r#"        <th>Level "#);
        html_str.push_str(&level_string);
        html_str.push_str(r#" (not lured)</th>
        <th>Level "#);
        html_str.push_str(&level_string);
        html_str.push_str(r#" (lured)</th>
"#);
    }
    html_str.push_str(r#"      </tr>
"#);

    for toon_count in 1..=4u8 {
        for &org in &[false, true] {
            html_str.push_str(r#"      <tr>
        <td class="col-header">"#);
            html_str.push_str(
                &format!(r#"{} toon{} ({}"#,
                    toon_count,
                    if toon_count == 1 { "" } else { "s" },
                    if org { "with" } else { "no" }));
            html_str.push_str(r#" organic)</td>
"#);

            html_str.push_str(r#"      </tr>
"#);
        }
    }



    html_str.push_str(HTML_TAIL);

    Ok(html_str)
}

fn read_toml<P: AsRef<Path>>(toml_filename: P) -> Result<String, Error> {
    let toml_file = File::open(toml_filename)?;
    let mut buf_reader = BufReader::new(toml_file);
    let mut toml_contents = String::with_capacity(16_384);
    buf_reader.read_to_string(&mut toml_contents)?;

    Ok(toml_contents)
}

fn _main() -> Result<(), Error> {
    let mut toml_filename = None;
    let mut html_filename = None;
    for (i, arg) in env::args().enumerate() {
        match i {
            1 => toml_filename = Some(arg),
            2 => html_filename = Some(arg),
            _ => (),
        }
    }
    let toml_filename = toml_filename.ok_or(ArgsError::MissingArg {
        arg_name: "input TOML filename",
    })?;
    let html_filename = html_filename.ok_or(ArgsError::MissingArg {
        arg_name: "output HTML filename",
    })?;

    let toml_contents = read_toml(toml_filename)?;
    let toml_val: Value = toml_contents.parse()?;

    let html_contents = generate_html(toml_val)?;
    write_html(html_filename, html_contents)?;

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
