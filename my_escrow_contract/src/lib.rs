// Copyright 2026, The Tari Project
// SPDX-License-Identifier: BSD-3-Clause

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum EscrowStatus {
    Pending,
    Released,
    Disputed,
    Refunded,
}

pub struct EscrowState {
    pub amount: u64,
    pub buyer_pubkey: String,
    pub seller_pubkey: String,
    pub arbiter_pubkey: Option<String>,
    pub status: EscrowStatus,
}

impl EscrowState {
    pub fn create_escrow(
        &mut self,
        amount: u64,
        buyer_pubkey: String,
        seller_pubkey: String,
        arbiter_pubkey: Option<String>,
    ) -> Result<(), String> {
        if self.status != EscrowStatus::Pending {
            return Err("Escrow already exists".to_string());
        }
        if amount == 0 {
            return Err("Amount must be greater than 0".to_string());
        }
        self.amount = amount;
        self.buyer_pubkey = buyer_pubkey;
        self.seller_pubkey = seller_pubkey;
        self.arbiter_pubkey = arbiter_pubkey;
        self.status = EscrowStatus::Pending;
        Ok(())
    }

    pub fn release_funds(&mut self, sender_pubkey: String) -> Result<(), String> {
        if self.status != EscrowStatus::Pending {
            return Err("Escrow is not pending".to_string());
        }
        if sender_pubkey != self.buyer_pubkey && sender_pubkey != self.seller_pubkey {
            return Err("Only buyer or seller can confirm release".to_string());
        }
        self.status = EscrowStatus::Released;
        Ok(())
    }

    pub fn dispute_escrow(&mut self, sender_pubkey: String) -> Result<(), String> {
        if self.status != EscrowStatus::Pending {
            return Err("Escrow is not pending".to_string());
        }
        if Some(sender_pubkey) != self.arbiter_pubkey {
            return Err("Only the arbiter can raise a dispute".to_string());
        }
        self.status = EscrowStatus::Disputed;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_release_escrow() {
        let mut contract = EscrowState {
            amount: 0,
            buyer_pubkey: "buyer1".to_string(),
            seller_pubkey: "seller1".to_string(),
            arbiter_pubkey: None,
            status: EscrowStatus::Pending,
        };

        assert!(contract.create_escrow(100, "buyer1".to_string(), "seller1".to_string(), None).is_ok());
        assert_eq!(contract.amount, 100);
        assert_eq!(contract.status, EscrowStatus::Pending);

        assert!(contract.release_funds("buyer1".to_string()).is_ok());
        assert_eq!(contract.status, EscrowStatus::Released);
    }
}
