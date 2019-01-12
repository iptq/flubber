pub enum Unit {
    // fixed width
    Pixels(u32),
    // optional max/min
    Expand(Option<u32>, Option<u32>),
}

pub struct Dimension(Unit, Unit);
