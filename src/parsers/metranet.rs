// Copyleft (C) 2022 Robin Syihab.
// All Rights Reserved.
//
// This code is part of Leak Checker.
//

use std::io::Result;



use crate::parsers::{GenT, GeneratorIteratorAdapter, Parser};

pub struct MetranetLog {}

impl MetranetLog {
    pub fn new() -> MetranetLog {
        Self {}
    }
}

impl Parser for MetranetLog {
    fn name(&self) -> &'static str {
        "Indihome browsing history log (Metranet)"
    }

    fn file_out_name(&self) -> &'static str {
        "metranet"
    }

    fn parse(line: &str) -> Result<GeneratorIteratorAdapter<GenT>> {
        let generator = parse_internal(line);
        let tokens = GeneratorIteratorAdapter::new(generator);
        Ok(tokens)
    }
}

lazy_static! {
    static ref RE_NAME: regex::Regex = regex::Regex::new(r#"""name"":""(.*?)"""#).unwrap();
    // static ref RE_EMAIL: regex::Regex = regex::Regex::new(r#"""email"":""(.*?)"""#).unwrap();
    static ref RE_NIK: regex::Regex = regex::Regex::new(r#"""nik"":(.*?)(\}|,)"#).unwrap();
    // static ref RE_IP: regex::Regex = regex::Regex::new(r",(\d*\.\d*\.\d*\.\d*),").unwrap();
}

fn parse_internal(line: &str) -> GenT {
    let line = line.to_string();
    move || {
        if let Some(caps) = RE_NAME.captures(&line) {
            let name = caps.get(1).unwrap().as_str();
            yield name.to_string().to_lowercase();
        }
        if let Some(caps) = RE_NIK.captures(&line) {
            let nik = caps.get(1).unwrap().as_str();
            yield nik.to_string();
        }
    }
}
