//! Parser that successfully matches the empty string.

use crate::{error::Result, ParseIter, Parser};

#[derive(Clone, Copy)]
pub struct EmptyParser;

impl<'parse, 'source> Parser<'parse, 'source> for EmptyParser {
    type Output = ();
    type RawOutput = ();
    type Iter = EmptyParseIter;

    fn parse_iter(&'parse self, _source: &'source str, start: usize) -> EmptyParseIter {
        EmptyParseIter {
            used: false,
            location: start,
        }
    }
}

pub struct EmptyParseIter {
    used: bool,
    location: usize,
}

impl ParseIter for EmptyParseIter {
    type RawOutput = ();

    fn next_parse(&mut self) -> Option<Result<usize>> {
        if self.used {
            None
        } else {
            self.used = true;
            Some(Ok(self.location))
        }
    }

    fn take_data(&mut self) {}
}

pub fn empty() -> EmptyParser {
    EmptyParser
}