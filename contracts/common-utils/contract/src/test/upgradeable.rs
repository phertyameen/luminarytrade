#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{Env, testutils::Address as _};

    #[test]
    fn test_upgrade_flow() {
        let env = Env::default();

        let admin = Address::generate(&env);
        let logic_v1 = Address::generate(&env);
        let logic_v2 = Address::generate(&env);

        init(&env, admin.clone(), logic_v1.clone());

        upgrade(&env, admin.clone(), logic_v2.clone());

        let current: Address = env
            .storage()
            .instance()
            .get(&UpgradeKey::LogicAddress)
            .unwrap();

        assert_eq!(current, logic_v2);
    }

    #[test]
    fn test_rollback() {
        let env = Env::default();

        let admin = Address::generate(&env);
        let v1 = Address::generate(&env);
        let v2 = Address::generate(&env);

        init(&env, admin.clone(), v1.clone());
        upgrade(&env, admin.clone(), v2.clone());

        rollback(&env, admin.clone());

        let current: Address = env
            .storage()
            .instance()
            .get(&UpgradeKey::LogicAddress)
            .unwrap();

        assert_eq!(current, v1);
    }
}