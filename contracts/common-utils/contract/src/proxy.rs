// contracts/common-utils/src/proxy.rs

use soroban_sdk::{Env, Address, Bytes, symbol_short};
use crate::storage::UpgradeKey;

pub fn get_logic(env: &Env) -> Address {
    env.storage()
        .instance()
        .get(&UpgradeKey::LogicAddress)
        .expect("No logic contract set")
}

pub fn set_logic(env: &Env, logic: Address) {
    env.storage().instance().set(&UpgradeKey::LogicAddress, &logic);
}

// generic forward (Soroban-style dynamic call)
pub fn forward(
    env: &Env,
    func: symbol_short,
    args: Vec<Bytes>,
) -> Bytes {
    let logic = get_logic(env);

    env.invoke_contract(&logic, &func, args)
}