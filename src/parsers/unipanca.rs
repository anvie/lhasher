// Copyleft (C) 2022 Robin Syihab.
// All Rights Reserved.
//
// This code is part of Leak Checker.
//

use std::collections::HashMap;

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
        "Universitas Pancasila"
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
    static ref TABLE_NAMES: HashMap<&'static str, Vec<u32>> = vec![
        ("dosenskripsi", vec![2]),
        ("dosenttp", vec![2]),
        ("msdos", vec![2]),
        ("msmhs", vec![4]),
        ("tbdos", vec![7]),
        ("xangket_dosen", vec![3]),
        ("xmahasiswa", vec![2, 22]), // 2 = name, 22 = email
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
        if _query.ends_with(';') && self.in_buffer {
            self.in_buffer = false;
            return ParseStatus::BufferEnd(_query.to_string());
        }
        match nom_sql::parser::parse_query(&_query) {
            Ok(parsed) => {
                self.in_buffer = false;
                match parsed {
                    Insert(st) => {
                        if let Some(cols) = TABLE_NAMES.get(&st.table.name.as_str()) {
                            let mut names = vec![];
                            for item in st.data {
                                for col in cols {
                                    if let Literal::String(value) = &item[*col as usize] {
                                        let _value = normalize_name(value).trim().to_lowercase();
                                        if !_value.is_empty() {
                                            names.push(_value);
                                        }
                                    }
                                }
                            }
                            return ParseStatus::Ready(names.into_iter());
                        }
                        ParseStatus::Ignored
                    }
                    _ => ParseStatus::Ignored,
                }
            }
            Err(_) => {
                if _query.ends_with(';') {
                    return ParseStatus::Ignored;
                }
                self.in_buffer = true;
                ParseStatus::Buffer(query.to_string())
            }
        }
    }
}

// remove titles
#[inline(always)]
fn normalize_name(name: &str) -> String {
    name.chars().take_while(|c| *c != ',').collect()
}
