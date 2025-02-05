use crate::Result;


pub trait TradeCommand{
    fn execute(&self)->Result<()>;
}

