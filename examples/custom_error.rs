use {
    nom::error::ErrorKind,
    nomap::parse::core::Parse
};

// a perfectly helpful error type
#[derive(Debug, Clone, PartialEq)]
struct CustomError {
    reason: &'static str
}

impl <I> nom::error::ParseError<I> for CustomError {
    fn from_error_kind(_input: I, _kind: ErrorKind) -> Self {
        CustomError {
            reason: "something went wrong"
        }
    }

    fn append(_input: I, _kind: ErrorKind, other: Self) -> Self {
        other
    }
}

fn main() {
    let result: nomap::parse::core::ParseResult<_, CustomError>
        = nomap::Map::<nomap::formats::Standard>::parse("not a map");
    dbg!(result);
}