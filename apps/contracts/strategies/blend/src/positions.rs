use soroban_sdk::contracttype;

#[contracttype]
pub struct VaultPosition {
    /// Total amount deposited by the user
    pub deposited: i128,
    /// Total amount withdrawn by the user
    pub withdrawn: i128,
    /// Total amount of bTokens owned by the user
    pub b_tokens: i128,
}

impl VaultPosition {
    pub fn add(&mut self, amount: i128, b_tokens: i128) {
        self.deposited += amount;
        self.b_tokens += b_tokens;
    }

    pub fn remove(&mut self, amount: i128, b_tokens: i128) {
        self.withdrawn += amount;
        self.b_tokens -= b_tokens;
    }
}