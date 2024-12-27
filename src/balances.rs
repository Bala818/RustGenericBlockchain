use std::collections::BTreeMap;

pub trait Config {
    type AccountId: Clone + Ord + std::fmt::Debug + std::fmt::Display;
    type Balance: Default
        + Copy
        + std::ops::Add<Output = Self::Balance>
        + std::ops::AddAssign
        + std::ops::Sub<Output = Self::Balance> // Added Sub trait
        + std::ops::SubAssign
        + PartialOrd
        + std::fmt::Debug
        + std::fmt::Display;
}

#[derive(Debug)]
pub struct TestConfig;

impl Config for TestConfig {
    type AccountId = String;
    type Balance = u128;
}

#[derive(Debug)]
pub struct Balances<T: Config> {
    pub balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Balances<T> {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn add_user(&mut self, account: T::AccountId, initial_balance: T::Balance) {
        self.balances.insert(account, initial_balance);
    }

    pub fn get_balance(&self, account: &T::AccountId) -> T::Balance {
        *self.balances.get(account).unwrap_or(&T::Balance::default())
    }

    pub fn transfer(
        &mut self,
        from: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), String> {
        // Temporarily take ownership of the sender's balance
        let from_balance = self.balances.remove(&from).ok_or("Sender not found.")?;
        if from_balance < amount {
            self.balances.insert(from, from_balance); // Reinsert sender's balance if insufficient
            return Err("Insufficient funds.".to_string());
        }

        // Temporarily take ownership of the receiver's balance
        let to_balance = self.balances.remove(&to).ok_or("Receiver not found.")?;

        // Update balances
        self.balances.insert(from, from_balance - amount); // Subtract from sender
        self.balances.insert(to, to_balance + amount); // Add to receiver

        Ok(())
    }
}
