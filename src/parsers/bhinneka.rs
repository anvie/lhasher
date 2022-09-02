// Copyleft (C) 2022 Robin Syihab.
// All Rights Reserved.
//
// This code is part of Leak Checker.
//

use std::{
    collections::HashMap,
    str,
};

use crate::parsers::{ParseResult, ParseStatus, Parser};

use nom::{
    bytes::complete::{tag, take_while},
    character::{
        is_alphanumeric, is_digit,
        streaming::char,
    },
    sequence::{tuple}, IResult,
};

pub struct BhinnekaDB {
    in_buffer: bool,
    capture_mode: bool,
}

impl BhinnekaDB {
    pub fn new() -> BhinnekaDB {
        Self {
            in_buffer: false,
            capture_mode: false,
        }
    }
}

impl Parser for BhinnekaDB {
    fn name(&self) -> &'static str {
        "Bhinneka e-commerce Database"
    }

    fn file_out_name(&self) -> &'static str {
        "bhinneka"
    }

    fn parse(&mut self, line: &str) -> ParseResult {
        if let Ok(result) = self.parse_internal(line.to_string()) {
            return Ok(result);
        } else {
            return Ok(ParseStatus::Ignored);
        }
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

enum Error {
    InputError,
}

type Result<T> = std::result::Result<T, Error>;

impl<E> From<nom::Err<E>> for Error {
    fn from(_: nom::Err<E>) -> Error {
        Error::InputError
    }
}

impl BhinnekaDB {
    fn parse_internal(&mut self, q: String) -> Result<ParseStatus> {
        let _q = q.trim();
        if _q.is_empty() {
            return Err(Error::InputError);
        }
        if _q.starts_with("--") || _q.starts_with("/*") {
            return Err(Error::InputError);
        }
        if _q.ends_with(";") {
            if self.in_buffer {
                self.in_buffer = false;
                return Ok(ParseStatus::BufferEnd(_q.to_string()));
            }
        }

        if !self.capture_mode {
            let (input, _) = tag::<_, _, ()>("COPY ")(_q)?;
            let (input, _) = take_while::<_, _, ()>(|c| c != ')')(input)?;
            let (input, _) = take_while::<_, _, ()>(|c| c != ';')(input)?;
            let (input, _) = take_while::<_, _, ()>(|c| c != 0x09u8 as char)(input)?; // 0x09 = tab
            self.capture_mode = true;
            return Ok(ParseStatus::Buffer(input.to_string()));
        }
        if self.capture_mode {
            let (input, _) = take_while::<_, _, ()>(|c| c != 0x09u8 as char)(_q)?;
            let (input, _) = take_while::<_, _, ()>(|c| c == 0x09u8 as char)(input)?;
            let (_input, (first_name, _, last_name, _, email, _, _, _, phone)) =
                tuple((
                    person_name,
                    tab,
                    person_name,
                    tab,
                    email,
                    tab,
                    token,
                    tab,
                    token,
                ))(input.as_bytes())?;

            let mut rv = vec![];
            let first_name = str::from_utf8(first_name).unwrap().trim().to_lowercase();
            let last_name = str::from_utf8(last_name).unwrap().trim().to_lowercase();

            rv.push(format!("{} {}", first_name, last_name));
            rv.push(str::from_utf8(&email).unwrap().to_string());
            rv.push(str::from_utf8(&phone).unwrap().to_string());

            return Ok(ParseStatus::Ready(rv.into_iter()));
        }

        return Ok(ParseStatus::Ignored);
    }
}

#[inline(always)]
fn sp(i: &[u8]) -> IResult<&[u8], char> {
    char(' ')(i)
}

#[inline(always)]
fn tab(i: &[u8]) -> IResult<&[u8], char> {
    char(0x09u8 as char)(i)
}

#[inline(always)]
fn is_token_char(i: u8) -> bool {
    is_alphanumeric(i) || b"!#$%&'*+-.^_`|~".contains(&i)
}

#[inline(always)]
fn token(i: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(is_token_char)(i)
}

#[inline(always)]
fn person_name(i: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(|c| is_alphanumeric(c) || c == ' ' as u8)(i)
}

#[inline(always)]
fn email(i: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(|c| is_alphanumeric(c) || b"@._".contains(&c))(i)
}

#[inline(always)]
fn phone(i: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(|c| is_digit(c) || c == '-' as u8)(i)
}
