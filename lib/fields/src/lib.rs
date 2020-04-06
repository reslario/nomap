#[macro_export]
macro_rules! fields {
    ($strct:ident: $($field:ident = $parser:expr),+) => {
        nom::combinator::map(
            nom::sequence::tuple((
                $($parser),+
            )),
            |($($field),+)| $strct {
                $($field),+
            }
        )
    };
}
