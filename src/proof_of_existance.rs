use core::fmt::Debug;
use std::collections::BTreeMap;

use crate::support::DispatchResult;

pub trait Config: crate::system::Config {
    /// The type which represents the content that can be claimed using this pallet.
    /// Could be the content directly as bytes, or better yet the hash of that content.
    /// We leave that decision to the runtime developer.
    type Content: Debug + Ord;
}

/// This is the Proof of Existence Module.
/// It is a simple module that allows accounts to claim existence of some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
    /// A simple storage map from content to the owner of that content.
    /// Accounts can make multiple different claims, but each claim can only have one owner.
    /* TODO: Add a field `claims` which is a `BTreeMap` fom `T::Content` to `T::AccountId`. */
    claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
    /// Create a new instance of the Proof of Existence Module.
    pub fn new() -> Self {
        Self {
            claims: BTreeMap::new(),
        }
    }
    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
        self.claims.get(claim)
    }
    pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        match self.get_claim(&claim) {
            Some(_) => Err("Claim already exists".to_string()),
            None => {
                self.claims.insert(claim, caller);
                Ok(())
            }
        }
    }
    pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        match self.get_claim(&claim) {
            Some(owner) if *owner == caller => {
                self.claims.remove(&claim);
                Ok(())
            }
            Some(_) => Err("Caller is not the owner of the claim".to_string()),
            None => Err("Claim does not exist".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	struct TestConfig;
	impl crate::system::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	impl super::Config for TestConfig {
		type Content = String;
	}

	#[test]
	fn test_create_claim() {
		let mut pallet = Pallet::<TestConfig>::new();
		let account_id = "Alice".to_string();
		let content = "content1".to_string();

		assert!(pallet.create_claim(account_id.clone(), content.clone()).is_ok());
		assert_eq!(pallet.get_claim(&content), Some(&account_id));
	}

	#[test]
	fn test_create_claim_already_exists() {
		let mut pallet = Pallet::<TestConfig>::new();
		let account_id = "Alice".to_string();
		let content = "content1".to_string();

		pallet.create_claim(account_id.clone(), content.clone()).unwrap();
		assert!(pallet.create_claim(account_id.clone(), content.clone()).is_err());
	}

	#[test]
	fn test_revoke_claim() {
		let mut pallet = Pallet::<TestConfig>::new();
		let account_id = "Alice".to_string();
		let content = "content1".to_string();

		pallet.create_claim(account_id.clone(), content.clone()).unwrap();
		assert!(pallet.revoke_claim(account_id.clone(), content.clone()).is_ok());
		assert_eq!(pallet.get_claim(&content), None);
	}

	#[test]
	fn test_revoke_claim_not_owner() {
		let mut pallet = Pallet::<TestConfig>::new();
		let account_id1 = "Alice".to_string();
		let account_id2 = "Bob".to_string();
		let content = "content1".to_string();

		pallet.create_claim(account_id1.clone(), content.clone()).unwrap();
		assert!(pallet.revoke_claim(account_id2.clone(), content.clone()).is_err());
	}

	#[test]
	fn test_revoke_claim_not_exists() {
		let mut pallet = Pallet::<TestConfig>::new();
		let account_id = "Alice".to_string();
		let content = "content1".to_string();

		assert!(pallet.revoke_claim(account_id.clone(), content.clone()).is_err());
	}
}
