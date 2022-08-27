// Copyleft (C) 2022 Robin Syihab.
// All Rights Reserved.
//
// This code is part of Leak Checker.
//
use std::io::Result;
use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

pub trait Parser {
    type R = std::vec::IntoIter<String>;

    fn name(&self) -> &'static str;
    fn file_out_name(&self) -> &'static str;
    fn parse(line: &str) -> Result<Self::R>;
}

mod metranet;

pub use metranet::MetranetLog;
