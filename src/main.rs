mod balances;
mod runtime;
mod system;

use balances::TestConfig as BalancesTestConfig;
use runtime::Runtime;
use system::TestConfig as SystemTestConfig;

fn main() {
    let mut runtime = Runtime::<SystemTestConfig, BalancesTestConfig>::new();

    runtime.balances.add_user("Alice".to_string(), 1000);
    runtime.balances.add_user("Bob".to_string(), 500);

    runtime.simulate_transaction("Alice".to_string(), "Bob".to_string(), 300);
    runtime.simulate_transaction("Bob".to_string(), "Alice".to_string(), 200);
}
