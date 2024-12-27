use std::collections::BTreeMap;

pub trait Config {
    type AccountId: Clone + Ord + std::fmt::Debug + std::fmt::Display;
    type BlockNumber: Default
        + Copy
        + std::ops::Add<Output = Self::BlockNumber>
        + From<u8>
        + std::fmt::Debug;
    type Nonce: Default
        + Copy
        + std::ops::Add<Output = Self::Nonce>
        + std::ops::AddAssign
        + From<u8>
        + std::fmt::Debug;
}

#[derive(Debug)]
pub struct TestConfig;

impl Config for TestConfig {
    type AccountId = String;
    type BlockNumber = u32;
    type Nonce = u32;
}

#[derive(Debug)]
pub struct System<T: Config> {
    pub block_number: T::BlockNumber,
    pub nonces: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> System<T> {
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::default(),
            nonces: BTreeMap::new(),
        }
    }

    pub fn increment_block(&mut self) {
        self.block_number = self.block_number + 1.into(); // Increment block number
    }

    pub fn increment_nonce(&mut self, account: T::AccountId) {
        let nonce = self.nonces.entry(account).or_insert(T::Nonce::default());
        *nonce += 1.into(); // Increment nonce by 1
    }
}
