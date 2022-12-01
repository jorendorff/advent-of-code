use std::{
    any::{self, Any},
    fmt::Display,
    num::ParseIntError,
    str::FromStr,
};

use crate::{
    types::{Never, ParserOutput, TupleConcat},
    ParseError, Result,
};
use lazy_static::lazy_static;
use regex::Regex;

/// Trait implemented by all parsers.
///
/// This is implemented by the built-in parsers, like `i32`, as well as
/// user-defined parsers created with `parser!`.
///
/// To run a parser, pass some text to [the `parse` method][Parser::parse].
pub trait Parser<'parse, 'source> {
    /// The type of value this parser produces from text.
    type Output;

    /// The type this parser produces internally before converting to the output type.
    ///
    /// Some combinators use the `RawOutput` to determine how types should combine.
    /// For example, if `A::RawOutput = ()`, then `A` produces no output;
    /// and if `B::RawOutput = (i32,)` then `B` produces an integer;
    /// `SequenceParser<A, B>::RawOutput` will then be `(i32,)`, the
    /// result of concatenating the two raw tuples, rather than `((), i32)`.
    ///
    /// However, `RawOutput` is very often a singleton tuple, and these are
    /// awkward for users, so we convert to the `Output` type before presenting a
    /// result to the user.
    type RawOutput: ParserOutput<UserType = Self::Output>;

    /// The type that implements matching, backtracking, and type conversion
    /// for this parser, an implementation detail.
    type Iter: ParseIter<RawOutput = Self::RawOutput>;

    /// Fully parse the given source string `s` and return the resulting value.
    ///
    /// This is the main way of using a `Parser`.
    ///
    /// This succeeds only if this parser matches the entire input string. It's
    /// an error if any unmatched characters are left over at the end of `s`.
    fn parse(&'parse self, s: &'source str) -> Result<Self::Output> {
        self.parse_raw(s).map(|v| v.into_user_type())
    }

    /// Produce a [parse iterator][ParseIter]. This is an internal implementation detail of
    /// the parser and shouldn't normally be called directly from application code.
    fn parse_iter(&'parse self, source: &'source str, start: usize) -> Self::Iter;

    /// Like `parse` but produce the output in its [raw form][Self::RawOutput].
    fn parse_raw(&'parse self, s: &'source str) -> Result<Self::RawOutput> {
        let mut it = self.parse_iter(s, 0);
        let mut best_end: Option<usize> = None;
        while let Some(parse) = it.next_parse() {
            let end = parse?;
            if end == s.len() {
                return Ok(it.take_data());
            } else {
                best_end = best_end.max(Some(end));
            }
        }
        if let Some(end) = best_end {
            Err(ParseError::new_extra(s, end))
        } else {
            panic!("parse iterator broke the contract: no matches and no error");
        }
    }

    /// Like [`.map()`][Self::map], but operate on the
    /// [`RawOutput`][Self::RawOutput] type.
    ///
    /// `.map()` always produces a 1-element tuple, but `.map_raw()` can return
    /// `()` to indicate that the matching input should be ignored entirely
    /// instead of creating a value.
    fn map_raw<T, F>(self, mapper: F) -> MapRawParser<Self, F>
    where
        Self: Sized,
        F: Fn(Self::RawOutput) -> T,
    {
        MapRawParser {
            parser: self,
            mapper,
        }
    }

    /// Produce a new parser that behaves like this parser but additionally
    /// applies the given closure when producing the value.
    ///
    /// ```
    /// use aoc_parse::{parser, prelude::*};
    /// let p = u32.map(|x| x * 1_000_001);
    /// assert_eq!(p.parse("123").unwrap(), 123_000_123);
    /// ```
    ///
    /// This is used to implement the `=>` feature of `parser!`.
    ///
    /// ```
    /// # use aoc_parse::{parser, prelude::*};
    /// let p = parser!(x: u32 => x * 1_000_001);
    /// assert_eq!(p.parse("123").unwrap(), 123_000_123);
    /// ```
    ///
    /// The closure is called after the *overall* parse succeeds, as part of
    /// turning the parse into Output values. This means the function
    /// will not be called during a partly-successful parse that later fails.
    ///
    /// ```
    /// # use aoc_parse::{parser, prelude::*};
    /// let p = parser!(("A" => panic!()) "B" "C");
    /// assert!(p.parse("ABX").is_err());
    ///
    /// let p2 = parser!({
    ///    (i32 => panic!()) " ft" => 1,
    ///    i32 " km" => 2,
    /// });
    /// assert_eq!(p2.parse("37 km").unwrap(), 2);
    /// ```
    fn map<T, F>(self, mapper: F) -> MapParser<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Output) -> T,
    {
        MapParser {
            parser: self,
            mapper,
        }
    }
}

/// Parse the given puzzle input supplied by `#[aoc_generator]`.
///
/// This function is like `parser.parse(puzzle_input)` except that
/// `#[aoc_generator]` unfortunately [strips off trailing newlines][bad]. This
/// function therefore adds a newline at the end before parsing.
///
/// # Example
///
/// ```no_run
/// use aoc_runner_derive::*;
/// use aoc_parse::{parser, prelude::*};
///
/// #[aoc_generator(day1)]
/// fn parse_input(text: &str) -> anyhow::Result<Vec<Vec<u64>>> {
///     let p = parser!(repeat_sep(lines(u64), "\n"));
///     aoc_parse(text, p)
/// }
/// ```
///
/// [bad]: https://github.com/gobanos/aoc-runner/blob/master/src/lib.rs#L17
pub fn aoc_parse<P, T, E>(puzzle_input: &str, parser: P) -> std::result::Result<T, E>
where
    E: From<ParseError>,
    P: for<'p, 's> Parser<'p, 's, Output = T>,
{
    let p = puzzle_input.to_string() + "\n";
    Ok(parser.parse(&p)?)
}

/// A parser in action. Some parsers can match in several different ways (for
/// example, in `foo* bar` backtracking is accomplished by `foo*` first
/// matching as much as possible, then backing off one match at a time), so
/// this is an iterator.
///
/// This doesn't return a `RawOutput` value from `next_parse` but instead waits
/// until you're sure you have a complete, successful parse, and are thus ready
/// to destroy the iterator. This helps us avoid building values only to drop
/// them later when some downstream parser fails to match, so it makes
/// backtracking faster. It also means we don't call `.map` closures until
/// there is a successful overall match and the values are actually needed.
pub trait ParseIter {
    /// The type this iterator can produce on a successful match.
    type RawOutput;

    /// Try parsing the input.
    ///
    /// The first time this is called, it should return either `Some(Ok(end))`
    /// or `Some(Err(err))` indicating that parsing either succeeded or failed.
    ///
    /// Subsequently, it should return either `Some(Ok(end))` or `Some(None)`
    /// to indicate that there either is or isn't another, less preferable
    /// match.
    fn next_parse(&mut self) -> Option<Result<usize>>;

    /// Consume this iterator to extract data. This is called only after a
    /// successful `next_parse` call that returns `Some(Ok(offset))`.
    ///
    /// This would take `self` by value, except that's not compatible with
    /// trait objects. (`Box<Self>` is, so this could change someday.)
    fn take_data(&mut self) -> Self::RawOutput;
}

// --- Parser that successfully matches the empty string

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

// --- Parser that never matches anything

pub struct NeverParser;

pub struct NeverParseIter<'source> {
    source: &'source str,
    start: usize,
}

impl<'parse, 'source> Parser<'parse, 'source> for NeverParser {
    type Output = Never;
    type RawOutput = Never;
    type Iter = NeverParseIter<'source>;

    fn parse_iter(&'parse self, source: &'source str, start: usize) -> NeverParseIter<'source> {
        NeverParseIter { source, start }
    }
}

impl<'source> ParseIter for NeverParseIter<'source> {
    type RawOutput = Never;

    fn next_parse(&mut self) -> Option<Result<usize>> {
        Some(Err(ParseError::new_cannot_match(self.source, self.start)))
    }

    fn take_data(&mut self) -> Never {
        unreachable!("never matches");
    }
}

// --- Parser that matches a particular exact string

#[derive(Clone, Copy)]
pub struct ExactParser {
    s: &'static str,
}

pub struct ExactParseIter<'parse, 'source> {
    expected: &'parse str,
    input: &'source str,
    start: usize,
    done: bool,
}

impl<'parse, 'source> Parser<'parse, 'source> for ExactParser {
    type Output = ();
    type RawOutput = ();
    type Iter = ExactParseIter<'parse, 'source>;

    fn parse_iter(
        &'parse self,
        source: &'source str,
        start: usize,
    ) -> ExactParseIter<'parse, 'source> {
        ExactParseIter {
            expected: &self.s,
            input: source,
            start,
            done: false,
        }
    }
}

impl<'parse, 'source> ParseIter for ExactParseIter<'parse, 'source> {
    type RawOutput = ();

    fn next_parse(&mut self) -> Option<Result<usize>> {
        if self.done {
            None
        } else if self.input[self.start..].starts_with(self.expected) {
            self.done = true;
            Some(Ok(self.start + self.expected.len()))
        } else {
            Some(Err(ParseError::new_expected(
                self.input,
                self.start,
                self.expected,
            )))
        }
    }

    fn take_data(&mut self) {}
}

// --- Matching patterns in sequence

#[derive(Clone, Copy)]
pub struct SequenceParser<Head, Tail> {
    head: Head,
    tail: Tail,
}

pub struct SequenceParseIter<'parse, 'source, Head, Tail>
where
    Head: Parser<'parse, 'source>,
    Tail: Parser<'parse, 'source>,
{
    parsers: &'parse SequenceParser<Head, Tail>,
    is_at_start: bool,
    source: &'source str,
    start: usize,
    head_iter: Option<Head::Iter>,
    tail_iter: Option<Tail::Iter>,
}

impl<'parse, 'source, Head, Tail> Parser<'parse, 'source> for SequenceParser<Head, Tail>
where
    Head: Parser<'parse, 'source> + 'parse,
    Tail: Parser<'parse, 'source> + 'parse,
    Head::RawOutput: TupleConcat<Tail::RawOutput>,
{
    type Output =
        <<Head::RawOutput as TupleConcat<Tail::RawOutput>>::Output as ParserOutput>::UserType;
    type RawOutput = <Head::RawOutput as TupleConcat<Tail::RawOutput>>::Output;
    type Iter = SequenceParseIter<'parse, 'source, Head, Tail>;

    fn parse_iter(&'parse self, source: &'source str, start: usize) -> Self::Iter {
        SequenceParseIter {
            parsers: self,
            is_at_start: true,
            source,
            start,
            head_iter: None,
            tail_iter: None,
        }
    }
}

impl<'parse, 'source, Head, Tail> ParseIter for SequenceParseIter<'parse, 'source, Head, Tail>
where
    Head: Parser<'parse, 'source>,
    Tail: Parser<'parse, 'source>,
    Head::RawOutput: TupleConcat<Tail::RawOutput>,
{
    type RawOutput = <Head::RawOutput as TupleConcat<Tail::RawOutput>>::Output;

    fn next_parse(&mut self) -> Option<Result<usize>> {
        let mut foremost_error: Option<ParseError> = None;
        loop {
            if let Some(tail_iter) = &mut self.tail_iter {
                match tail_iter.next_parse() {
                    None => {}
                    Some(Err(err)) => {
                        if foremost_error.as_ref().map(|err| err.location) < Some(err.location) {
                            foremost_error = Some(err);
                        }
                    }

                    Some(Ok(tail_end)) => return Some(Ok(tail_end)),
                }
                self.tail_iter = None;
            } else if let Some(head_iter) = &mut self.head_iter {
                match head_iter.next_parse() {
                    None => {}
                    Some(Err(err)) => {
                        if foremost_error.as_ref().map(|err| err.location) < Some(err.location) {
                            foremost_error = Some(err);
                        }
                    }
                    Some(Ok(head_end)) => {
                        self.tail_iter = Some(self.parsers.tail.parse_iter(self.source, head_end));
                        continue;
                    }
                }
                self.head_iter = None;
                return foremost_error.map(Err);
            } else if self.is_at_start {
                self.is_at_start = false;
                self.head_iter = Some(self.parsers.head.parse_iter(self.source, self.start));
            } else {
                return None;
            }
        }
    }

    fn take_data(&mut self) -> Self::RawOutput {
        let head = self.head_iter.as_mut().unwrap().take_data();
        let tail = self.tail_iter.as_mut().unwrap().take_data();
        head.concat(tail)
    }
}

// --- Alternation

#[derive(Debug)]
pub enum Either<A, B> {
    Left(A),
    Right(B),
}

#[derive(Copy, Clone)]
pub struct EitherParser<A, B> {
    left: A,
    right: B,
}

pub struct EitherParseIter<'parse, 'source, A, B>
where
    A: Parser<'parse, 'source>,
    B: Parser<'parse, 'source>,
{
    source: &'source str,
    start: usize,
    parsers: &'parse EitherParser<A, B>,
    iter: Either<A::Iter, B::Iter>,
}

impl<'parse, 'source, A, B> Parser<'parse, 'source> for EitherParser<A, B>
where
    A: Parser<'parse, 'source> + 'parse,
    B: Parser<'parse, 'source> + 'parse,
{
    type Output = Either<A::Output, B::Output>;
    type RawOutput = (Either<A::Output, B::Output>,);
    type Iter = EitherParseIter<'parse, 'source, A, B>;

    fn parse_iter(&'parse self, source: &'source str, start: usize) -> Self::Iter {
        EitherParseIter {
            source,
            start,
            parsers: self,
            iter: Either::Left(self.left.parse_iter(source, start)),
        }
    }
}

impl<'parse, 'source, A, B> ParseIter for EitherParseIter<'parse, 'source, A, B>
where
    A: Parser<'parse, 'source>,
    B: Parser<'parse, 'source>,
{
    type RawOutput = (Either<A::Output, B::Output>,);

    fn next_parse(&mut self) -> Option<Result<usize>> {
        let mut foremost_error: Option<ParseError> = None;
        loop {
            match &mut self.iter {
                Either::Left(iter) => {
                    match iter.next_parse() {
                        None => {}
                        Some(Err(err)) => {
                            if Some(err.location) > foremost_error.as_ref().map(|err| err.location)
                            {
                                foremost_error = Some(err);
                            }
                        }
                        Some(Ok(end)) => return Some(Ok(end)),
                    }
                    self.iter =
                        Either::Right(self.parsers.right.parse_iter(self.source, self.start));
                }
                Either::Right(iter) => {
                    match iter.next_parse() {
                        None => {}
                        Some(Err(err)) => {
                            if Some(err.location) > foremost_error.as_ref().map(|err| err.location)
                            {
                                foremost_error = Some(err);
                            }
                        }
                        Some(Ok(end)) => return Some(Ok(end)),
                    }
                    return foremost_error.map(Err);
                }
            }
        }
    }

    fn take_data(&mut self) -> (Either<A::Output, B::Output>,) {
        (match &mut self.iter {
            Either::Left(iter) => Either::Left(iter.take_data().into_user_type()),
            Either::Right(iter) => Either::Right(iter.take_data().into_user_type()),
        },)
    }
}

// --- Parsing a repeated pattern

#[derive(Clone, Copy)]
pub struct RepeatParser<Pattern, Sep> {
    pattern: Pattern,
    min: usize,
    max: Option<usize>,
    sep: Sep,
    sep_is_terminator: bool,
}

pub struct RepeatParseIter<'parse, 'source, Pattern, Sep>
where
    Pattern: Parser<'parse, 'source>,
    Sep: Parser<'parse, 'source>,
{
    source: &'source str,
    params: &'parse RepeatParser<Pattern, Sep>,
    pattern_iters: Vec<Pattern::Iter>,
    sep_iters: Vec<Sep::Iter>,
    starts: Vec<usize>,
}

impl<'parse, 'source, Pattern, Sep> Parser<'parse, 'source> for RepeatParser<Pattern, Sep>
where
    Pattern: Parser<'parse, 'source> + 'parse,
    Sep: Parser<'parse, 'source> + 'parse,
{
    type Output = Vec<Pattern::Output>;
    type RawOutput = (Vec<Pattern::Output>,);
    type Iter = RepeatParseIter<'parse, 'source, Pattern, Sep>;

    fn parse_iter(&'parse self, source: &'source str, start: usize) -> Self::Iter {
        RepeatParseIter {
            source,
            params: self,
            pattern_iters: vec![self.pattern.parse_iter(source, start)],
            sep_iters: vec![],
            starts: vec![start],
        }
    }
}

impl<Pattern, Sep> RepeatParser<Pattern, Sep> {
    fn check_repeat_count(&self, count: usize) -> bool {
        let expected_parity = !self.sep_is_terminator as usize;
        let nmatches = (count + expected_parity) / 2;
        (count == 0 || count % 2 == expected_parity)
            && self.min <= nmatches
            && match self.max {
                None => true,
                Some(max) => nmatches <= max,
            }
    }
}

impl<'parse, 'source, Pattern, Sep> ParseIter for RepeatParseIter<'parse, 'source, Pattern, Sep>
where
    Pattern: Parser<'parse, 'source> + 'parse,
    Sep: Parser<'parse, 'source> + 'parse,
{
    type RawOutput = (Vec<Pattern::Output>,);

    fn next_parse(&mut self) -> Option<Result<usize>> {
        // TODO: When considering creating a new iterator, if we have already
        // matched `max` times, don't bother; no matches can come of it.
        let mut foremost_error: Option<ParseError> = None;
        let mut got_iter = true;
        loop {
            assert_eq!(self.pattern_iters.len(), (self.starts.len() + 1) / 2);
            assert_eq!(self.sep_iters.len(), self.starts.len() / 2);
            if got_iter {
                let next_parse = if self.starts.is_empty() {
                    // No more iterators. We exhausted all possibilities.
                    return foremost_error.map(Err);
                } else if self.starts.len() % 2 == 1 {
                    self.pattern_iters.last_mut().unwrap().next_parse()
                } else {
                    self.sep_iters.last_mut().unwrap().next_parse()
                };

                // Get the next match.
                match next_parse {
                    None => {
                        got_iter = false;
                    }
                    Some(Err(err)) => {
                        got_iter = false;
                        if Some(err.location) > foremost_error.as_ref().map(|err| err.location) {
                            foremost_error = Some(err);
                        }
                    }
                    Some(Ok(point)) => {
                        // Got a match! But don't return it to the user yet.
                        // Repeats are "greedy"; we press on to see if we can
                        // match again! If we just matched `pattern`, try
                        // `sep`; if we just matched `sep`, try `pattern`.
                        self.starts.push(point);
                        if self.starts.len() % 2 == 1 {
                            self.pattern_iters
                                .push(self.params.pattern.parse_iter(self.source, point));
                        } else {
                            self.sep_iters
                                .push(self.params.sep.parse_iter(self.source, point));
                        }
                    }
                }
            } else {
                // The current top-of-stack iterator is exhausted and needs to
                // be discarded.
                if self.starts.len() % 2 == 1 {
                    self.pattern_iters.pop();
                } else {
                    self.sep_iters.pop();
                }
                let end = self.starts.pop().unwrap();
                got_iter = true;

                // Repeats are "greedy", so we need to yield the longest match
                // first. This means returning only "on the way out" (a
                // postorder walk of the tree of possible parses).
                if self.params.check_repeat_count(self.starts.len()) {
                    return Some(Ok(end));
                }
            }
        }
    }

    fn take_data(&mut self) -> (Vec<Pattern::Output>,) {
        self.starts.truncate(0);
        self.sep_iters.truncate(0);
        let v = self
            .pattern_iters
            .split_off(0)
            .iter_mut()
            .map(|iter| iter.take_data().into_user_type())
            .collect();
        (v,)
    }
}

// --- Mapping parsers

pub struct MapRawParser<P, F> {
    parser: P,
    mapper: F,
}

pub struct MapRawParseIter<'parse, 'source, P, F>
where
    P: Parser<'parse, 'source>,
{
    iter: P::Iter,
    mapper: &'parse F,
}

impl<'parse, 'source, P, F, T> Parser<'parse, 'source> for MapRawParser<P, F>
where
    P: Parser<'parse, 'source>,
    F: Fn(P::RawOutput) -> T,
    F: 'parse,
    T: ParserOutput,
{
    type Output = <T as ParserOutput>::UserType;
    type RawOutput = T;
    type Iter = MapRawParseIter<'parse, 'source, P, F>;

    fn parse_iter(&'parse self, source: &'source str, start: usize) -> Self::Iter {
        MapRawParseIter {
            iter: self.parser.parse_iter(source, start),
            mapper: &self.mapper,
        }
    }
}

impl<'parse, 'source, P, F, T> ParseIter for MapRawParseIter<'parse, 'source, P, F>
where
    P: Parser<'parse, 'source>,
    F: Fn(P::RawOutput) -> T,
{
    type RawOutput = T;

    fn next_parse(&mut self) -> Option<Result<usize>> {
        self.iter.next_parse()
    }

    fn take_data(&mut self) -> T {
        (self.mapper)(self.iter.take_data())
    }
}

#[derive(Clone, Copy)]
pub struct MapParser<P, F> {
    parser: P,
    mapper: F,
}

pub struct MapParseIter<'parse, 'source, P, F>
where
    P: Parser<'parse, 'source>,
{
    iter: P::Iter,
    mapper: &'parse F,
}

impl<'parse, 'source, P, F, T> Parser<'parse, 'source> for MapParser<P, F>
where
    P: Parser<'parse, 'source>,
    F: Fn(P::Output) -> T,
    F: 'parse,
{
    type Output = T;
    type RawOutput = (T,);
    type Iter = MapParseIter<'parse, 'source, P, F>;

    fn parse_iter(&'parse self, source: &'source str, start: usize) -> Self::Iter {
        MapParseIter {
            iter: self.parser.parse_iter(source, start),
            mapper: &self.mapper,
        }
    }
}

impl<'parse, 'source, P, F, T> ParseIter for MapParseIter<'parse, 'source, P, F>
where
    P: Parser<'parse, 'source>,
    F: Fn(P::Output) -> T,
{
    type RawOutput = (T,);

    fn next_parse(&mut self) -> Option<Result<usize>> {
        self.iter.next_parse()
    }

    fn take_data(&mut self) -> (T,) {
        let value = (self.mapper)(self.iter.take_data().into_user_type());
        (value,)
    }
}

// --- Parsers using Regex

pub struct RegexParser<T, E> {
    regex: fn() -> &'static Regex,
    parse_fn: fn(&str) -> std::result::Result<T, E>,
}

// Manual Clone impl because `#[derive(Clone)]` is buggy in this case.
impl<T, E> Clone for RegexParser<T, E> {
    fn clone(&self) -> Self {
        RegexParser {
            regex: self.regex,
            parse_fn: self.parse_fn,
        }
    }
}

impl<T, E> Copy for RegexParser<T, E> {}

pub enum RegexParseIter<'parse, 'source, T, E> {
    Init {
        source: &'source str,
        start: usize,
        parser: &'parse RegexParser<T, E>,
    },
    Done {
        value: Option<T>,
    },
}

impl<'parse, 'source, T, E> Parser<'parse, 'source> for RegexParser<T, E>
where
    T: 'static,
    E: Display + 'parse,
{
    type Output = T;
    type RawOutput = (T,);
    type Iter = RegexParseIter<'parse, 'source, T, E>;

    fn parse_iter(&'parse self, source: &'source str, start: usize) -> Self::Iter {
        RegexParseIter::Init {
            source,
            start,
            parser: self,
        }
    }
}

impl<'parse, 'source, T, E> ParseIter for RegexParseIter<'parse, 'source, T, E>
where
    T: Any,
    E: Display,
{
    type RawOutput = (T,);

    fn next_parse(&mut self) -> Option<Result<usize>> {
        match *self {
            RegexParseIter::Init {
                source,
                start,
                parser,
            } => match (parser.regex)().find(&source[start..]) {
                Some(m) => match (parser.parse_fn)(m.as_str()) {
                    Ok(value) => {
                        *self = RegexParseIter::Done { value: Some(value) };
                        Some(Ok(start + m.end()))
                    }
                    Err(err) => {
                        *self = RegexParseIter::Done { value: None };
                        Some(Err(ParseError::new_from_str_failed(
                            source,
                            start,
                            start + m.end(),
                            any::type_name::<T>(),
                            format!("{err}"),
                        )))
                    }
                },
                None => {
                    *self = RegexParseIter::Done { value: None };
                    Some(Err(ParseError::new_expected(
                        source,
                        start,
                        any::type_name::<T>(),
                    )))
                }
            },
            _ => None,
        }
    }

    fn take_data(&mut self) -> Self::RawOutput {
        let v = match self {
            RegexParseIter::Done { value } => value.take().unwrap(),
            _ => unreachable!("matching failed"),
        };
        (v,)
    }
}

// --- Default parsers for some types that implement FromStr

macro_rules! regexes {
    ( $( $name:ident = $re:expr ; )* ) => {
        $(
            fn $name() -> &'static Regex {
                lazy_static! {
                    static ref RE: Regex = Regex::new($re).unwrap();
                }
                &RE
            }
        )*
    }
}

regexes! {
    uint_regex = r"\A(0|[1-9][0-9]*)";
    int_regex = r"\A(?:0|[+-]?[1-9][0-9]*)";
    bool_regex = r"true|false";
    uint_bin_regex = r"\A[01]+";
    int_bin_regex = r"\A[+-]?[01]+";
}

macro_rules! from_str_parse_impl {
    ( $( $ty:ident )+ , $re_name:ident) => {
        $(
            #[allow(non_upper_case_globals)]
            pub const $ty: RegexParser<$ty, <$ty as FromStr>::Err> =
                RegexParser {
                    regex: $re_name,
                    parse_fn: <$ty as FromStr>::from_str,
                };
        )+
    };
}

from_str_parse_impl!(u8 u16 u32 u64 u128 usize, uint_regex);
from_str_parse_impl!(i8 i16 i32 i64 i128 isize, int_regex);
from_str_parse_impl!(bool, bool_regex);

macro_rules! from_str_radix_parsers {
    ( $( ( $ty:ident , $bin:ident , $hex:ident ) ),* : $re_name:ident ) => {
        $(
            #[allow(non_upper_case_globals)]
            pub const $bin: RegexParser<$ty, ParseIntError> = RegexParser {
                regex: $re_name,
                parse_fn: |s| $ty::from_str_radix(s, 2),
            };

            #[allow(non_upper_case_globals)]
            pub const $hex: RegexParser<$ty, ParseIntError> = RegexParser {
                regex: $re_name,
                parse_fn: |s| $ty::from_str_radix(s, 16),
            };

        )*
    }
}

from_str_radix_parsers!(
    (u8, u8_bin, u8_hex),
    (u16, u16_bin, u16_hex),
    (u32, u32_bin, u32_hex),
    (u64, u64_bin, u64_hex),
    (u128, u128_bin, u128_hex),
    (usize, usize_bin, usize_hex): uint_bin_regex
);

from_str_radix_parsers!(
    (i8, i8_bin, i8_hex),
    (i16, i16_bin, i16_hex),
    (i32, i32_bin, i32_hex),
    (i64, i64_bin, i64_hex),
    (i128, i128_bin, i128_hex),
    (isize, isize_bin, isize_hex): int_bin_regex
);

// --- Wrappers

pub fn empty() -> EmptyParser {
    EmptyParser
}

pub fn exact(s: &'static str) -> ExactParser {
    ExactParser { s }
}

pub fn sequence<Head, Tail>(head: Head, tail: Tail) -> SequenceParser<Head, Tail> {
    SequenceParser { head, tail }
}

// pub fn sequence(parsers: impl ParserTuple) -> impl for<'parse, 'source> Parser<'parse, 'source> {
//     parsers.product()
// }

pub fn either<A, B>(left: A, right: B) -> EitherParser<A, B> {
    EitherParser { left, right }
}

pub fn alt<T>(
    left: impl for<'parse, 'source> Parser<'parse, 'source, Output = T> + 'static,
    right: impl for<'parse, 'source> Parser<'parse, 'source, Output = T> + 'static,
) -> impl for<'parse, 'source> Parser<'parse, 'source, Output = T, RawOutput = (T,)> + 'static {
    EitherParser { left, right }.map(|out| match out {
        Either::Left(value) => value,
        Either::Right(value) => value,
    })
}

// pub fn one_of(parsers: impl ParserTuple) -> impl for<'parse, 'source> Parser<'parse, 'source> {
//     parsers.sum()
// }

pub fn repeat<Pattern, Sep>(
    pattern: Pattern,
    sep: Sep,
    min: usize,
    max: Option<usize>,
    sep_is_terminator: bool,
) -> RepeatParser<Pattern, Sep> {
    RepeatParser {
        pattern,
        min,
        max,
        sep,
        sep_is_terminator,
    }
}

pub fn opt<T>(
    pattern: impl for<'parse, 'source> Parser<'parse, 'source, Output = T> + 'static,
) -> impl for<'parse, 'source> Parser<'parse, 'source, Output = Option<T>, RawOutput = (Option<T>,)>
{
    either(pattern, empty()).map(|e: Either<T, ()>| match e {
        Either::Left(left) => Some(left),
        Either::Right(()) => None,
    })
}

// Kleene *
pub fn star<Pattern>(pattern: Pattern) -> RepeatParser<Pattern, EmptyParser> {
    repeat(pattern, empty(), 0, None, false)
}

// Kleene +
pub fn plus<Pattern>(pattern: Pattern) -> RepeatParser<Pattern, EmptyParser> {
    repeat(pattern, empty(), 1, None, false)
}

pub fn sep_by<Pattern, Sep>(pattern: Pattern, sep: Sep) -> RepeatParser<Pattern, Sep> {
    repeat(pattern, sep, 0, None, false)
}

pub fn lines<Pattern>(pattern: Pattern) -> RepeatParser<Pattern, ExactParser> {
    repeat(pattern, exact("\n"), 0, None, true)
}

// Make sure that RawOutput is exactly `(T,)`.
//
// Parenthesizing an expression makes a semantic difference to prevent it from
// disappearing in concatenation.
//
// Example 1: In `parser!("hello " (x: i32) => x)` the raw output type of `"hello "` is `()`
// and it disappears when concatenated with `(x: i32)`. Now if we label `"hello"`
// `parser!((a: "hello ") (x: i32) => (a, x))` we have to make sure that doesn't happen
// so that we can build a pattern that matches both `a` and `x`.
//
// Example 2: `parser!((i32 " " i32) " " (x: i32))` should have the output type `((i32, i32), i32)`;
// but conatenating the three top-level RawOutput types, `(i32, i32)` `()` and `(i32,)`, would
// produce the flat `(i32, i32, i32)` instead.
//
// It turns out all we need is to ensure the `RawOutput` type of the parenthesized parser is
// a singleton tuple type.
pub fn parenthesize<A, T>(pattern: A) -> MapParser<A, fn(T) -> T>
where
    A: for<'parse, 'source> Parser<'parse, 'source, Output = T>,
{
    pattern.map(|val| val)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[track_caller]
    fn assert_parse<'s, P>(parser: &'s P, s: &'s str)
    where
        P: Parser<'s, 's>,
    {
        if let Err(err) = parser.parse(s) {
            panic!("parse failed: {}", err);
        }
    }

    #[track_caller]
    fn assert_parse_eq<'s, P, E>(parser: &'s P, s: &'s str, expected: E)
    where
        P: Parser<'s, 's>,
        P::Output: PartialEq<E> + Debug,
        E: Debug,
    {
        match parser.parse(s) {
            Err(err) => panic!("parse failed: {}", err),
            Ok(val) => assert_eq!(val, expected),
        }
    }

    #[track_caller]
    fn assert_no_parse<'s, P>(parser: &'s P, s: &'s str)
    where
        P: Parser<'s, 's>,
        P::Output: Debug,
    {
        if let Ok(m) = parser.parse(s) {
            panic!("expected no match, got: {:?}", m);
        }
    }

    #[test]
    fn test_parse() {
        let p = empty();
        assert_parse_eq(&p, "", ());
        assert_no_parse(&p, "x");

        let p = exact("ok");
        assert_parse(&p, "ok");
        assert_no_parse(&p, "");
        assert_no_parse(&p, "o");
        assert_no_parse(&p, "nok");

        let p = sequence(exact("ok"), exact("go"));
        assert_parse(&p, "okgo");
        assert_no_parse(&p, "ok");
        assert_no_parse(&p, "go");
        assert_no_parse(&p, "");

        let p = either(empty(), exact("ok"));
        assert_parse(&p, "");
        assert_parse(&p, "ok");
        assert_no_parse(&p, "okc");
        assert_no_parse(&p, "okok");

        let p = star(exact("a"));
        assert_parse(&p, "");
        assert_parse(&p, "a");
        assert_parse(&p, "aa");
        assert_parse(&p, "aaa");
        assert_no_parse(&p, "b");
        assert_no_parse(&p, "ab");
        assert_no_parse(&p, "ba");

        let p = sep_by(exact("cow"), exact(","));
        assert_parse(&p, "");
        assert_parse(&p, "cow");
        assert_parse(&p, "cow,cow");
        assert_parse(&p, "cow,cow,cow");
        assert_no_parse(&p, "cowcow");
        assert_no_parse(&p, "cow,");
        assert_no_parse(&p, "cow,,cow");
        assert_no_parse(&p, "cow,cow,");
        assert_no_parse(&p, ",");

        let p = plus(exact("a"));
        assert_no_parse(&p, "");
        assert_parse(&p, "a");
        assert_parse(&p, "aa");

        let p = sep_by(usize, exact(","));
        assert_parse_eq(&p, "11417,0,0,334", vec![11417usize, 0, 0, 334]);

        assert_no_parse(&u8, "256");

        assert_parse_eq(&u8, "255", 255u8);
        assert_parse_eq(&sequence(exact("#"), u32), "#100", 100u32);
        assert_parse_eq(
            &sequence(exact("forward "), u64).map(|a| a),
            "forward 1234",
            1234u64,
        );
    }
}
