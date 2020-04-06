fn main() {
    // parse the example map with the standard format
    let map = nomap::parse::<nomap::formats::Standard>(include_str!("example.map")).unwrap();

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
}