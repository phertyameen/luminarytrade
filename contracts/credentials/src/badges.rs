use soroban_sdk::{contracttype, Address, String, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BadgeType {
    CreditScore,
    FraudFree,
    Staking,
    Participation,
    Volume,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct CredentialMetadata {
    pub badge_type: BadgeType,
    pub issuer: Address,
    pub issued_at: u64,
    pub expiry: u64,
    pub verification_link: String,
    pub is_revoked: bool,
    pub revocation_note: String,
}

impl CredentialMetadata {
    pub fn is_expired(&self, env: &Env) -> bool {
        self.expiry > 0 && env.ledger().timestamp() > self.expiry
    }

    pub fn is_valid(&self, env: &Env) -> bool {
        !self.is_revoked && !self.is_expired(env)
    }
}
