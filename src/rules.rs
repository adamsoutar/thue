pub enum SpecialType {
    None,
    Output,
    Input
}

pub struct Rule {
    pub left: String,
    pub right: String,
    pub special: SpecialType
}
