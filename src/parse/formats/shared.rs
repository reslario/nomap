use {
    std::{
        collections::HashMap,
        ops::{Deref, DerefMut}
    },
    crate::parse::{
        common::{fields, parse, quoted_string, many_fixed},
        core::{
            Parse,
            Input,
            ParseResult,
            nom::{
                branch::alt,
                number::float,
                error::ParseError,
                bytes::{tag, take_till},
                multi::{many0, fold_many0},
                combinator::{map, opt, iterator, recognize},
                sequence::{pair, delimited, terminated, preceded},
                character::{char, multispace1, line_ending, not_line_ending}
            }
        }
    }
};

pub(crate) fn separator<'i, E>(input: Input<'i>) -> ParseResult<Input<'i>, E>
where E: ParseError<Input<'i>> + Clone {
    recognize(
        |input| {
            let mut iter = iterator(
                input,
                alt((
                    multispace1,
                    terminated(comment, line_ending),
                ))
            );
            iter.for_each(drop);
            iter.finish()
        }
    )(input)
}

pub(crate) fn sep_terminated<'i, F, O, E>(parsed: F) -> impl Fn(Input<'i>) -> ParseResult<O, E>
where
    F: Fn(Input<'i>) -> ParseResult<O, E>,
    E: ParseError<Input<'i>> + Clone
{
    terminated(parsed, separator)
}

pub(crate) fn maybe_sep_terminated<'i, F, O, E>(parsed: F) -> impl Fn(Input<'i>) -> ParseResult<O, E>
    where
        F: Fn(Input<'i>) -> ParseResult<O, E>,
        E: ParseError<Input<'i>> + Clone
{
    terminated(parsed, opt(separator))
}

/// A wrapper around a `HashMap<String, String>` representing
/// an entity's key/value pairs. In a map file, they usually look
/// something like this:
/// ```plain
/// "classname" "light"
/// "wait" "2"
/// "light" "600"
/// "angle" "315"
/// "mangle" "0 90 0"
/// "origin" "-2704 1908 50"
/// "_color" "1.00 0.93 0.70"
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Fields(pub HashMap<String, String>);

impl Fields {
    pub fn into_inner(self) -> HashMap<String, String> {
        self.0
    }
}

impl Deref for Fields {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Fields {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl <'i, E> Parse<'i, E> for Fields
where E: ParseError<Input<'i>> + Clone {
    fn parse(input: Input<'i>) -> ParseResult<Self, E> {
        map(
            fold_many0(
                maybe_sep_terminated(
                    pair(
                        maybe_sep_terminated(quoted_string),
                        quoted_string
                    )
                ),
                HashMap::new(),
                |mut map, (k, v)| {
                    map.insert(k.into(), v.into());
                    map
                }
            ),
            Fields
        )(input)
    }
}

/// Representation of a map entity with [key/value pairs](Fields) and a list
/// of [Brush](Brush)es, which may be empty if the entity in question is a
/// point entity, like a light.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Entity<B> {
    pub fields: Fields,
    pub brushes: Vec<B>
}

impl <'i, E, B> Parse<'i, E> for Entity<B>
where
    E: ParseError<Input<'i>> + Clone,
    B: Parse<'i, E>
{
    fn parse(input: Input<'i>) -> ParseResult<Self, E> {
        delimited(
            pair(char('{'), opt(separator)),
            fields!(Entity:
                fields = maybe_sep_terminated(parse),
                brushes = many0(maybe_sep_terminated(parse))
            ),
            char('}')
        )(input)
    }
}

/// Representation of a plane with three points describing a
/// half-space and a texture. In a map file, it usually looks
/// something like this with the standard format:
/// ```plain
/// ( 1704 1412 592 ) ( 1688 1424 592 ) ( 1680 1420 592 ) kn_floorp2 53 0 -63 1.15 1
/// ```
/// and like this with the valve format:
/// ```plain
/// ( 816 -796 356 ) ( 816 -804 356 ) ( 808 -804 356 ) stone1_3 [ 0 -1 0 -20 ] [ 1 0 0 16 ] -0 1 1
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Plane<TA> {
    pub points: [Vector3; 3],
    pub texture: Texture<TA>
}

impl <'i, E, TA> Parse<'i, E> for Plane<TA>
where
    E: ParseError<Input<'i>> + Clone,
    TA: Parse<'i, E>
{
    fn parse(input: Input<'i>) -> ParseResult<Self, E> {
        fields!(Plane:
            points = many_fixed(
                maybe_sep_terminated(
                    delimited(
                        pair(char('('), opt(separator)),
                        parse,
                        pair(opt(separator), char(')'))
                    ),
                )
            ),
            texture = parse
        )(input)
    }
}

/// A simple three-dimensional vector using `f32`s.
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl <'i, E> Parse<'i, E> for Vector3
where E: ParseError<Input<'i>> + Clone {
    fn parse(input: Input<'i>) -> ParseResult<Self, E> {
        fields!(Vector3:
            x = sep_terminated(float),
            y = sep_terminated(float),
            z = float
        )(input)
    }
}

/// Representation of a texture, consisting of the
/// texture's name and alignment. The format of the
/// latter differs between map formats.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Texture<TA> {
    pub name: String,
    pub alignment: TA,
}

impl <'i, E, TA> Parse<'i, E> for Texture<TA>
where
    E: ParseError<Input<'i>> + Clone,
    TA: Parse<'i, E>
{
    fn parse(input: Input<'i>) -> ParseResult<Self, E> {
        fields!(Texture:
            name = map(
                sep_terminated(
                    take_till(char::is_whitespace)
                ),
                String::from
            ),
            alignment = parse
        )(input)
    }
}

/// Representation of a map brush, consisting of a
/// list of [Plane](Plane)s.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Brush<TA> {
    pub planes: Vec<Plane<TA>>
}

impl <'i, E, TA> Parse<'i, E> for Brush<TA>
where
    E: ParseError<Input<'i>> + Clone,
    TA: Parse<'i, E>
{
    fn parse(input: Input<'i>) -> ParseResult<Self, E> {
        map(
            delimited(
                pair(char('{'), opt(separator)),
                many0(maybe_sep_terminated(parse)),
                pair(opt(separator), char('}'))
            ),
            |planes| Brush { planes }
        )(input)
    }
}

pub(crate) fn comment<'i, E>(input: Input<'i>) -> ParseResult<Input<'i>, E>
where E: ParseError<Input<'i>> {
    preceded(tag("//"), not_line_ending)(input)
}

#[cfg(test)]
mod test {
    use {
        super::*,
        crate::parse::common::test::expected
    };

    #[test]
    fn fields() {
        let mut map = HashMap::new();
        map.insert("classname".into(), "func_parser".into());
        map.insert("good".into(), "yes".into());

        assert_eq!(
            parse(
                r#""classname" "func_parser"
                "good" "yes""#
            ),
            expected(Fields(map))
        )
    }

    #[test]
    fn fields_weird() {
        let mut map = HashMap::new();
        map.insert("classname".into(), "func_parser".into());
        map.insert("good".into(), "no".into());
        map.insert("msg".into(), r#"evil message with \"quotes\" "#.into());

        assert_eq!(
            parse(
                r#""classname" // don't mind me, just commenting my entity fields
                        "func_parser"
               // no really
                "good" // SOMEONE's gotta find these comments helpful
                                                                                "no"
                                    "msg" // help, I'm horribly misaligned
                "evil message with \"quotes\" " // UH OH WATCH OUT
                "#
            ),
            expected(Fields(map))
        )
    }

    #[test]
    fn vector3() {
        assert_eq!(
            parse(r"105252 3.14 .04"),
            expected(Vector3 { x: 105252., y: 3.14, z: 0.04 })
        )
    }

    #[test]
    fn vector3_weird() {
        assert_eq!(
            parse(r"105252

            //
                    3.14
                    // weeeeeeeeeeeeeeeeeeeeeee
                 .04"),
            expected(Vector3 { x: 105252., y: 3.14, z: 0.04 })
        )
    }

    #[test]
    fn texture() {
        assert_eq!(
            parse(r"wizmet1_2 <texture alignment>"),
            expected(Texture {
                name: "wizmet1_2".into(),
                alignment: DummyTextureAlignment
            })
        )
    }

    #[test]
    fn texture_weird() {
        assert_eq!(
            parse(r"wizmet1_2 //
            // <texture alignment>
            <texture alignment>"),
            expected(Texture {
                name: "wizmet1_2".into(),
                alignment: DummyTextureAlignment
            })
        )
    }

    #[test]
    fn plane() {
        assert_eq!(
            parse(r"( -3104 1736 384 ) ( -3096 1752 384 ) ( -3088 1744 384 ) kn_floorp2 <texture alignment>"),
            expected(Plane {
                points: [
                    Vector3 { x: -3104., y: 1736., z: 384. },
                    Vector3 { x: -3096., y: 1752., z: 384. },
                    Vector3 { x: -3088., y: 1744., z: 384. }
                ],
                texture: Texture {
                    name: "kn_floorp2".into(),
                    alignment: DummyTextureAlignment
                }
            })
        )
    }

    #[test]
    fn plane_weird() {
        assert_eq!(
            parse(r"(-3104 1736 384)(
            -3096 1752 384

            )(

             -3088 //A̎ͤ̓̃̇ͪͬ̇̓͌̓̃͗̀̈̆̿̉́̀̀͏̶͏̯̤̪̼̗A̸̵̢̛̲̘̠̣͕͎̼̜̪͚̲͚̞̮̣̺̫ͣ̊͊̔̈ͤͨͬ̿̇A̴͕̪̬̘̬͔̘͇̭̠͗ͫͮ̔͋̽̔̓͗̅ͩͫ͐̑̂ͤ̌̂̚͢ͅA̸̭̬͚̟̼͉̪ͧ́͐ͩ̍ͤ̀Ȃ͈̣̗͔̰̲͔̻̣̬̻͓̦̠̹̥͆ͮ̊͗́́͘͜͠ͅ
                    1744
             384)kn_floorp2 <texture alignment>"),
            expected(Plane {
                points: [
                    Vector3 { x: -3104., y: 1736., z: 384. },
                    Vector3 { x: -3096., y: 1752., z: 384. },
                    Vector3 { x: -3088., y: 1744., z: 384. }
                ],
                texture: Texture {
                    name: "kn_floorp2".into(),
                    alignment: DummyTextureAlignment
                }
            })
        )
    }

    #[derive(Copy, Clone, PartialEq, Debug)]
    struct DummyTextureAlignment;

    impl <'i, E> Parse<'i, E> for DummyTextureAlignment
    where E: ParseError<Input<'i>> + Clone {
        fn parse(input: Input<'i>) -> ParseResult<Self, E> {
            map(
                tag("<texture alignment>"),
                |_| Self
            )(input)
        }
    }

    #[test]
    fn brush() {
        assert_eq!(
            parse(
                r"{
( -95.75 61 -16 ) ( 31.75 3.75 -16 ) ( -60.5 53.5 10 ) __TB_empty <texture alignment>
( -32.5 -29 -16 ) ( -95.75 61 -16 ) ( -60.5 53.5 10 ) yeeeeeeeeee <texture alignment>
}"
            ),
            expected(Brush {
                planes: vec![
                    Plane {
                        points: [
                            Vector3 { x: -95.75, y: 61., z: -16. },
                            Vector3 { x: 31.75, y: 3.75, z: -16. },
                            Vector3 { x: -60.5, y: 53.5, z: 10. }
                        ],
                        texture: Texture {
                            name: "__TB_empty".into(),
                            alignment: DummyTextureAlignment
                        }
                    },
                    Plane {
                        points: [
                            Vector3 { x: -32.5, y: -29., z: -16. },
                            Vector3 { x: -95.75, y: 61., z: -16. },
                            Vector3 { x: -60.5, y: 53.5, z: 10. }
                        ],
                        texture: Texture {
                            name: "yeeeeeeeeee".into(),
                            alignment: DummyTextureAlignment
                        }
                    }
                ]
            })
        )
    }

    #[test]
    fn brush_weird() {
        assert_eq!(
            parse(
                r"{(-95.75 61 -16 ) ( 31.75 3.75 -16 )
                 ( -60.5 53.5 10 ) __TB_empty <texture alignment>
( -32.5 -29 -16 )
 ( -95.75 61 -16 )                   ( -60.5 53.5 10 )
  yeeeeeeeeee <texture alignment>
  // 🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀
}"
            ),
            expected(Brush {
                planes: vec![
                    Plane {
                        points: [
                            Vector3 { x: -95.75, y: 61., z: -16. },
                            Vector3 { x: 31.75, y: 3.75, z: -16. },
                            Vector3 { x: -60.5, y: 53.5, z: 10. }
                        ],
                        texture: Texture {
                            name: "__TB_empty".into(),
                            alignment: DummyTextureAlignment
                        }
                    },
                    Plane {
                        points: [
                            Vector3 { x: -32.5, y: -29., z: -16. },
                            Vector3 { x: -95.75, y: 61., z: -16. },
                            Vector3 { x: -60.5, y: 53.5, z: 10. }
                        ],
                        texture: Texture {
                            name: "yeeeeeeeeee".into(),
                            alignment: DummyTextureAlignment
                        }
                    }
                ]
            })
        )
    }

    #[derive(Copy, Clone, PartialEq, Debug)]
    struct DummyBrush;

    impl <'i, E> Parse<'i, E> for DummyBrush
        where E: ParseError<Input<'i>> + Clone {
        fn parse(input: Input<'i>) -> ParseResult<Self, E> {
            map(
                char('B'),
                |_| DummyBrush
            )(input)
        }
    }

    #[test]
    fn entity() {
        let mut map = HashMap::new();
        map.insert("classname".into(), "func_parser".into());
        map.insert("good".into(), "yes".into());

        assert_eq!(
            parse(r#"{
            "classname" "func_parser"
            "good" "yes"
            BBB
            }"#),
            expected(Entity {
                fields: Fields(map),
                brushes: vec![DummyBrush; 3]
            })
        )
    }
}
