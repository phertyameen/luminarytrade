#![cfg(test)]
use soroban_sdk::{testutils::Address as _, Address, Env, String, Vec};
use credentials::{CredentialsContract, CredentialsContractClient};
use credentials::badges::BadgeType;

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, CredentialsContract);
    let client = CredentialsContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);
}

#[test]
fn test_mint_and_query() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, CredentialsContract);
    let client = CredentialsContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let minter = Address::generate(&env);
    let user = Address::generate(&env);

    client.initialize(&admin);
    client.add_minter(&minter);

    let verification_link = String::from_str(&env, "https://verify.me/123");
    let token_id = client.mint(&minter, &user, &BadgeType::CreditScore, &0, &verification_link);

    assert_eq!(token_id, 1);
    assert!(client.has_credential(&user, &BadgeType::CreditScore));
    assert!(!client.has_credential(&user, &BadgeType::FraudFree));

    let credentials = client.get_credentials(&user);
    assert_eq!(credentials.len(), 1);
    let cred = credentials.get(0).unwrap();
    assert_eq!(cred.badge_type, BadgeType::CreditScore);
    assert_eq!(cred.verification_link, verification_link);
}

#[test]
#[should_panic(expected = "Non-transferable badge")]
fn test_soulbound_transfer_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, CredentialsContract);
    let client = CredentialsContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let recipient = Address::generate(&env);

    client.initialize(&admin);

    let link = String::from_str(&env, "link");
    let token_id = client.mint(&admin, &user, &BadgeType::CreditScore, &0, &link);

    client.transfer(&user, &recipient, &token_id);
}

#[test]
fn test_transferable_badge() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, CredentialsContract);
    let client = CredentialsContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let recipient = Address::generate(&env);

    client.initialize(&admin);

    let link = String::from_str(&env, "link");
    let token_id = client.mint(&admin, &user, &BadgeType::Participation, &0, &link);

    client.transfer(&user, &recipient, &token_id);
    
    assert!(client.has_credential(&recipient, &BadgeType::Participation));
    assert!(!client.has_credential(&user, &BadgeType::Participation));
}

#[test]
fn test_revocation() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, CredentialsContract);
    let client = CredentialsContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    client.initialize(&admin);

    let link = String::from_str(&env, "link");
    let token_id = client.mint(&admin, &user, &BadgeType::CreditScore, &0, &link);

    assert!(client.is_valid(&token_id));

    let note = String::from_str(&env, "Revoked for testing");
    client.revoke(&admin, &token_id, &note);

    assert!(!client.is_valid(&token_id));
    assert!(!client.has_credential(&user, &BadgeType::CreditScore));
}

#[test]
fn test_expiry() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, CredentialsContract);
    let client = CredentialsContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    client.initialize(&admin);

    let expiry = env.ledger().timestamp() + 1000;
    let link = String::from_str(&env, "link");
    let token_id = client.mint(&admin, &user, &BadgeType::Staking, &expiry, &link);

    assert!(client.is_valid(&token_id));

    // Fast forward ledger time
    env.ledger().set_timestamp(expiry + 1);

    assert!(!client.is_valid(&token_id));
    assert!(!client.has_credential(&user, &BadgeType::Staking));
}
