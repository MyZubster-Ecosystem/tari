// Copyright 2026. The Tari Project
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

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
