use {
    crate::parse::formats::valve::*,
    std::fmt::{Display, Formatter, Result}
};

impl Display for Scale {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}", self.u, self.v)
    }
}

impl Display for Axis {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[ {} {} ]", self.normal, self.offset)
    }
}

impl Display for Axes {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}", self.u, self.v)
    }
}

impl Display for TextureAlignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {} {}", self.axes, self.rotation, self.scale)
    }
}
