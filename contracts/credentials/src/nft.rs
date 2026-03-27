use soroban_sdk::{contracttype, Address, Env, Vec};
use crate::badges::{BadgeType, CredentialMetadata};

#[contracttype]
pub enum DataKey {
    Admin,
    TokenId,
    Credential(u64),
    OwnerCredentials(Address),
    Minters(Address),
}

pub fn is_soulbound(badge_type: &BadgeType) -> bool {
    match badge_type {
        BadgeType::CreditScore | BadgeType::FraudFree => true,
        BadgeType::Staking | BadgeType::Participation | BadgeType::Volume => false,
    }
}

pub fn mint(env: &Env, to: Address, metadata: CredentialMetadata) -> u64 {
    // Get and increment token ID
    let mut token_id: u64 = env.storage().instance().get(&DataKey::TokenId).unwrap_or(0);
    token_id += 1;
    env.storage().instance().set(&DataKey::TokenId, &token_id);

    // Store credential
    env.storage()
        .persistent()
        .set(&DataKey::Credential(token_id), &metadata);

    // Update owner's list
    let mut owner_creds: Vec<u64> = env
        .storage()
        .persistent()
        .get(&DataKey::OwnerCredentials(to.clone()))
        .unwrap_or(Vec::new(&env));
    owner_creds.push_back(token_id);
    env.storage()
        .persistent()
        .set(&DataKey::OwnerCredentials(to.clone()), &owner_creds);

    token_id
}

pub fn transfer(env: &Env, from: Address, to: Address, token_id: u64) {
    from.require_auth();

    let metadata: CredentialMetadata = env
        .storage()
        .persistent()
        .get(&DataKey::Credential(token_id))
        .expect("Credential not found");

    // Check if soulbound
    if is_soulbound(&metadata.badge_type) {
        panic!("Non-transferable badge");
    }

    // Remove from previous owner
    let mut from_creds: Vec<u64> = env
        .storage()
        .persistent()
        .get(&DataKey::OwnerCredentials(from.clone()))
        .unwrap_or(Vec::new(&env));
    
    if let Some(index) = from_creds.iter().position(|id| id == token_id) {
        from_creds.remove(index as u32);
        env.storage()
            .persistent()
            .set(&DataKey::OwnerCredentials(from.clone()), &from_creds);
    } else {
        panic!("Unauthorized: caller is not the owner");
    }

    // Add to new owner
    let mut to_creds: Vec<u64> = env
        .storage()
        .persistent()
        .get(&DataKey::OwnerCredentials(to.clone()))
        .unwrap_or(Vec::new(&env));
    to_creds.push_back(token_id);
    env.storage()
        .persistent()
        .set(&DataKey::OwnerCredentials(to.clone()), &to_creds);
}

pub fn revoke(env: &Env, token_id: u64, note: String) {
    let mut metadata: CredentialMetadata = env
        .storage()
        .persistent()
        .get(&DataKey::Credential(token_id))
        .expect("Credential not found");
    
    metadata.is_revoked = true;
    metadata.revocation_note = note;

    env.storage()
        .persistent()
        .set(&DataKey::Credential(token_id), &metadata);
}
