// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Conversions from types generated by [`wit-bindgen-guest-rust`] to types declared in [`linera-sdk`].

use linera_base::{
    crypto::CryptoHash,
    data_types::{Amount, BlockHeight},
    identifiers::{ApplicationId, BytecodeId, ChainId, MessageId, Owner},
};

use super::wit_system_api;

impl From<wit_system_api::CryptoHash> for CryptoHash {
    fn from(hash_value: wit_system_api::CryptoHash) -> Self {
        CryptoHash::from([
            hash_value.part1,
            hash_value.part2,
            hash_value.part3,
            hash_value.part4,
        ])
    }
}

impl From<wit_system_api::CryptoHash> for ChainId {
    fn from(hash_value: wit_system_api::CryptoHash) -> Self {
        ChainId(hash_value.into())
    }
}

impl From<wit_system_api::CryptoHash> for Owner {
    fn from(guest: wit_system_api::CryptoHash) -> Self {
        let integers = [guest.part1, guest.part2, guest.part3, guest.part4];
        Owner(CryptoHash::from(integers))
    }
}

impl From<wit_system_api::ApplicationId> for ApplicationId {
    fn from(application_id: wit_system_api::ApplicationId) -> Self {
        ApplicationId {
            bytecode_id: BytecodeId::new(application_id.bytecode_id.into()),
            creation: application_id.creation.into(),
        }
    }
}

impl From<wit_system_api::MessageId> for MessageId {
    fn from(message_id: wit_system_api::MessageId) -> Self {
        MessageId {
            chain_id: ChainId(message_id.chain_id.into()),
            height: BlockHeight(message_id.height),
            index: message_id.index,
        }
    }
}

impl From<wit_system_api::Amount> for Amount {
    fn from(balance: wit_system_api::Amount) -> Self {
        let value = ((balance.upper_half as u128) << 64) | (balance.lower_half as u128);
        Amount::from_attos(value)
    }
}
