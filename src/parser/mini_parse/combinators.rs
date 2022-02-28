#![allow(unused)]
use std::fmt;

use super::{Spanned, Token};

// TODO Added custom errors to ParseResult
pub type ParseResult<'a, Input, Output> =
    Result<(&'a [Spanned<Input>], Output), &'a [Spanned<Input>]>;

pub trait Parser<'a, Input, Output>
where
    Input: Clone + PartialEq,
{
    fn parse(&self, input: &'a [Spanned<Input>]) -> ParseResult<'a, Input, Output>;

    fn map<F, NewOutput>(self, map_fn: F) -> BoxedParser<'a, Input, NewOutput>
    where
        Self: Sized + 'a,
        Input: 'a,
        Output: fmt::Debug + 'a,
        NewOutput: 'a,
        F: Fn(Output) -> NewOutput + 'a,
    {
        BoxedParser::new(map(self, map_fn))
    }

    fn pred<F>(self, pred_fn: F) -> BoxedParser<'a, Input, Output>
    where
        Self: Sized + 'a,
        Input: 'a,
        Output: Clone + PartialEq + 'a,
        F: Fn(&Output) -> bool + 'a,
    {
        BoxedParser::new(pred(self, pred_fn))
    }

    fn and_then<F, NextParser, NewOutput>(self, f: F) -> BoxedParser<'a, Input, NewOutput>
    where
        Self: Sized + 'a,
        Input: 'a,
        Output: 'a,
        NewOutput: 'a,
        NextParser: Parser<'a, Input, NewOutput> + 'a,
        F: Fn(Output) -> NextParser + 'a,
    {
        BoxedParser::new(and_then(self, f))
    }
    fn dbg(self, msg: &'a str, show_output: bool) -> BoxedParser<'a, Input, Output>
    where
        Self: Sized + 'a,
        Input: fmt::Debug + 'a,
        Output: fmt::Debug + 'a,
    {
        BoxedParser::new(dbg_name(self, msg, show_output))
    }
}

impl<'a, F, Input, Output> Parser<'a, Input, Output> for F
where
    Input: Clone + PartialEq + 'a,
    F: for<'b> Fn(&'a [Spanned<Input>]) -> ParseResult<'a, Input, Output>,
{
    fn parse(&self, input: &'a [Spanned<Input>]) -> ParseResult<'a, Input, Output> {
        self(input)
    }
}

pub struct BoxedParser<'a, Input, Output> {
    parser: Box<dyn Parser<'a, Input, Output> + 'a>,
}

impl<'a, Input, Output> BoxedParser<'a, Input, Output> {
    fn new<P>(parser: P) -> Self
    where
        Input: Clone + PartialEq + 'a,
        P: Parser<'a, Input, Output> + 'a,
    {
        BoxedParser {
            parser: Box::new(parser),
        }
    }
}

impl<'a, Input, Output> fmt::Debug for BoxedParser<'a, Input, Output> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("BoxedParser")
    }
}

impl<'a, Input, Output> Parser<'a, Input, Output> for BoxedParser<'a, Input, Output>
where
    Input: Clone + PartialEq + 'a,
{
    fn parse(&self, input: &'a [Spanned<Input>]) -> ParseResult<'a, Input, Output> {
        self.parser.parse(input)
    }
}

// pub fn match_literal<'a>(expected: &'static str) -> impl Parser<'a, ()> {
//     move |input: &'a str| match input.get(0..expected.len()) {
//         Some(next) if next == expected => Ok((&input[expected.len()..], ())),
//         _ => Err((
//             input,
//             ParseError::new(format!(
//                 "Match Literal Failed |{}| != |{}|",
//                 expected, input
//             )),
//         )),
//     }
// }
//
// #[test]
// fn literal_parser() {
//     let parse_joe = match_literal("Hello Joe!");
//     assert_eq!(Ok(("", ())), parse_joe.parse("Hello Joe!"));
//     assert_eq!(
//         Ok((" Hello Robert!", ())),
//         parse_joe.parse("Hello Joe! Hello Robert!")
//     );
//
//     assert_eq!(
//         Err((
//             "Hello Mike!",
//             ParseError::new("Match Literal Failed Hello Joe! != Hello Mike!".into())
//         )),
//         parse_joe.parse("Hello Mike!")
//     );
// }
//
// pub fn identifier(input: &str) -> ParseResult<String> {
//     let mut matched = String::new();
//     let mut chars = input.chars();
//     match chars.next() {
//         Some(next) if next.is_alphabetic() => matched.push(next),
//         _ => {
//             return Err((
//                 input,
//                 ParseError::new(format!(
//                     "Identifier Starts with Illegal character: {}",
//                     input[0..1].to_owned()
//                 )),
//             ))
//         }
//     }
//
//     while let Some(next) = chars.next() {
//         if next.is_alphanumeric() || next == '-' {
//             matched.push(next);
//         } else {
//             break;
//         }
//     }
//
//     let next_index = matched.len();
//     Ok((&input[next_index..], matched))
// }
//
// #[test]
// fn identifier_parser() {
//     assert_eq!(
//         Ok(("", "i-am-an-identifier".to_string())),
//         identifier("i-am-an-identifier")
//     );
//     assert_eq!(
//         Ok((" entirely an identifier", "not".to_string())),
//         identifier("not entirely an identifier")
//     );
//     assert_eq!(
//         Err(("!not at all an identifier", ParseError::new("".into()))),
//         identifier("!not at all an identifier")
//     );
// }

pub fn pair<'a, Input, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, Input, (R1, R2)>
where
    Input: Clone + PartialEq + 'a,
    P1: Parser<'a, Input, R1>,
    P2: Parser<'a, Input, R2>,
{
    move |input| {
        parser1.parse(input).and_then(|(next_input, result1)| {
            parser2
                .parse(next_input)
                .map(|(last_input, result2)| (last_input, (result1, result2)))
        })
    }
}

// #[test]
// fn pair_combinator() {
//     let tag_opener = pair(match_literal("<"), identifier);
//     assert_eq!(
//         Ok(("/>", ((), "my-first-element".to_string()))),
//         tag_opener.parse("<my-first-element/>")
//     );
//
//     assert_eq!(
//         Err(("oops", ParseError::new("".into()))),
//         tag_opener.parse("oops")
//     );
//     assert_eq!(
//         Err(("!oops", ParseError::new("".into()))),
//         tag_opener.parse("<!oops")
//     );
//
//     let less_equal = pair(match_literal("<"), match_literal("="));
//     assert_eq!(Ok(("", ((), ()))), less_equal.parse("<="));
//     assert_eq!(
//         Err(("==", ParseError::new("".into()))),
//         less_equal.parse("==")
//     );
// }

fn map<'a, Input, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, Input, B>
where
    Input: Clone + PartialEq + 'a,
    P: Parser<'a, Input, A>,
    F: Fn(A) -> B,
{
    move |input| {
        parser
            .parse(input)
            .map(|(next_input, result)| (next_input, map_fn(result)))
    }
}

pub fn left<'a, Input, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, Input, R1>
where
    Input: Clone + PartialEq + 'a,
    P1: Parser<'a, Input, R1>,
    P2: Parser<'a, Input, R2>,
{
    map(pair(parser1, parser2), |(left, _right)| left)
}

pub fn right<'a, Input, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, Input, R2>
where
    Input: Clone + PartialEq + 'a,
    P1: Parser<'a, Input, R1>,
    P2: Parser<'a, Input, R2>,
{
    map(pair(parser1, parser2), |(_left, right)| right)
}

// #[test]
// fn right_combinator() {
//     let tag_opener = right(match_literal("<"), identifier);
//     assert_eq!(
//         Ok(("/>", "my-first-element".to_string())),
//         tag_opener.parse("<my-first-element/>")
//     );
//     assert_eq!(
//         Err(("oops", ParseError::new("".into()))),
//         tag_opener.parse("oops")
//     );
//     assert_eq!(
//         Err(("!oops", ParseError::new("".into()))),
//         tag_opener.parse("<!oops")
//     );
// }

pub fn one_or_more<'a, Input, P, A>(parser: P) -> impl Parser<'a, Input, Vec<A>>
where
    Input: Clone + PartialEq + 'a,
    P: Parser<'a, Input, A>,
{
    move |mut input| {
        let mut result = Vec::new();

        if let Ok((next_input, first_item)) = parser.parse(input) {
            input = next_input;
            result.push(first_item);
        } else {
            return Err(input);
        }

        while let Ok((next_input, next_item)) = parser.parse(input) {
            input = next_input;
            result.push(next_item);
        }

        Ok((input, result))
    }
}

// #[test]
// fn one_or_more_combinator() {
//     let parser = one_or_more(match_literal("ha"));
//     assert_eq!(Ok(("", vec![(), (), ()])), parser.parse("hahaha"));
//     assert_eq!(
//         Err(("ahah", ParseError::new("".into()))),
//         parser.parse("ahah")
//     );
//     assert_eq!(Err(("", ParseError::new("".into()))), parser.parse(""));
// }

pub fn zero_or_more<'a, Input, P, A>(parser: P) -> impl Parser<'a, Input, Vec<A>>
where
    Input: Clone + PartialEq + 'a,
    P: Parser<'a, Input, A>,
{
    move |mut input| {
        let mut result = Vec::new();

        while let Ok((next_input, next_item)) = parser.parse(input) {
            input = next_input;
            result.push(next_item);
        }

        Ok((input, result))
    }
}

// #[test]
// fn zero_or_more_combinator() {
//     let parser = zero_or_more(match_literal("ha"));
//     assert_eq!(Ok(("", vec![(), (), ()])), parser.parse("hahaha"));
//     assert_eq!(Ok(("ahah", vec![])), parser.parse("ahah"));
//     assert_eq!(Ok(("", vec![])), parser.parse(""));
// }

// pub fn any_char(input: &str) -> ParseResult<char> {
//     match input.chars().next() {
//         Some(next) => Ok((&input[next.len_utf8()..], next)),
//         _ => Err(input),
//     }
// }

fn pred<'a, Input, P, A, F>(parser: P, predicate: F) -> impl Parser<'a, Input, A>
where
    Input: Clone + PartialEq + 'a,
    P: Parser<'a, Input, A>,
    F: Fn(&A) -> bool,
{
    move |input| {
        if let Ok((next_input, value)) = parser.parse(input) {
            if predicate(&value) {
                return Ok((next_input, value));
            }
        }
        Err(input)
    }
}

// #[test]
// fn predicate_combinator() {
//     let parser = pred(any_char, |c| *c == 'o');
//     assert_eq!(Ok(("mg", 'o')), parser.parse("omg"));
//
//     assert_eq!(
//         Err(("lol", ParseError::new("".into()))),
//         parser.parse("lol")
//     );
// }

// pub fn whitespace_char<'a>() -> impl Parser<'a, char> {
//     pred(any_char, |c| c.is_whitespace())
// }
//
// pub fn space1<'a>() -> impl Parser<'a, Vec<char>> {
//     one_or_more(whitespace_char())
// }
//
// pub fn space0<'a>() -> impl Parser<'a, Vec<char>> {
//     zero_or_more(whitespace_char())
// }
//
// pub fn quoted_string<'a>() -> impl Parser<'a, String> {
//     right(
//         match_literal("\""),
//         left(
//             zero_or_more(any_char.pred(|c| *c != '"')),
//             match_literal("\""),
//         ),
//     )
//     .map(|chars| chars.into_iter().collect())
// }
//
// #[test]
// pub fn quoted_string_parser() {
//     assert_eq!(
//         Ok(("", "Hello Joe!".to_string())),
//         quoted_string().parse("\"Hello Joe!\"")
//     );
// }

pub fn either<'a, Input, P1, P2, A>(parser1: P1, parser2: P2) -> impl Parser<'a, Input, A>
where
    Input: Clone + PartialEq + 'a,
    P1: Parser<'a, Input, A>,
    P2: Parser<'a, Input, A>,
{
    move |input| match parser1.parse(input) {
        ok @ Ok(_) => ok,
        Err(_) => parser2.parse(input),
    }
}

pub fn and_then<'a, Input, P, F, A, B, NextP>(parser: P, f: F) -> impl Parser<'a, Input, B>
where
    Input: Clone + PartialEq + 'a,
    P: Parser<'a, Input, A>,
    NextP: Parser<'a, Input, B>,
    F: Fn(A) -> NextP,
{
    move |input| match parser.parse(input) {
        Ok((next_input, result)) => f(result).parse(next_input),
        Err(err) => Err(err),
    }
}

// pub fn whitespace_wrap<'a, P, A>(parser: P) -> impl Parser<'a, A>
// where
//     P: Parser<'a, A>,
// {
//     right(space0(), left(parser, space0()))
// }
pub fn dbg_name<'a, Input, P, O>(
    parser: P,
    msg: &'a str,
    show_input: bool,
) -> impl Parser<'a, Input, O>
where
    O: fmt::Debug + 'a,
    Input: Clone + PartialEq + fmt::Debug + 'a,
    P: Parser<'a, Input, O>,
{
    move |input| {
        eprintln!("----START--{}----", msg);
        let result = parser.parse(input);
        match &result {
            Ok((next_input, output)) => {
                if show_input {
                    eprintln!("next_input: {:?}", next_input);
                }
                eprintln!("output: {:?}", output);
            }
            Err(input) => {
                eprintln!("input: {:?}", input);
            }
        }
        eprintln!("----End----{}----", msg);
        result
    }
}
