// Copyleft (C) 2022 Robin Syihab.
// All Rights Reserved.
//
// This code is part of Leak Checker.
//
use std::{io::Result, vec::IntoIter};

pub type Tokens = IntoIter<String>;

pub enum ParseStatus {
    Ready(Tokens),
    Buffer(String),
    Ignored,
    BufferEnd(String),
}

pub type ParseResult = Result<ParseStatus>;

pub trait Parser {
    // type R = ParseResult;

    fn name(&self) -> &'static str;
    fn file_out_name(&self) -> &'static str;
    fn parse(&mut self, line: &str) -> ParseResult;
}

mod bhinneka;
mod metranet;
mod unipanca;

pub use bhinneka::BhinnekaDB;
pub use metranet::MetranetLog;
pub use unipanca::UniPancaDB;
