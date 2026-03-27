#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, String, Vec};

pub mod badges;
pub mod nft;

use crate::badges::{BadgeType, CredentialMetadata};
use crate::nft::DataKey;

#[contract]
pub struct CredentialsContract;

#[contractimpl]
impl CredentialsContract {
    /// Initialize the credentials contract
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Contract already initialized");
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TokenId, &0u64);
    }

    /// Add an authorized minter
    pub fn add_minter(env: Env, minter: Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        env.storage()
            .persistent()
            .set(&DataKey::Minters(minter.clone()), &true);
    }

    /// Mint a new credential badge
    pub fn mint(
        env: Env,
        minter: Address,
        to: Address,
        badge_type: BadgeType,
        expiry: u64,
        verification_link: String,
    ) -> u64 {
        minter.require_auth();

        // Check if caller is authorized minter or admin
        let is_minter = env
            .storage()
            .persistent()
            .get::<_, bool>(&DataKey::Minters(minter.clone()))
            .unwrap_or(false);
        
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        
        if !is_minter && minter != admin {
            panic!("Unauthorized: caller is not a minter");
        }

        let metadata = CredentialMetadata {
            badge_type: badge_type.clone(),
            issuer: minter.clone(),
            issued_at: env.ledger().timestamp(),
            expiry,
            verification_link: verification_link.clone(),
            is_revoked: false,
            revocation_note: String::from_str(&env, ""),
        };

        let token_id = nft::mint(&env, to.clone(), metadata);

        env.events().publish(
            (symbol_short!("mint"), symbol_short!("cred")),
            (to, token_id, badge_type),
        );

        token_id
    }

    /// Revoke a credential badge
    pub fn revoke(env: Env, admin: Address, token_id: u64, note: String) {
        admin.require_auth();
        let stored_admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        if admin != stored_admin {
            panic!("Unauthorized: only admin can revoke");
        }

        nft::revoke(&env, token_id, note.clone());

        env.events().publish(
            (symbol_short!("revoke"), symbol_short!("cred")),
            (token_id, note),
        );
    }

    /// Get all credentials for an account
    pub fn get_credentials(env: Env, account: Address) -> Vec<CredentialMetadata> {
        let owner_creds: Vec<u64> = env
            .storage()
            .persistent()
            .get(&DataKey::OwnerCredentials(account))
            .unwrap_or(Vec::new(&env));
        
        let mut results = Vec::new(&env);
        for token_id in owner_creds.iter() {
            let metadata: CredentialMetadata = env
                .storage()
                .persistent()
                .get(&DataKey::Credential(token_id))
                .unwrap();
            results.push_back(metadata);
        }
        results
    }

    /// Check if account has a specific valid credential
    pub fn has_credential(env: Env, account: Address, badge_type: BadgeType) -> bool {
        let owner_creds: Vec<u64> = env
            .storage()
            .persistent()
            .get(&DataKey::OwnerCredentials(account))
            .unwrap_or(Vec::new(&env));
        
        for token_id in owner_creds.iter() {
            let metadata: CredentialMetadata = env
                .storage()
                .persistent()
                .get(&DataKey::Credential(token_id))
                .unwrap();
            
            if metadata.badge_type == badge_type && metadata.is_valid(&env) {
                return true;
            }
        }
        false
    }

    /// Check if a credential is still valid
    pub fn is_valid(env: Env, token_id: u64) -> bool {
        if let Some(metadata) = env
            .storage()
            .persistent()
            .get::<_, CredentialMetadata>(&DataKey::Credential(token_id)) {
            metadata.is_valid(&env)
        } else {
            false
        }
    }

    /// Transfer a non-soulbound badge
    pub fn transfer(env: Env, from: Address, to: Address, token_id: u64) {
        nft::transfer(&env, from, to, token_id);
    }
}
