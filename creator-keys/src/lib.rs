#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, Env, String,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    AlreadyRegistered = 1,
    NotRegistered = 2,
    Overflow = 3,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Creator(Address),
}

#[derive(Clone, Debug, PartialEq)]
#[contracttype]
pub struct CreatorProfile {
    pub creator: Address,
    pub handle: String,
    pub supply: u32,
}

#[contract]
pub struct CreatorKeysContract;

#[contractimpl]
impl CreatorKeysContract {
    pub fn register_creator(
        env: Env,
        creator: Address,
        handle: String,
    ) -> Result<(), ContractError> {
        creator.require_auth();

        let key = DataKey::Creator(creator.clone());
        if env.storage().persistent().has(&key) {
            return Err(ContractError::AlreadyRegistered);
        }

        let profile = CreatorProfile {
            creator,
            handle,
            supply: 0,
        };

        env.storage().persistent().set(&key, &profile);
        env.events().publish((symbol_short!("register"),), key);

        Ok(())
    }

    pub fn buy_key(env: Env, creator: Address, buyer: Address) -> Result<u32, ContractError> {
        buyer.require_auth();

        let key = DataKey::Creator(creator.clone());
        let mut profile: CreatorProfile = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(ContractError::NotRegistered)?;

        profile.supply = profile
            .supply
            .checked_add(1)
            .ok_or(ContractError::Overflow)?;
        env.storage().persistent().set(&key, &profile);
        env.events()
            .publish((symbol_short!("buy"), creator, buyer), profile.supply);

        Ok(profile.supply)
    }

    pub fn get_creator(env: Env, creator: Address) -> Result<CreatorProfile, ContractError> {
        let key = DataKey::Creator(creator);
        env.storage()
            .persistent()
            .get(&key)
            .ok_or(ContractError::NotRegistered)
    }
}

#[cfg(test)]
mod test;
