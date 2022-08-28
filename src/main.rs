// Copyleft (C) 2022 Robin Syihab.
// All Rights Reserved.
//
// This code is part of Leak Checker.
//

// #![feature(generators, generator_trait, type_alias_impl_trait)]
#![feature(associated_type_defaults)]

extern crate clap;
#[macro_use]
extern crate lazy_static;

use clap::{App, Arg};

mod converter;
mod parsers;

fn main() {
    let mc = App::new("lhasher")
        .version("0.1.0")
        .author("Robin Syihab <robinsyihab@gmail.com>")
        .about("Data hasher")
        .arg(
            Arg::with_name("input")
                .short('i')
                .long("input")
                .value_name("INPUT_FILE")
                .help("Set input file")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("format")
                .short('f')
                .long("format")
                .value_name("FORMAT")
                .help("Input format")
                .required(true)
                .takes_value(true),
        )
        .try_get_matches();

    if mc.is_err() {
        eprintln!("{}", mc.unwrap_err());
        return;
    }

    match mc {
        Err(e) => eprintln!("{}", e),
        Ok(mc) => {
            let input_file = mc
                .value_of("input")
                .unwrap_or_else(|| panic!("No input file"));
            let format = mc.value_of("format").unwrap_or_else(|| panic!("No format"));

            println!("File to process: {}, format: {}", input_file, format);

            match format {
                "metranetlog" => {
                    converter::parse_file(input_file, parsers::MetranetLog::new()).unwrap()
                }
                "unipanca" => {
                    converter::parse_file(input_file, parsers::UniPancaDB::new()).unwrap()
                }
                "bhinneka" => {
                    converter::parse_file(input_file, parsers::BhinnekaDB::new()).unwrap()
                }
                x => eprintln!("Unknown format {}", x),
            }
        }
    }
}
