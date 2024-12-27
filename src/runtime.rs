use crate::balances::{Balances, Config as BalancesConfig};
use crate::system::{Config as SystemConfig, System};

macro_rules! print_runtime_state {
    ($runtime:expr) => {
        println!("Current Runtime State:\n{:?}", $runtime);
    };
}

#[derive(Debug)]
pub struct Runtime<SystemConfigImpl, BalancesConfigImpl>
where
    SystemConfigImpl: SystemConfig<AccountId = BalancesConfigImpl::AccountId>,
    BalancesConfigImpl: BalancesConfig,
{
    pub system: System<SystemConfigImpl>,
    pub balances: Balances<BalancesConfigImpl>,
}

impl<SystemConfigImpl, BalancesConfigImpl> Runtime<SystemConfigImpl, BalancesConfigImpl>
where
    SystemConfigImpl: SystemConfig<AccountId = BalancesConfigImpl::AccountId> + std::fmt::Debug,
    BalancesConfigImpl: BalancesConfig + std::fmt::Debug,
{
    pub fn new() -> Self {
        Self {
            system: System::new(),
            balances: Balances::new(),
        }
    }

    pub fn simulate_transaction(
        &mut self,
        from: SystemConfigImpl::AccountId,
        to: SystemConfigImpl::AccountId,
        amount: BalancesConfigImpl::Balance,
    ) {
        self.system.increment_block();
        self.system.increment_nonce(from.clone());

        match self.balances.transfer(from.clone(), to.clone(), amount) {
            Ok(_) => println!(
                "Transaction successful! {} sent {} to {}.",
                from, amount, to
            ),
            Err(e) => println!("Transaction failed: {}", e),
        }

        print_runtime_state!(self);
    }
}
