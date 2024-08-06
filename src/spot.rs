#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Suit {
    None,
    Cross,
    Circle,
}

impl Suit {
    pub fn is_none(&self) -> bool {
        *self == Self::None
    }

    pub fn oposite(&self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Cross => Self::Circle,
            Self::Circle => Self::Cross,
        }
    }
}
