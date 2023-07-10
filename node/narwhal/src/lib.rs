// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkOS library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![forbid(unsafe_code)]
#![allow(clippy::type_complexity)]

#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate tracing;

pub mod helpers;

mod bft;
pub use bft::*;

mod event;
pub use event::*;

mod gateway;
pub use gateway::*;

mod primary;
pub use primary::*;

mod worker;
pub use worker::*;

mod ledgerservice;
pub use ledgerservice::*;

pub const CONTEXT: &str = "[MemoryPool]";

/// The maximum number of milliseconds to wait before proposing a batch.
pub const MAX_BATCH_DELAY: u64 = 2500; // ms
/// The maximum number of nodes that can be in a committee.
pub const MAX_COMMITTEE_SIZE: u16 = 4; // members
/// The maximum number of seconds before a proposed batch is considered expired.
pub const MAX_EXPIRATION_TIME_IN_SECS: i64 = 10; // seconds
/// The maximum number of round to store before garbage collecting.
pub const MAX_GC_ROUNDS: u64 = 50; // rounds
/// The maximum number of seconds before the timestamp is considered expired.
pub const MAX_TIMESTAMP_DELTA_IN_SECS: i64 = 10; // seconds
/// The maximum number of transmissions allowed in a batch.
pub const MAX_TRANSMISSIONS_PER_BATCH: usize = 1000; // transmissions
/// The maximum number of workers that can be spawned.
pub const MAX_WORKERS: u8 = 3; // workers
/// The minimum amount of stake required for a validator to bond.
/// TODO (howardwu): Change to 1_000_000_000_000u64.
pub const MIN_STAKE: u64 = 1_000u64; // microcredits
/// The port on which the memory pool listens for incoming connections.
pub const MEMORY_POOL_PORT: u16 = 5000; // port
/// The frequency at which each worker broadcasts a ping to every other node.
pub const WORKER_PING_INTERVAL: u64 = 1000; // ms

// TODO (howardwu): Switch the worker's `TransmissionID` to use or include a sha256/blake2s hash.
// TODO (howardwu): Implement sha256/blake2s hashing on `Data::Bytes`, so we can compare IDs without deserializing.
//  This is needed by the worker in `process_event_response` to guarantee integrity of the transmission.

// TODO (howardwu): Add a mechanism to keep validators connected (add reconnect logic).
