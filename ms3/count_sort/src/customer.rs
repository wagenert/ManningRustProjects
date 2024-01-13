use std::fmt;

pub struct Customer {
    pub id: String,
    pub num_purchases: i32,
}

impl fmt::Display for Customer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.id, self.num_purchases)
    }
}