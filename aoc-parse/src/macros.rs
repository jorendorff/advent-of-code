//use crate::matches::Match;
//use crate::parser::{self, Parser};
//use crate::ParseError;

pub use crate::parser::{alt, empty, exact, lines, opt, parenthesize, plus, sequence, star};

// Output type of a parser
// `e => rustexpr` => type of the rustexpr
// `ident: e` => type of e
// `e as ty` => ty
// `e1 e2` => match (type of e1, type of e2) {
//                ((t1, ...), (t2, ...)) => (t1, ..., t2, ...)
//                ((t1, ...), t2) => (t1, ..., t2)
//                (t1, (t2, ...)) => (t1, t2, ...)
//            }
// `e*` => Vec<type of e>
// `e+` => Vec<type of e>
// `e?` => Option<type of e>
// `(e)` => type of e
// `{ e, ...}` => type of e
// `f(e, ...)` => return type of f (look it up)
// `x` => type of x (look it up)
//
// Other syntax-directed static functions might include:
// - environment generated by the expression before `=>`
// - struct fields (actually the same thing)
// - string label of (used by conversion from Variant to enums)

/// ```text
/// ident ::= a Rust identifier
/// expr ::= a Rust expression
/// literal ::= a Rust literal
///
/// parser ::= rule* expr
///
/// rule ::= "rule" ident "=" expr ";"
///
/// expr ::= label
///   | label "=>" rust_expr        => Map($1, |...pattern of $1...| $2)
///
/// label ::= cast
///   | ident ":" cast              => Label($1, $2)
///
/// cast ::= seq
///   | seq "as" ty                 => TryMap($1, |x| $2::try_from(x))
///
/// seq ::= term
///   | seq term                    => Sequence($1, $2)
///
/// term ::= prim
///   | term "*"                    => Repeat($1, 0)
///   | term "+"                    => Repeat($1, 1)
///   | term "?"                    => Optional($1)
///
/// prim ::= "(" expr ")"
///   | ident "(" expr,* ")"        => Call($1, $2)
///   | ident                       => NamedRule($1)  -- when not followed by "("
///   | literal                     => Literal($1)
///   | "{" expr,* "}"              => OneOf($1)
/// ```
#[macro_export]
macro_rules! parser {
    // parser!(@seq label [expr] [stack] [patterns])
    //
    // Submacro to transform a pattern matching `expr` to a Rust Parser
    // expression; except if `expr` was labeled, the label must already have
    // been stripped off and passed in as `label`.
    //
    // Gradually parses the tokens in `expr`, producing `stack` (in reverse)
    // and `patterns` (in reverse for no good reason), then at the end converts
    // those output-stacks into a Rust parser-expression using `@reverse` and
    // `@reverse_pats`.
    //
    // `stack` is a list of Rust expressions, parsers for the elements of the
    // `expr`. `patterns` is a list of patterns that match the output of the
    // overall SequenceParser we will build from the bits in `stack`.
    //
    // BUG: Because of the simplistic way this macro-parses the input, it
    // doesn't reject some bad syntax like `foo?(x)` or `foo??` or `foo++`.

    // Mapper at the end of a pattern that is not labeled, `expr ::= label => rust_expr`.
    (@seq _ [ => $mapper:expr ] [ $($stack:tt)* ] [ $($pats:tt ,)* ]) => {
        $crate::parser!(@seq _ [] [ $($stack)* ] [ $($pats ,)* ])
            .map(| ( $crate::parser!(@reverse_pats [ $($pats ,)* ] []) ) | $mapper)
    };

    // Mapper at the end of a pattern that is labeled.
    //
    // Experimentally: In `(foo: t1 ", " t2 => f1(foo)) => f2(foo)` the pair is
    // available as `foo` in the inner mapper. Then the result of the inner
    // mapper is named `foo` in the outer mapper.
    (@seq $label:ident [ => $mapper:expr ] [ $($stack:tt)* ] [ $($pats:tt ,)* ]) => {
        $crate::parser!(@seq $label [] [ $($stack)* ] [ $($pats ,)* ])
            .map(| $label @ ( $crate::parser!(@reverse_pats [ $($pats ,)* ] []) ) | $mapper)
    };

    // Recognize `as` keyword, not yet supported.
    //
    // By design of the syntax, `as` only happens at the end of input here.
    (@seq $label:tt [ as $ty:ty ] [ $top:expr , $($stack:expr ,)* ] [ $($pats:tt ,)* ]) => {
        todo!("cast syntax")
    };

    // Reject unsupported non-greedy regex syntax.
    (@seq $label:tt [ * ? $($tail:tt)* ] [ $($stack:expr ,)* ] [ $($pats:tt ,)* ]) => {
        core::compile_error!("non-greedy quantifier `*?` is not supported")
    };

    // Reject unsupported non-greedy regex syntax.
    (@seq $label:tt [ + ? $($tail:tt)* ] [ $($stack:expr ,)* ] [ $($pats:tt ,)* ]) => {
        core::compile_error!("non-greedy quantifier `+?` is not supported")
    };

    // Detect Kleene * and apply it to the preceding term.
    (@seq $label:tt [ * $($tail:tt)* ] [ $top:expr , $($stack:expr ,)* ] [ $top_pat:tt , $($pats:tt ,)* ]) => {
        $crate::parser!(@seq $label [ $($tail)* ] [ $crate::macros::star($top) , $($stack ,)* ] [ _ , $($pats ,)* ])
    };

    // Detect Kleene + and apply it to the preceding term.
    (@seq $label:tt [ + $($tail:tt)* ] [ $top:expr , $($stack:expr ,)* ] [ $top_pat:tt , $($pats:tt ,)* ]) => {
        $crate::parser!(@seq $label [ $($tail)* ] [ $crate::macros::plus($top) , $($stack ,)* ] [ _ , $($pats ,)* ])
    };

    // Detect optional `?` and apply it to the preceding term.
    (@seq $label:tt [ ? $($tail:tt)* ] [ $top:expr , $($stack:tt)* ] [ $top_pat:tt , $($pats:tt ,)* ]) => {
        $crate::parser!(@seq $label [ $($tail)* ] [ $crate::macros::opt($top) , $($stack)* ] [ _ , $($pats ,)* ])
    };

    // A quantifier at the beginning of input (nothing on the stack) is an errror.
    (@seq $label:tt [ * $($tail:tt)* ] [ ] [ $($pats:tt ,)* ]) => {
        core::compile_error!("quantifier `*` has to come after something, not at the start of an expression.")
    };
    (@seq $label:tt [ + $($tail:tt)* ] [ ] [ $($pats:tt ,)* ]) => {
        core::compile_error!("quantifier `+` has to come after something, not at the start of an expression.")
    };
    (@seq $label:tt [ ? $($tail:tt)* ] [ ] [ $($pats:tt ,)* ]) => {
        core::compile_error!("quantifier `?` has to come after something, not at the start of an expression.")
    };

    // call syntax
    (@seq $label:tt [ $f:ident ( $($args:tt)* ) $($tail:tt)* ] [ $($stack:expr ,)* ] [ $($pats:tt ,)* ]) => {
        $crate::parser!(
            @seq $label
            [ $($tail)* ]
            [
                $crate::functions::ParserFunction::call_parser_function(
                    &$f,
                    ( $crate::parser!(@args [$($args)*] [] ()) ),
                )
                ,
                $($stack ,)*
            ]
            [ _ , $($pats ,)* ]
        )
    };

    // parenthesized subpattern with a label
    (@seq $label:tt [ ( $sublabel:ident : $($expr:tt)* ) $($tail:tt)* ] [ $($stack:expr ,)* ] [ $($pats:tt ,)* ]) => {
        $crate::parser!(
            @seq $label
            [ $($tail)* ]
            [
                $crate::parser!(@prim ( $($expr)* )) ,
                $($stack ,)*
            ]
            [ $sublabel , $($pats ,)* ]
        )
    };

    // string literal
    (@seq $label:tt [ $x:literal $($tail:tt)* ] [ $($stack:expr ,)* ] [ $($pats:tt ,)* ]) => {
        $crate::parser!(
            @seq $label
            [ $($tail)* ]
            [
                $crate::parser!(@prim $x) ,
                $($stack ,)*
            ]
            [ #, /* no pattern */ $($pats ,)* ]
        )
    };

    // the first `tt` of any other `term`
    (@seq $label:tt [ $x:tt $($tail:tt)* ] [ $($stack:expr ,)* ] [ $($pats:tt ,)* ]) => {
        $crate::parser!(
            @seq $label
            [ $($tail)* ]
            [
                $crate::parser!(@prim $x) ,
                $($stack ,)*
            ]
            [ _ , $($pats ,)* ]
        )
    };

    // end of input
    (@seq $label:tt [ ] [ $($parts:expr ,)* ] [ $($pats:tt ,)* ]) => {
        $crate::parser!(@reverse [ $($parts ,)* ] [ $crate::macros::empty() ])
    };

    // anything not matched by this point is an error
    (@seq $label:tt [ $($tail:tt)* ] [ $($parts:expr ,)* ] [ $($pats:tt ,)* ]) => {
        core::compile_error!(stringify!(unrecognized syntax @ $($tail)*))
    };

    // parser!(@reverse [input expr stack] [output stack])
    //
    // Take the stack of parsers and produce a single sequence-parser.
    (@reverse [ ] [ $out:expr ]) => {
        $out
    };
    (@reverse [ $head:expr , $($tail:expr ,)* ] [ $out:expr ]) => {
        $crate::parser!(@reverse [ $($tail ,)* ] [ $crate::macros::sequence($head, $out) ])
    };

    // parser!(@reverse_pats [pattern stack] [output stack])
    //
    // Take the stack of Rust patterns, possibly interspersed with `#`
    // to indicate "no pattern", and produce a single pattern.
    (@reverse_pats [ ] [ $out:pat , ]) => {
        $out  // don't produce a singleton-tuple-pattern
    };
    (@reverse_pats [ ] [ $($out:pat ,)* ]) => {
        ( $( $out , )* )
    };
    (@reverse_pats [ #, $($tail:tt ,)* ] [ $( $out:pat , )* ]) => {
        $crate::parser!(@reverse_pats [ $( $tail , )* ] [ $( $out , ) * ])
    };
    (@reverse_pats [ $head:pat , $($tail:tt ,)* ] [ $($out:pat ,)* ]) => {
        $crate::parser!(@reverse_pats [ $( $tail , )* ] [ $head, $($out ,)* ])
    };

    // parser!(@prim pattern)
    //
    // Transform a `prim` into a Rust Parser expression.
    (@prim $x:ident) => {
        $crate::macros::check_parser($x) // TODO: does check_parser help here? does it hurt?
    };
    (@prim $x:literal) => {
        $crate::macros::exact($x)
    };
    (@prim ( $($nested:tt)* )) => {
        $crate::macros::parenthesize(
            $crate::parser!(@seq _ [ $( $nested )* ] [ ] [ ])
        )
    };
    (@prim { $($nested:tt)* }) => {
        $crate::parser!(@list [ $( $nested )* ] [ ] [ ])
    };

    // parser!(@args [unexamined input tokens] [current argument holding area] [transformed output argument exprs])
    //
    // Transform argument lists.

    // end of an argument in an argument list
    (@args [ , $($tail:tt)* ] [ $($seq:tt)* ] ( $( $arg:expr , )* )) => {
        $crate::parser!(
            @args
            [ $( $tail )* ]
            [ ]
            (
                $( $arg:expr , )*
                $crate::parser!(@seq _ [ $( $seq )* ] [ ] [ ]) ,
            )
        )
    };

    // not the end of an arg; just move a token from the input to the holding area
    (@args [ $next:tt $($tail:tt)* ] [ $($seq:tt)* ] ( $( $out:expr , )* )) => {
        $crate::parser!(
            @args
            [ $( $tail )* ]
            [ $( $seq )* $next ]
            ( $( $out , )* )
        )
    };

    // end of argument list, after trailing comma or empty
    (@args [] [ ] $out:expr) => {
        $out
    };

    // end of argument list with no trailing comma: infer one
    (@args [] [ $($seq:tt)+ ] ( $( $out:expr , )* )) => {
        $crate::parser!(@args [,] [ $($seq)+ ] ( $( $out , )* ))
    };

    // parser!(@list [unexamined input tokens] [current arm holding area] [transformed output arm parser expressions])
    //
    // The list of patterns in the body of an alternation.

    // end of first arm of an alternation
    (@list [ , $($tail:tt)* ] [ $($seq:tt)* ] [ ]) => {
        $crate::parser!(
            @list
            [ $( $tail )* ]
            [ ]
            [ $crate::parser!(@seq _ [ $( $seq )* ] [ ] [ ]) ]
        )
    };

    // end of a non-first arm of an alternation
    (@list [ , $($tail:tt)* ] [ $($seq:tt)* ] [ $out:expr ]) => {
        $crate::parser!(
            @list
            [ $( $tail )* ]
            [ ]
            [ $crate::macros::alt($out, $crate::parser!(@seq _ [ $( $seq )* ] [ ] [ ])) ]
        )
    };

    // not the end of an arm; just move a token from the input to the holding area
    (@list [ $next:tt $($tail:tt)* ] [ $($seq:tt)* ] [ $($out:expr)? ]) => {
        $crate::parser!(
            @list
            [ $( $tail )* ]
            [ $( $seq )* $next ]
            [ $( $out )? ]
        )
    };

    // completely empty alternation; could technically be understood as never matching,
    // but it's not a useful thing to express, so reject.
    (@list [ ] [ ] [ ]) => {
        ::core::compile_error("no arms in alternation")
    };

    // end of alternation after comma
    (@list [ ] [ ] [ $out:expr ]) => {
        $out
    };

    // end of alternation with no comma: infer one
    (@list [ ] [ $($seq:tt)+ ] [ $( $out:expr )? ]) => {
        $crate::parser!(@list [,] [ $($seq)+ ] [ $( $out )? ])
    };

    // parser!(@...) - This is an internal error, shouldn't happen in the wild.
    (@ $($tail:tt)*) => {
        ::core::compile_error!(stringify!(unrecognized syntax @ $($tail)*))
    };

    // parser!(rule ...) - Rule-expression, `parser ::= rule* expr`
    (rule $name:ident = $($body:tt)* ; $($tail:tt)*) => {
        {
            let $name = check_parser($crate::parser!($($body)*));
            $crate::parser!($($tail)*)
        }
    };

    // parser!(ident : ...) - Labeled expression, `label ::= ident : cast`
    ($label:ident : $($tail:tt)*) => {
        $crate::parser!(@seq $label [ $($tail)* ] [ ] [ ])
    };

    // Hand anything else off to the @seq submacro.
    ($($tail:tt)*) => {
        $crate::parser!(@seq _ [ $($tail)* ] [ ] [ ])
    };
}

/// My hope here is that this will help with the error message when you
/// accidentally do something like `parser!(blah)` where `blah` isn't a parser
/// at all.
pub fn check_parser<'a, 'b, P>(parser: P) -> P
where
    P: crate::Parser<'a, 'b>,
{
    parser
}
