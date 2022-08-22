// Copyleft (C) 2022 Robin Syihab.
// All Rights Reserved.
//
// This code is part of Leak Checker.
//

#![feature(generators, generator_trait, type_alias_impl_trait)]

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
                .takes_value(true),
        )
        .get_matches();

    let input_file = mc.value_of("input").unwrap_or("../data/metranet_log.csv");

    println!("File to process: {}", input_file);

    converter::parse_file(input_file, parsers::MetranetLog::new()).unwrap();
}
