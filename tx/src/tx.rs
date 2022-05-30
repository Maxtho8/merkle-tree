
pub struct Tx{
    amount : u64,
    sender : String,
    receiver : String,
}

impl Tx{
    pub fn new(amount : u64, sender : String, receiver : String) -> Tx{
        Tx{
            amount : amount,
            sender : sender,
            receiver : receiver,
        }
    }
    pub fn to_string(&self) -> String{
        format!("{}:{}:{}", self.amount, self.sender, self.receiver)
    }
} 