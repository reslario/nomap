use {
    crate::parse::{
        common::{fields, parse},
        formats::{
            Format,
            shared::{self, sep_terminated}
        },
        core::{
            Parse,
            Input,
            ParseResult,
            nom::{
                number::float,
                error::ParseError
            }
        }
    }
};

/// The standard map format created by id Software.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Standard;

impl Format for Standard {
    type Entity = Entity;
}

/// The standard format's Entity type.
pub type Entity = shared::Entity<Brush>;

/// The standard format's Brush type.
pub type Brush = shared::Brush<TextureAlignment>;

/// The standard format's Plane type.
pub type Plane = shared::Plane<TextureAlignment>;

/// The standard format's Texture type.
pub type Texture = shared::Texture<TextureAlignment>;

/// Representation of the standard format's texture alignment.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TextureAlignment {
    pub offset: Vector2,
    pub rotation: f32,
    pub scale: Vector2
}

impl <'i, E> Parse<'i, E> for TextureAlignment
where E: ParseError<Input<'i>> + Clone {
    fn parse(input: Input<'i>) -> ParseResult<Self, E> {
        fields!(TextureAlignment:
            offset = sep_terminated(parse),
            rotation = sep_terminated(float),
            scale = parse
        )(input)
    }
}

/// A simple two-dimensional vector using `f32`s.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

impl <'i, E> Parse<'i, E> for Vector2
where E: ParseError<Input<'i>> + Clone {
    fn parse(input: Input<'i>) -> ParseResult<Self, E> {
        fields!(Vector2:
            x = sep_terminated(float),
            y = float
        )(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse::common::test::expected;

    #[test]
    fn vector2() {
        assert_eq!(
            parse(r"105252 3.14"),
            expected(Vector2 { x: 105252., y: 3.14 })
        )
    }

    #[test]
    fn vector2_weird() {
        assert_eq!(
            parse(r"105252
            // why would you do this

             3.14"),
            expected(Vector2 { x: 105252., y: 3.14 })
        );
    }

    #[test]
    fn texture_alignment() {
        assert_eq!(
            parse(r"1367 9.41 .54 14242 0.141"),
            expected(TextureAlignment {
                offset: Vector2 { x: 1367., y: 9.41 },
                rotation: 0.54,
                scale: Vector2 { x: 14242., y: 0.141 }
            })
        )
    }

    #[test]
    fn texture_alignment_weird() {
        assert_eq!(
            parse(r"1367 // gudavdhgawdgawjfdvgh
                         9.41
            .54
             //no
             14242//e



            0.141"),
            expected(TextureAlignment {
                offset: Vector2 { x: 1367., y: 9.41 },
                rotation: 0.54,
                scale: Vector2 { x: 14242., y: 0.141 }
            })
        )
    }
}

