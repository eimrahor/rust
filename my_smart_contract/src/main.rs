struct Token {
    name: String,
    symbol: String,
    total_supply: u64,
    balances: std::collections::HashMap<String, u64>,
}

impl Token {
    fn new(name: &str, symbol: &str, total_supply: u64) -> Self {
        let mut balances = std::collections::HashMap::new();
        balances.insert(String::from("owner"), total_supply);
        Token {
            name: String::from(name),
            symbol: String::from(symbol),
            total_supply,
            balances,
        }
    }

    fn balance_of(&self, owner: &str) -> u64 {
        *self.balances.get(owner).unwrap_or(&0)
    }

    fn transfer(&mut self, from: &str, to: &str, amount: u64) -> bool {
        let sender_balance = self.balances.entry(String::from(from)).or_insert(0);
        if *sender_balance >= amount {
            *sender_balance -= amount;
            let receiver_balance = self.balances.entry(String::from(to)).or_insert(0);
            *receiver_balance += amount;
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut token = Token::new("MyToken", "MTK", 1000);
    println!("Owner balance: {}", token.balance_of("owner"));
    token.transfer("owner", "alice", 100);
    println!("Alice balance: {}", token.balance_of("alice"));
}
