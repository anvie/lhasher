// Copyleft (C) 2022 Robin Syihab.
// All Rights Reserved.
//
// This code is part of Leak Checker.
//

#![allow(dead_code)]

use nom::{
    bytes::complete::take_while,
    character::{complete::digit0, is_alphanumeric, is_digit, streaming::char},
    IResult,
};

#[inline(always)]
pub(crate) fn sp(i: &str) -> IResult<&str, char> {
    char(' ')(i)
}

// match comma
#[inline(always)]
pub(crate) fn comma(i: &str) -> IResult<&str, char> {
    char(',')(i)
}

#[inline(always)]
pub(crate) fn tab(i: &str) -> IResult<&str, char> {
    char(0x09 as char)(i)
}

#[inline(always)]
pub(crate) fn non_tab(i: &str) -> IResult<&str, &str> {
    take_while(|c| c != 0x09u8 as char)(i)
}

#[inline(always)]
pub(crate) fn is_token_char(i: char) -> bool {
    is_alphanumeric(i as u8) || b"!#$%&'*+-.^_`|~".contains(&(i as u8))
}

#[inline(always)]
pub(crate) fn token(i: &str) -> IResult<&str, &str> {
    take_while(is_token_char)(i)
}

#[inline(always)]
pub(crate) fn person_name(i: &str) -> IResult<&str, &str> {
    take_while(|c| is_alphanumeric(c as u8) || c == ' ')(i)
}

#[inline(always)]
pub(crate) fn email(i: &str) -> IResult<&str, &str> {
    take_while(|c| is_alphanumeric(c as u8) || b"@._".contains(&(c as u8)))(i)
}

#[inline(always)]
pub(crate) fn phone_number(i: &str) -> IResult<&str, &str> {
    take_while(|c| is_digit(c as u8) || c == '-')(i)
}

#[inline(always)]
pub(crate) fn numeric(i: &str) -> IResult<&str, &str> {
    // take_while(|c| is_digit(c as u8) || c == '.')(i)
    digit0(i)
}

#[inline(always)]
fn skip_tab(input: &str) -> IResult<&str, &str> {
    take_while(|c| c == 0x09 as char)(input)
}
