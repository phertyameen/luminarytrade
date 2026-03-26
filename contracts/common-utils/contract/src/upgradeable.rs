// contracts/common-utils/src/upgradeable.rs

use soroban_sdk::{Env, Address};
use crate::storage::UpgradeKey;

pub fn init(env: &Env, admin: Address, logic: Address) {
    env.storage().instance().set(&UpgradeKey::Admin, &admin);
    env.storage().instance().set(&UpgradeKey::LogicAddress, &logic);
    env.storage().instance().set(&UpgradeKey::Version, &1u32);
}