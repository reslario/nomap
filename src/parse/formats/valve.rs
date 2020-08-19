use {
    crate::parse::{
        common::{fields, parse},
        formats::{
            Format,
            standard::Vector2,
            shared::{self, Vector3, separator, sep_terminated, maybe_sep_terminated}
        },
        core::{
            Parse,
            Input,
            ParseResult,
            nom::{
                number::float,
                character::char,
                error::ParseError,
                combinator::{map, opt},
                sequence::{pair, delimited}
            },
        },
    }
};

/// Valve Software's map format used in Half-Life 1.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Valve;

impl Format for Valve {
    type Entity = Entity;
}

/// The Valve format's Entity type.
pub type Entity = shared::Entity<Brush>;

/// The Valve format's Brush type.
pub type Brush = shared::Brush<TextureAlignment>;

/// The Valve format's Plane type.
pub type Plane = shared::Plane<TextureAlignment>;

/// The Valve format's Texture type.
pub type Texture = shared::Texture<TextureAlignment>;

/// Representation of the Valve format's texture alignment.
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct TextureAlignment {
    pub axes: Axes,
    pub rotation: f32,
    pub scale: Scale
}

impl <'i, E> Parse<'i, E> for TextureAlignment
where E: ParseError<Input<'i>> + Clone {
    fn parse(input: Input<'i>) -> ParseResult<Self, E> {
        fields!(TextureAlignment:
            axes = maybe_sep_terminated(parse),
            rotation = sep_terminated(float),
            scale = parse
        )(input)
    }
}

/// The u and v axes of the Valve format's texture alignment.
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Axes {
    pub u: Axis,
    pub v: Axis
}

impl <'i, E> Parse<'i, E> for Axes
where E: ParseError<Input<'i>> + Clone {
    fn parse(input: Input<'i>) -> ParseResult<Self, E> {
        fields!(Axes:
            u = maybe_sep_terminated(parse),
            v = parse
        )(input)
    }
}

/// A [texture alignment](TextureAlignment) axis in Valve's map format.
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Axis {
    pub normal: Vector3,
    pub offset: f32
}

impl <'i, E> Parse<'i, E> for Axis
where E: ParseError<Input<'i>> + Clone {
    fn parse(input: Input<'i>) -> ParseResult<Self, E> {
        delimited(
            pair(char('['), opt(separator)),
            fields!(Axis:
                normal = sep_terminated(parse),
                offset = float
            ),
            pair(opt(separator), char(']'))
        )(input)
    }
}

/// The scale of a Valve format [Texture](super::shared::Texture).
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Scale {
    pub u: f32,
    pub v: f32
}

impl <'i, E> Parse<'i, E> for Scale
where E: ParseError<Input<'i>> + Clone {
    fn parse(input: Input<'i>) -> ParseResult<Self, E> {
        map(
            Vector2::parse,
            |vec| Scale {
                u: vec.x,
                v: vec.y
            }
        )(input)
    }
}

#[cfg(test)]
mod test {
    use {
        super::*,
        crate::parse::common::test::expected
    };

    #[test]
    fn axis() {
        assert_eq!(
            parse(r"[ -41 541 -876.2 8 ]"),
            expected(Axis {
                normal: Vector3 { x: -41., y: 541., z: -876.2 },
                offset: 8.
            })
        )
    }

    #[test]
    fn axis_weird() {
        assert_eq!(
            parse(r"[-41
             541 // j


            -876.2
            8]"),
            expected(Axis {
                normal: Vector3 { x: -41., y: 541., z: -876.2 },
                offset: 8.
            })
        )
    }

    #[test]
    fn axes() {
        assert_eq!(
            parse(r"[ -41 541 -876.2 8 ] [ 82 -1082 1752.4 -16 ]"),
            expected(Axes {
                u: Axis {
                    normal: Vector3 { x: -41., y: 541., z: -876.2 },
                    offset: 8.
                },
                v: Axis {
                    normal: Vector3 { x: 82., y: -1082., z: 1752.4 },
                    offset: -16.
                }
            })
        )
    }

    #[test]
    fn axes_weird() {
        assert_eq!(
            parse(r"[
             -20.5 270.5 -438.1 4
             ][
              82 -1082 1752.4 -16
              ]"),
            expected(Axes {
                u: Axis {
                    normal: Vector3 { x: -20.5, y: 270.5, z: -438.1 },
                    offset: 4.
                },
                v: Axis {
                    normal: Vector3 { x: 82., y: -1082., z: 1752.4 },
                    offset: -16.
                }
            })
        )
    }

    #[test]
    fn texture_alignment() {
        assert_eq!(
            parse(r"[ 0 -1 33 -31.4111 ] [ 0.242536 0 -0.970143 -31.4109 ] .1 1 1.03078"),
            expected(TextureAlignment {
                axes: Axes {
                    u: Axis {
                        normal: Vector3 { x: 0., y: -1., z: 33. },
                        offset: -31.4111
                    }, v: Axis {
                        normal: Vector3 { x: 0.242536, y: 0., z: -0.970143 },
                        offset: -31.4109
                    }
                },
                rotation: 0.1,
                scale: Scale { u: 1., v: 1.03078 }
            })
        )
    }

    #[test]
    fn texture_alignment_weird() {
        assert_eq!(
            parse(r"[ 0 -1 33 -31.4111 ] [ 0.242536 0 -0.970143 -31.4109 ].1
            // HUH
             1 1.03078"),
            expected(TextureAlignment {
                axes: Axes {
                    u: Axis {
                        normal: Vector3 { x: 0., y: -1., z: 33. },
                        offset: -31.4111
                    }, v: Axis {
                        normal: Vector3 { x: 0.242536, y: 0., z: -0.970143 },
                        offset: -31.4109
                    }
                },
                rotation: 0.1,
                scale: Scale { u: 1., v: 1.03078 }
            })
        )
    }

    #[cfg(feature = "display")]
    #[test]
    fn roundtrip() {
        let map = crate::Map::<Valve> {
            entities: vec![Entity {
                fields: crate::formats::shared::Fields(std::iter::once(("k".into(), "v".into())).collect()),
                brushes: vec![Brush {
                    planes: vec![Plane {
                        texture: Texture {
                            name: "texture".into(),
                            ..<_>::default()
                        },
                        ..<_>::default()
                    }]
                }]
            }]
        };
        let string = map.to_string();
        assert_eq!(
            expected(map),
            parse(&string)
        )
    }
}
