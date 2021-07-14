pub struct Player {
    pub id: PlayerId,
    pub name: String,
    balance: u32,
    //probably more to come
}

impl Player {
    pub fn new(id: PlayerId, name: String, balance: u32) -> Player {
        Player {
            id,
            name,
            balance,
        }
    }

    pub fn bet(&mut self, amount: u32) {
        assert!(self.balance > amount);
        self.balance = self.balance - amount;
    }

    pub fn win(&mut self, amount: u32) {
        self.balance = self.balance + amount;
    }

    pub fn balance(&self) -> u32 {
        self.balance
    }
}


pub type PlayerId = usize;