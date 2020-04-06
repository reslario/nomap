pub mod shared;
pub mod standard;
pub mod valve;

use crate::parse::{
    common::parse,
    formats::shared::{separator, maybe_sep_terminated},
    core::{
        Parse,
        Input,
        ParseResult,
        nom::{
            multi::many1,
            error::ParseError,
            sequence::preceded,
            combinator::{map, opt}
        }
    }
};

pub use {
    valve::Valve,
    standard::Standard
};

/// Trait to define a map format by providing the entity type that it contains.
pub trait Format {
    type Entity;
}

/// Representation of a Quake/Half-Life 1 map as a `Vec` of entities,
/// where the entity type is defined by the format.
#[derive(Debug, Clone, PartialEq)]
pub struct Map<F: Format> {
    pub entities: Vec<F::Entity>,
}

impl <'i, E, F> Parse<'i, E> for Map<F>
where
    E: ParseError<Input<'i>> + Clone,
    F: Format,
    F::Entity: Parse<'i, E>
{
    fn parse(input: Input<'i>) -> ParseResult<Self, E> {
        preceded(
            opt(separator),
            map(
                many1(maybe_sep_terminated(parse)),
                |entities| Map { entities }
            )
        )(input)
    }
}