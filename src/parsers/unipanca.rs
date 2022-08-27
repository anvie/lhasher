// Copyleft (C) 2022 Robin Syihab.
// All Rights Reserved.
//
// This code is part of Leak Checker.
//

use std::{collections::HashMap, io::Result};

use crate::parsers::{ParseResult, ParseStatus, Parser};

use nom_sql::{Literal, SqlQuery::Insert};

pub struct UniPancaDB {
    in_buffer: bool,
}

impl UniPancaDB {
    pub fn new() -> UniPancaDB {
        Self { in_buffer: false }
    }
}

impl Parser for UniPancaDB {
    fn name(&self) -> &'static str {
        "Universitas Pancasila DB"
    }

    fn file_out_name(&self) -> &'static str {
        "unipanca"
    }

    fn parse(&mut self, line: &str) -> ParseResult {
        let ps = self.parse_queryset(line.to_string());
        Ok(ps)
    }
}

lazy_static! {
    static ref TABLE_NAMES: HashMap<&'static str, u32> = vec![
        ("dosenskripsi", 2),
        ("dosenttp", 2),
        ("msdos", 2),
        ("msmhs", 4),
        ("tbdos", 7),
        ("xangket_dosen", 3),
    ]
    .iter()
    .cloned()
    .collect();
}

impl UniPancaDB {
    fn parse_queryset(&mut self, query: String) -> ParseStatus {
        let _query = query.trim();
        if _query.is_empty() {
            return ParseStatus::Ignored;
        }
        if _query.starts_with("--") || _query.starts_with("/*") {
            return ParseStatus::Ignored;
        }
        if _query.ends_with(";") {
            if self.in_buffer {
                self.in_buffer = false;
                return ParseStatus::BufferEnd(_query.to_string());
            }
        }
        match nom_sql::parser::parse_query(&query) {
            Ok(parsed) => {
                // println!("{} OK", &query);
                // println!("parsed: {}", parsed);
                self.in_buffer = false;
                match parsed {
                    Insert(st) => {
                        if let Some(col) = TABLE_NAMES.get(&st.table.name.as_str()) {
                            let mut names = vec![];
                            for item in st.data {
                                if let Literal::String(value) = &item[*col as usize] {
                                    // println!("item: {}", value);
                                    let _value = normalize_name(value);
                                    if _value.trim().len() > 0 {
                                        names.push(_value);
                                    }
                                }
                            }
                            return ParseStatus::Ready(names.into_iter());
                        }
                        return ParseStatus::Ignored;
                    }
                    _ => return ParseStatus::Ignored,
                }
            }
            Err(_) => {
                if _query.ends_with(";") {
                    return ParseStatus::Ignored;
                }
                self.in_buffer = true;
                // ParseStatus::Eof //vec!["failed".to_string()].into_iter()
                ParseStatus::Buffer(query.to_string())
            }
        }
    }
}

// remove titles
#[inline(always)]
fn normalize_name(name: &String) -> String {
    name.chars().take_while(|c| *c != ',').collect()
}
