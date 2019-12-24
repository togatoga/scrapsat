#[derive(PartialEq, Debug, Clone, Copy)]
pub enum LitBool {
    True = 0,
    False = 1,
    UnDef = 2,
}

impl From<i8> for LitBool {
    fn from(x: i8) -> Self {
        match x {
            0 => LitBool::True,
            1 => LitBool::False,
            _ => LitBool::UnDef,
        }
    }
}

impl Default for LitBool {
    fn default() -> Self {
        LitBool::UnDef
    }
}
