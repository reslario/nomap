[![](http://meritbadge.herokuapp.com/nomap)](https://crates.io/crates/nomap)
[![Docs](https://docs.rs/nomap/badge.svg)](https://docs.rs/nomap/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)]()
# nomap
A parser for the `.map` file format used by Quake 1 & 2 as well as Half-Life 1,
implemented using the [nom](https://www.crates.io/crates/nom) parsing framework. It can
easily be integrated with other `nom` parsers.

`nomap` is whitespace agnostic and ignores comments.
It also optionally provides `Display` implementations for all its types (through
the "display" feature), so you can serialise a parsed map back into a string.

## Example
```rust
// parse the example map with the standard format
let map = nomap::parse::<nomap::formats::Standard>(include_str!("../examples/example.map")).unwrap();

// report our findings
for ent in map.entities.iter() {
    println!(
        "Found entity of class `{}` with {} brush{}",
        // every entity should have this, so we optimistically index here
        ent.fields["classname"],
        ent.brushes.len(),
        // some fanciness
        if ent.brushes.len() == 1 { "" } else { "es" }
    )
}
```