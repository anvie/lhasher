// Copyleft (C) 2022 Robin Syihab.
// All Rights Reserved.
//
// This code is part of Leak Checker.
//
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fs::File,
    io::{prelude::*, BufReader, LineWriter, Result},
    path::Path,
};

use crate::parsers::Parser;

pub fn parse_file<P: Parser, PT: AsRef<Path>>(filename: PT, p: P) -> Result<()> {
    let file = File::open(filename)?;
    let file_output = File::create(
        Path::new("../compiled_data")
            .join(p.file_out_name())
            .with_extension("json"),
    )?;
    let mut file_output_w = LineWriter::new(file_output);

    file_output_w.write_all(format!(r#"{{ "name":"{}", "hashes": ["#, p.name()).as_bytes())?;
    file_output_w.write_all(b"\n")?;

    let reader = BufReader::new(file);

    let mut i = 0;
    let mut existing = BTreeSet::new();

    for line in reader.lines() {
        let line = line?;
        let tokens = P::parse(&line)?;
        for token in tokens {
            let hash_str = hash(&token);
            let crc32_hash = crc32fast::hash(token.as_bytes());
            if existing.contains(&crc32_hash) {
                continue;
            }
            file_output_w.write_fmt(format_args!("\"{}\",\n", hash_str))?;
            existing.insert(crc32_hash);
            i = i + 1;
            if i % 500 == 0 {
                println!("{} -> {} - exists: {}", token, hash_str, existing.len());
                file_output_w.flush()?;
            }
        }
    }
    file_output_w.write_all(b"\"\"\n")?;
    file_output_w.write_all(b"]}\n")?;
    Ok(())
}

fn hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let hash = hasher.finalize();
    format!("{:x}", hash)
}
