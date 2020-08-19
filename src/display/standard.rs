use {
    crate::parse::formats::standard::*,
    std::fmt::{Display, Formatter, Result}
};

impl Display for Vector2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

impl Display for TextureAlignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {} {}", self.offset, self.rotation, self.scale)
    }
}
