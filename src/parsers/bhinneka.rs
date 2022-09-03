// Copyleft (C) 2022 Robin Syihab.
// All Rights Reserved.
//
// This code is part of Leak Checker.
//

use std::str;

use crate::parsers::{plats::*, util, ParseResult, ParseStatus, Parser};

use nom::{
    bytes::complete::{tag, take_while},
    character::{is_alphabetic, is_alphanumeric},
    IResult,
};

pub struct BhinnekaDB {}

impl BhinnekaDB {
    pub fn new() -> BhinnekaDB {
        Self {}
    }
}

impl Parser for BhinnekaDB {
    fn name(&self) -> &'static str {
        "Bhinneka e-commerce database"
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

        let (input, _) = take_while::<_, _, ()>(|c| c != 0x09u8 as char)(_q)?;
        let (input, _) = tab(input)?;
        let (input, first_name) = person_name(input)?;
        let (input, _) = tab(input)?;
        let (input, last_name) = person_name(input)?;
        let (input, _) = tab(input)?;
        let (input, _email) = email(input)?;
        let (input, _) = tab(input)?;
        let (input, _) = gender(input)?;

        let mut rv = vec![_email.to_uppercase()];

        let mut input = input;

        // gather phone information
        (input, _) = tab(input)?;
        if let Ok((_input, mobile)) = phone_number(input) {
            input = _input;
            let _num = util::normalize_phone_number(mobile).trim().to_string();
            if !_num.is_empty() {
                rv.push(_num);
            }
        }
        (input, _) = tab(input)?;
        if let Ok((_input, phone)) = phone_number(input) {
            let _num = util::normalize_phone_number(phone).trim().to_string();
            if !_num.is_empty() {
                rv.push(_num);
            }
        }

        let full_name = format!("{} {}", first_name, last_name)
            .trim()
            .to_uppercase();
        rv.push(full_name);

        return Ok(ParseStatus::Ready(rv.into_iter()));
    }
}

fn gender(input: &str) -> IResult<&str, &str> {
    take_while(|c| is_alphabetic(c as u8) || b"\\N".contains(&(c as u8)))(input)
}
