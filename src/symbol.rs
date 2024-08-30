use std::fmt::Display;

#[derive(Clone, Eq, PartialEq)]
pub struct Symbol {
    pub id: String,
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl From<String> for Symbol {
    fn from(value: String) -> Self {
        Symbol { id: value }
    }
}

impl From<&str> for Symbol {
    fn from(value: &str) -> Self {
        Symbol {
            id: String::from(value),
        }
    }
}
