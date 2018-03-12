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
static GAG_TYPES: [&'static str; 4] =
    ["sound", "throw", "squirt", "drop"];
static GAG_NAMES: [[&'static str; 8]; 4] = [
    ["pass", "bikehorn",         "whistle",         "bugle",           "aoogah",         "elephant_trunk", "foghorn",     "opera_singer"],
    ["pass", "cupcake",          "fruit_pie_slice", "cream_pie_slice", "fruit_pie",      "cream_pie",      "cake",        "wedding_cake"],
    ["pass", "squirting_flower", "glass_of_water",  "squirtgun",       "seltzer_bottle", "fire_hose",      "storm_cloud", "geyser"      ],
    ["pass", "flowerpot",        "sandbag",         "anvil",           "big_weight",     "safe",           "grand_piano", "toontanic"   ],
];
static GAG_NAMES_DISPLAY: [[&'static str; 8]; 4] = [
    ["Pass", "Bikehorn",         "Whistle",         "Bugle",           "Aoogah",         "Elephant Trunk", "Foghorn",     "Opera Singer"],
    ["Pass", "Cupcake",          "Fruit Pie Slice", "Cream Pie Slice", "Fruit Pie",      "Cream Pie",      "Cake",        "Wedding Cake"],
    ["Pass", "Squirting Flower", "Glass of Water",  "Squirtgun",       "Seltzer Bottle", "Fire Hose",      "Storm Cloud", "Geyser"      ],
    ["Pass", "Flowerpot",        "Sandbag",         "Anvil",           "Big Weight",     "Safe",           "Grand Piano", "Toontanic"   ],
];


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
    #[fail(display = "TOML key error: expected {}, got: {}", expected, key)]
    KeyError {
        key:      String,
        expected: &'static str,
    },
    #[fail(display = "Missing TOML key: {}", key)]
    MissingKey {
        key: &'static str,
    },
    #[fail(display = "TOML array index out of bounds: {}", ix)]
    IxOutOfBounds {
        ix: usize,
    },
    #[fail(display = "TOML entity has wrong type: expected {}, got: {}",
                         expected,
                         got)]
    WrongType {
        got:      &'static str,
        expected: &'static str,
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
    for (&(v2, level), _) in &lvs {
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

    for toon_count in 1..=4 {
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

            for (&(v2, level), lv_data) in &lvs {
                let nonlured_data =
                    lv_data.get("nonlured").ok_or(ParseError::MissingKey {
                        key: "nonlured",
                    })?;
                let lured_data =
                    lv_data.get("lured").ok_or(ParseError::MissingKey {
                        key: "lured",
                    })?;

                let (nonlured, lured) = if org {
                    let nonlured_org_data =
                        nonlured_data.get("org").ok_or(ParseError::MissingKey {
                            key: "org",
                        })?;
                    let lured_org_data =
                        lured_data.get("org").ok_or(ParseError::MissingKey {
                            key: "org",
                        })?;

                    (nonlured_org_data, lured_org_data)
                } else {
                    let nonlured_nonorg_data =
                        nonlured_data.get("nonorg")
                                     .ok_or(ParseError::MissingKey {
                            key: "nonorg",
                        })?;
                    let lured_nonorg_data =
                        lured_data.get("nonorg").ok_or(ParseError::MissingKey {
                            key: "nonorg",
                        })?;

                    (nonlured_nonorg_data, lured_nonorg_data)
                };

                for &gag_data in &[nonlured, lured] {
                    html_str.push_str(r#"        <td><table>
"#);
                    for gag_type_ix in 0..GAG_TYPES.len() {
                        let gag_type = GAG_TYPES[gag_type_ix];
                        let typed_data =
                            gag_data.get(gag_type)
                                    .ok_or(ParseError::MissingKey {
                                key: gag_type,
                            })?.get(toon_count - 1)
                               .ok_or(ParseError::IxOutOfBounds {
                                ix: toon_count - 1,
                            })?.as_array().ok_or(ParseError::WrongType {
                                got:      "?",
                                expected: "array",
                            })?.iter().map(|v|
                                v.as_integer().ok_or(ParseError::WrongType {
                                    got:      v.type_str(),
                                    expected: "integer",
                                }));

                        html_str.push_str(r#"          <tr>"#);
                        for gag_ix in typed_data {
                            let gag_ix = gag_ix? as usize;
                            let gag_name = GAG_NAMES[gag_type_ix][gag_ix];
                            let gag_name_display =
                                GAG_NAMES_DISPLAY[gag_type_ix][gag_ix];

                            html_str.push_str(r#"<td><img src="img/"#);
                            html_str.push_str(gag_name);
                            html_str.push_str(r#".png" alt=""#);
                            html_str.push_str(gag_name_display);
                            html_str.push_str(r#"" title=""#);
                            html_str.push_str(gag_name_display);
                            html_str.push_str(r#""></td>"#);
                        }
                        html_str.push_str(r#"</tr>
"#);
                    }
                    html_str.push_str(r#"        </table></td>
"#);
                }
            }

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
