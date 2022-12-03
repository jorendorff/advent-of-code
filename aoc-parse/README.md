# aoc-parse

A parser library designed for Advent of Code.

This library mainly provides a macro, `parser!`, that lets you write
a custom parser for your [AoC] puzzle input in seconds.

For example, my puzzle input for [December 2, 2015][example] looked like this:

```
4x23x21
22x29x19
11x4x11
8x10x5
24x18x16
...
```

The parser for this format is a one-liner:
`parser!(lines(u64 "x" u64 "x" u64))`.

## How to use aoc-parse

**If you are NOT using [aoc-runner],** you can use aoc-parse like this:

```rust
use aoc_parse::{parser, prelude::*};

let p = parser!(lines(u64 "x" u64 "x" u64));
assert_eq!(
    p.parse("4x23x21\n22x29x19\n").unwrap(),
    vec![(4, 23, 21), (22, 29, 19)]
);
```

**If you ARE using aoc-runner,** do this instead:

```rust
use aoc_runner_derive::*;

#[aoc_generator(day2)]
fn parse_input(text: &str) -> anyhow::Result<Vec<(u64, u64, u64)>> {
    use aoc_parse::{parser, prelude::*};
    let p = parser!(lines(u64 "x" u64 "x" u64));
    aoc_parse(text, p)
}

assert_eq!(
    parse_input("4x23x21\n22x29x19").unwrap(),
    vec![(4, 23, 21), (22, 29, 19)]
);
```

## Patterns

The argument you need to pass to the `parser!` macro is a *pattern*;
all aoc-parse does is **match** strings against your chosen pattern
and **convert** them into Rust values.

Here are the pieces that you can use in a pattern:

*   `i8`, `i16`, `i32`, `i64`, `i128`, `isize`: These match an integer,
    written out using decimal digits, with an optional `+` or `-` sign
    at the start, like `0` or `-11474`.

    It's an error if the string contains a number too big to fit in the
    type you chose. For example, `parser!(i8).parse("1000")` is an error.
    (It matches the string, but fails during the "convert" phase.)

*   `u8`, `u16`, `u32`, `u64`, `u128`, `usize`: The same, but without
    the sign.


*   `bool` - Matches either `true` or `false` and converts it to the
    corresponding `bool` value.

*   `"x"` - A Rust string, in quotes, is a pattern that matches that exact
    string only.

    Exact patterns don't produce a value.

*   <code><var>pattern1 pattern2 pattern3</var>...</code> - Patterns can be
    concatenated to form larger patterns. This is how
    `parser!(u64 "x" u64 "x" u64)` matches the string `4x23x21`. It simply
    matches each subpattern in order. It converts the match to a tuple if
    there are two or more subpatterns that produce values.

*   <code><var>parser_var</var></code> - You can use previously defined
    parsers that you've stored in local variables.

    For example, the `amount` parser below makes use of the `fraction` parser
    defined on the previous line.

    ```
    let fraction = parser!(i64 "/" u64);
    let amount = parser!(fraction " tsp");

    assert_eq!(amount.parse("1/4 tsp").unwrap(), (1, 4));
    ```

Repeating patterns:

*   <code><var>pattern</var>*</code> - Any pattern followed by an asterisk
    matches that pattern zero or more times. It converts the results to a
    `Vec`. For example, `parser!("A"*)` matches the strings `A`, `AA`,
    `AAAAAAAAAAAAAA`, and so on, as well as the empty string.

*   <code><var>pattern</var>+</code> - Matches the pattern one or more times, producing a `Vec`.
    `parser!("A"+)` matches `A`, `AA`, etc., but not the empty string.

*   <code><var>pattern</var>?</code> - Optional pattern, producing a Rust `Option`. For
    example, `parser!("x=" i32?)` matches `x=123`, producing `Some(123)`;
    it also matches `x=`, producing the value `None`.

    These behave just like the `*`, `+`, and `?` special characters in
    regular expressions.

*   <code>repeat_sep(<var>pattern</var>, <var>separator</var>)</code> -
    Match the given *pattern* any number of times, separated by the *separator*.
    This converts only the bits that match *pattern* to Rust values, producing
    a `Vec`. Any parts of the string matched by *separator* are not converted.

*   <code>lines(<var>pattern</var>)</code> - Matches any number of lines of
    text matching *pattern*. Each line must be terminated by a newline, `'\n'`.

Custom conversion:

*   <code>... (<var>name1</var>: <var>pattern1</var>) ... => <var>expr</var></code> -
    On successfully matching the patterns to the left of `=>`, evaluate the Rust
    expression *expr* to convert the results to a single Rust value.

    Use this to convert input to structs or enums. For example, suppose we have
    input that looks like `(3,66)-(27,8)` and we want to produce these structs:

    ```
    #[derive(Debug, PartialEq)]
    struct Point(i64, i64);

    #[derive(Debug, PartialEq)]
    struct Line {
        p1: Point,
        p2: Point,
    }
    ```

    The patterns we need are:

    ```
    let point = parser!("(" (x: i64) "," (y: i64) ")" => Point(x, y));
    let line = parser!((p1: point) "-" (p2: point) => Line { p1, p2 });

    assert_eq!(
        line.parse("(3,66)-(27,8)").unwrap(),
        Line { p1: Point(3, 66), p2: Point(27, 8) },
    );
    ```

Patterns with two or more alternatives:

*   <code>{<var>pattern1</var>, <var>pattern2</var>, ...}</code> -
    First try matching *pattern1*; if it matches, stop. If not, try
    *pattern2*, and so on. All the patterns must produce the same type of
    Rust value.

    For example, `parser!({"<" => -1, ">" => 1})` either matches `<`,
    returning the value `-1`, or matches `>`, returing `1`.

[AoC]: https://adventofcode.com/
[example]: https://adventofcode.com/2015/day/2
[aoc-runner]: https://lib.rs/crates/aoc-runner

License: MIT