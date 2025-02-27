use nanoid::nanoid;

#[derive(Debug, Clone, PartialEq)]
pub enum AlertSet {
    High(f64),
    Low(f64),
}

// Make abstraction for Ids. for TradeId and OrderId and AlertId
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id(pub String);

impl Id {
    pub fn new() -> Self {
        Id(nanoid!())
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Id{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

