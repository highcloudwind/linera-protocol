// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Conversions from types declared in [`linera_sdk`] to types generated by [`wit_bindgen_rust`].

use super::writable_system as system;
use crate::{ApplicationId, CryptoHash, EffectId, SessionId};

impl From<CryptoHash> for system::CryptoHash {
    fn from(hash_value: CryptoHash) -> Self {
        let parts = <[u64; 8]>::from(hash_value);

        system::CryptoHash {
            part1: parts[0],
            part2: parts[1],
            part3: parts[2],
            part4: parts[3],
            part5: parts[4],
            part6: parts[5],
            part7: parts[6],
            part8: parts[7],
        }
    }
}

impl From<ApplicationId> for system::ApplicationId {
    fn from(application_id: ApplicationId) -> system::ApplicationId {
        system::ApplicationId {
            bytecode_id: application_id.bytecode.0.into(),
            creation: application_id.creation.into(),
        }
    }
}

impl From<SessionId> for system::SessionId {
    fn from(session_id: SessionId) -> Self {
        system::SessionId {
            application_id: session_id.application_id.into(),
            kind: session_id.kind,
            index: session_id.index,
        }
    }
}

impl From<EffectId> for system::EffectId {
    fn from(effect_id: EffectId) -> Self {
        system::EffectId {
            chain_id: effect_id.chain_id.0.into(),
            height: effect_id.height.0,
            index: effect_id.index,
        }
    }
}
impl From<log::Level> for system::LogLevel {
    fn from(level: log::Level) -> Self {
        match level {
            log::Level::Trace => system::LogLevel::Trace,
            log::Level::Debug => system::LogLevel::Debug,
            log::Level::Info => system::LogLevel::Info,
            log::Level::Warn => system::LogLevel::Warn,
            log::Level::Error => system::LogLevel::Error,
        }
    }
}
