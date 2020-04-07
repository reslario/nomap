use {
    crate::{
        Map,
        parse::formats::shared::*
    },
    std::fmt::{Display, Formatter, Result}
};
use crate::parse::formats::Format;

impl Display for Vector3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}


impl <TA: Display> Display for Texture<TA> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}", self.name, self.alignment)
    }
}

impl <TA: Display> Display for Plane<TA> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for point in self.points.iter() {
            write!(f, "( {} ) ", point)?
        }
        write!(f, "{}", self.texture)
    }
}

impl <TA: Display> Display for Brush<TA> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "{{")?;
        for plane in self.planes.iter() {
            writeln!(f, "{}", plane)?
        }
        write!(f, "}}")
    }
}

impl Display for Fields {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for (key, value) in self.iter() {
            writeln!(f, r#""{}" "{}""#, key, value)?
        }
        Ok(())
    }
}

impl <B: Display> Display for Entity<B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "{{")?;
        write!(f, "{}", self.fields)?;
        for brush in self.brushes.iter() {
            writeln!(f, "{}", brush)?
        }
        write!(f, "}}")
    }
}

impl <F> Display for Map<F>
where
    F: Format,
    F::Entity: Display
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for ent in self.entities.iter() {
            writeln!(f, "{}", ent)?
        }
        Ok(())
    }
}
