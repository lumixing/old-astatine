#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
#[repr(u32)]
pub enum Blocks {
    Air,
    Grass,
    Dirt,
    Stone,
    Border,
}

impl Blocks {
    pub fn from(block: u32) -> Self {
        match block {
            0 => Self::Air,
            1 => Self::Grass,
            2 => Self::Dirt,
            3 => Self::Stone,
            4 => Self::Border,
            _ => Self::Air,
        }
    }
}