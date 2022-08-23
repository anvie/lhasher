// Copyleft (C) 2022 Robin Syihab.
// All Rights Reserved.
//
// This code is part of Leak Checker.
//
use std::io::Result;
use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

pub struct GeneratorIteratorAdapter<G>(Pin<Box<G>>);

impl<G> GeneratorIteratorAdapter<G>
where
    G: Generator<Return = ()>,
{
    fn new(gen: G) -> Self {
        Self(Box::pin(gen))
    }
}

impl<G> Iterator for GeneratorIteratorAdapter<G>
where
    G: Generator<Return = ()>,
{
    type Item = G::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.as_mut().resume(()) {
            GeneratorState::Yielded(x) => Some(x),
            GeneratorState::Complete(_) => None,
        }
    }
}

pub type GenT = impl Generator<Yield = String, Return = ()>;

pub trait Parser {
    fn name(&self) -> &'static str;
    fn file_out_name(&self) -> &'static str;
    fn parse(line: &str) -> Result<GeneratorIteratorAdapter<GenT>>;
}

mod metranet;

pub use metranet::MetranetLog;
