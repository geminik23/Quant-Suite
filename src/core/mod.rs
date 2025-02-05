use nanoid::nanoid;

#[derive(Debug, Clone, PartialEq)]
pub enum AlertSet {
    High(f64),
    Low(f64),
}

// Make abstraction for Ids. for TradeId and OrderId and AlertId
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id(String);

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


