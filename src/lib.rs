//! # nomap
//! A parser for the `.map` file format used by Quake 1 & 2 as well as Half-Life 1,
//! implemented using the [nom](https://www.crates.io/crates/nom) parsing framework.
//! It can easily be integrated with other `nom` parsers.
//!
//! `nomap` is whitespace agnostic and ignores comments.
//! It also optionally provides `Display` implementations for all its types (through
//! the "display" feature), so you can serialise a parsed map back into a string.
//!
//! ## Example
//! ```
//! // parse the example map with the standard format
//! let map = nomap::parse::<nomap::formats::Standard>(include_str!("../examples/example.map")).unwrap();
//!
//! // report our findings
//! for ent in map.entities.iter() {
//!     println!(
//!         "Found entity of class `{}` with {} brush{}",
//!         // every entity should have this, so we optimistically index here
//!         ent.fields["classname"],
//!         ent.brushes.len(),
//!         // some fanciness
//!         if ent.brushes.len() == 1 { "" } else { "es" }
//!     )
//! }
//! ```

pub mod parse;
#[cfg(feature = "display")]
pub mod display;

use parse::core::{
    Input,
    Error,
    Parse,
    nom::{
        self,
        combinator::all_consuming
    }
};

pub use parse::{
    *,
    formats::Map
};

/// Convenience function to parse a map from a string. Assumes that the input
/// consists entirely of the map and returns the [Error](parse::core::Error)
/// type provided by this crate. If you wish to integrate with other `nom` parsers,
/// using the [Parse](parse::core::Parse) implementation on [Map](parse::formats::Map)
/// is recommended.
pub fn parse<'i, F>(input: Input<'i>) -> Result<Map<F>, nom::Err<Error<'i>>>
where
    F: formats::Format,
    F::Entity: Parse<'i, Error<'i>>
{
    all_consuming(Map::parse)(input)
        .map(|(_rest, map)| map)
}
