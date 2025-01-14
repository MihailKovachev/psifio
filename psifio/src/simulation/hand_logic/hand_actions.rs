#[derive(Debug, PartialEq)]
pub enum HandAction {
    Hit,
    Stand,
    DoubleDown,
    Split,
    Surrender,
}

#[derive(Debug)]
pub enum InsuranceAction {
    Take,
    Refuse
}