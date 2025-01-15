#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HandAction {
    Hit,
    Stand,
    DoubleDown,
    Split,
    Surrender,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InsuranceAction {
    Take,
    Refuse,
}
