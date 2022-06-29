#![allow(dead_code)]

mod tests;

use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_while, take_while1},
    character::complete::{char, i64, none_of, one_of},
    combinator::{cut, map, opt, recognize, value},
    error::{context, ParseError, ContextError},
    multi::{separated_list0, many0},
    number::complete::double,
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult,
};

use crate::data::{Children, Element, Value};

pub fn document<'a, E: ParseError<&'a str> + ContextError<&'a str>>(i: &'a str) -> IResult<&'a str, Element, E> {
    delimited(whitespace, element, whitespace)(i)
}

fn element<'a, E: ParseError<&'a str> + ContextError<&'a str>>(i: &'a str) -> IResult<&'a str, Element, E> {
    let (rest, name) = identifier(i)?;
    let (rest, attrs) = preceded(whitespace, opt(attributes))(rest)?;
    let (rest, children) = preceded(whitespace, opt(children))(rest)?;
    
    Ok((
        rest,
        Element {
            name: name.to_string(),
            attributes: attrs
                .unwrap_or_default()
                .into_iter()
                .map(|(name, value)| (name.to_string(), value))
                .collect(),
            children: children.unwrap_or(Children::Elements(vec![])),
        },
    ))
}

fn attributes<'a, E: ParseError<&'a str> + ContextError<&'a str>>(i: &'a str) -> IResult<&'a str, Vec<(&'a str, Value)>, E> {
    context(
        "attribute list",
        preceded(
            terminated(char('('), whitespace),
            cut(terminated(
                separated_list0(delimited(whitespace, char(','), whitespace), attribute),
                preceded(whitespace, char(')')),
            )),
        ),
    )(i)
}

fn children<'a, E: ParseError<&'a str> + ContextError<&'a str>>(i: &'a str) -> IResult<&'a str, Children, E> {
    context(
        "children list",
        preceded(
            char('{'),
            cut(terminated(
                alt((
                    map(preceded(whitespace, string), |s| Children::String(s.to_string())),
                    map(many0(preceded(whitespace, element)), Children::Elements),
                )),
                preceded(whitespace, char('}')),
            )),
        ),
    )(i)
}

fn attribute<'a, E: ParseError<&'a str> + ContextError<&'a str>>(i: &'a str) -> IResult<&'a str, (&'a str, Value), E> {
    separated_pair(
        identifier,
        cut(preceded(whitespace, one_of("=:"))),
        preceded(whitespace, mml_value),
    )(i)
}

fn mml_value<'a, E: ParseError<&'a str> + ContextError<&'a str>>(i: &'a str) -> IResult<&'a str, Value, E> {
    alt((
        map(string, |s| Value::String(s.to_string())),
        map(i64, Value::Integer),
        map(double, Value::Float),
        map(
            alt((value(true, tag("true")), value(false, tag("false")))),
            Value::Bool,
        ),
    ))(i)
}

fn identifier<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    recognize(pair(
        take_while1(|c: char| c.is_alphabetic() || c == '_'),
        take_while(|c: char| c.is_alphanumeric() || "_-.".contains(c)),
    ))(i)
}

fn string<'a, E: ParseError<&'a str> + ContextError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    context(
        "string",
        preceded(char('"'), cut(terminated(string_content, char('"')))),
    )(i)
}

fn string_content<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    escaped(none_of("\\\""), '\\', one_of("\"\\nrt"))(i)
}

fn whitespace<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    take_while(|c: char| c.is_whitespace())(i)
}
