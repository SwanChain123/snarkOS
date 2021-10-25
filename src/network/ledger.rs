// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the snarkOS library.

// The snarkOS library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkOS library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkOS library. If not, see <https://www.gnu.org/licenses/>.

use crate::{Environment, Message, PeersRequest, PeersRouter};
use snarkos_ledger::{storage::Storage, LedgerState};
use snarkvm::dpc::prelude::*;

use anyhow::{anyhow, Result};
use rand::thread_rng;
use std::{
    net::SocketAddr,
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use tokio::{sync::mpsc, task};

/// Shorthand for the parent half of the `Ledger` message channel.
pub(crate) type LedgerRouter<N, E> = mpsc::Sender<LedgerRequest<N, E>>;
/// Shorthand for the child half of the `Ledger` message channel.
type LedgerHandler<N, E> = mpsc::Receiver<LedgerRequest<N, E>>;

///
/// An enum of requests that the `Ledger` struct processes.
///
#[derive(Debug)]
pub enum LedgerRequest<N: Network, E: Environment> {
    /// Mine := (miner_address, peers_router)
    Mine(Address<N>, PeersRouter<N, E>),
    /// SyncRequest := (peer_ip, block_height, peers_router)
    SyncRequest(SocketAddr, u32, PeersRouter<N, E>),
    /// SyncResponse := (peer_ip, block_height, block, peers_router)
    SyncResponse(SocketAddr, u32, Block<N>, PeersRouter<N, E>),
    /// UnconfirmedBlock := (peer_ip, block, peers_router)
    UnconfirmedBlock(SocketAddr, Block<N>, PeersRouter<N, E>),
    /// UnconfirmedTransaction := (peer_ip, transaction, peers_router)
    UnconfirmedTransaction(SocketAddr, Transaction<N>, PeersRouter<N, E>),
}

///
/// A ledger for a specific network on the node server.
///
#[derive(Clone, Debug)]
pub struct Ledger<N: Network> {
    /// The canonical chain of block hashes.
    canon: LedgerState<N>,
    /// The pool of unconfirmed transactions.
    memory_pool: MemoryPool<N>,
    /// A terminator bit for the miner.
    terminator: Arc<AtomicBool>,
    /// A status bit for the miner.
    is_mining: Arc<AtomicBool>,
}

impl<N: Network> Ledger<N> {
    /// Initializes a new instance of the ledger.
    pub fn open<S: Storage, P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(Self {
            canon: LedgerState::open::<S, P>(path)?,
            memory_pool: MemoryPool::new(),
            terminator: Arc::new(AtomicBool::new(false)),
            is_mining: Arc::new(AtomicBool::new(false)),
        })
    }

    /// Returns the latest block height.
    pub fn latest_block_height(&self) -> u32 {
        self.canon.latest_block_height()
    }

    /// Returns the latest block hash.
    pub fn latest_block_hash(&self) -> N::BlockHash {
        self.canon.latest_block_hash()
    }

    /// Returns the latest ledger root.
    pub fn latest_ledger_root(&self) -> N::LedgerRoot {
        self.canon.latest_ledger_root()
    }

    /// Returns the latest block timestamp.
    pub fn latest_block_timestamp(&self) -> Result<i64> {
        self.canon.latest_block_timestamp()
    }

    /// Returns the latest block difficulty target.
    pub fn latest_block_difficulty_target(&self) -> Result<u64> {
        self.canon.latest_block_difficulty_target()
    }

    /// Returns the latest block header.
    pub fn latest_block_header(&self) -> Result<BlockHeader<N>> {
        self.canon.latest_block_header()
    }

    /// Returns the transactions from the latest block.
    pub fn latest_block_transactions(&self) -> Result<Transactions<N>> {
        self.canon.latest_block_transactions()
    }

    /// Returns the latest block.
    pub fn latest_block(&self) -> Result<Block<N>> {
        self.canon.latest_block()
    }

    /// Returns `true` if the given ledger root exists in storage.
    pub fn contains_ledger_root(&self, ledger_root: &N::LedgerRoot) -> Result<bool> {
        self.canon.contains_ledger_root(ledger_root)
    }

    /// Returns `true` if the given block height exists in storage.
    pub fn contains_block_height(&self, block_height: u32) -> Result<bool> {
        self.canon.contains_block_height(block_height)
    }

    /// Returns `true` if the given block hash exists in storage.
    pub fn contains_block_hash(&self, block_hash: &N::BlockHash) -> Result<bool> {
        self.canon.contains_block_hash(block_hash)
    }

    /// Returns `true` if the given transaction ID exists in storage.
    pub fn contains_transaction(&self, transaction_id: &N::TransactionID) -> Result<bool> {
        self.canon.contains_transaction(transaction_id)
    }

    /// Returns `true` if the given serial number exists in storage.
    pub fn contains_serial_number(&self, serial_number: &N::SerialNumber) -> Result<bool> {
        self.canon.contains_serial_number(serial_number)
    }

    /// Returns `true` if the given commitment exists in storage.
    pub fn contains_commitment(&self, commitment: &N::Commitment) -> Result<bool> {
        self.canon.contains_commitment(commitment)
    }

    /// Returns `true` if the given ciphertext ID exists in storage.
    pub fn contains_ciphertext_id(&self, ciphertext_id: &N::CiphertextID) -> Result<bool> {
        self.canon.contains_ciphertext_id(ciphertext_id)
    }

    /// Returns the record ciphertext for a given ciphertext ID.
    pub fn get_ciphertext(&self, ciphertext_id: &N::CiphertextID) -> Result<RecordCiphertext<N>> {
        self.canon.get_ciphertext(ciphertext_id)
    }

    /// Returns the transition for a given transition ID.
    pub fn get_transition(&self, transition_id: &N::TransitionID) -> Result<Transition<N>> {
        self.canon.get_transition(transition_id)
    }

    /// Returns the transaction for a given transaction ID.
    pub fn get_transaction(&self, transaction_id: &N::TransactionID) -> Result<Transaction<N>> {
        self.canon.get_transaction(transaction_id)
    }

    /// Returns the block hash for the given block height.
    pub fn get_block_hash(&self, block_height: u32) -> Result<N::BlockHash> {
        self.canon.get_block_hash(block_height)
    }

    /// Returns the previous block hash for the given block height.
    pub fn get_previous_block_hash(&self, block_height: u32) -> Result<N::BlockHash> {
        self.canon.get_previous_block_hash(block_height)
    }

    /// Returns the block header for the given block height.
    pub fn get_block_header(&self, block_height: u32) -> Result<BlockHeader<N>> {
        self.canon.get_block_header(block_height)
    }

    /// Returns the transactions from the block of the given block height.
    pub fn get_block_transactions(&self, block_height: u32) -> Result<Transactions<N>> {
        self.canon.get_block_transactions(block_height)
    }

    /// Returns the block for a given block height.
    pub fn get_block(&self, block_height: u32) -> Result<Block<N>> {
        self.canon.get_block(block_height)
    }

    ///
    /// Performs the given `request` to the ledger.
    /// All requests must go through this `update`, so that a unified view is preserved.
    ///
    pub(super) async fn update<E: Environment>(&mut self, request: LedgerRequest<N, E>) -> Result<()> {
        match request {
            LedgerRequest::Mine(recipient, peers_router) => {
                if let Err(error) = self.mine_next_block(recipient, peers_router) {
                    error!("Failed to mine the next block: {}", error)
                }
                Ok(())
            }
            LedgerRequest::SyncRequest(peer_ip, block_height, peers_router) => {
                let request = match self.get_block(block_height) {
                    Ok(block) => PeersRequest::MessagePropagate(peer_ip, Message::SyncResponse(block.height(), block)),
                    Err(error) => PeersRequest::Failure(peer_ip, format!("{}", error)),
                };
                peers_router.send(request).await?;
                Ok(())
            }
            LedgerRequest::SyncResponse(peer_ip, block_height, block, peers_router) => {
                if let Err(error) = self.add_sync_block::<E>(block).await {
                    let request = PeersRequest::Failure(peer_ip, format!("{}", error));
                    peers_router.send(request).await?;
                }
                Ok(())
            }
            LedgerRequest::UnconfirmedBlock(peer_ip, block, peers_router) => self.add_unconfirmed_block(peer_ip, block, peers_router).await,
            LedgerRequest::UnconfirmedTransaction(peer_ip, transaction, peers_router) => {
                self.add_unconfirmed_transaction(peer_ip, transaction, peers_router).await
            }
        }
    }

    /// Mines a new block and adds it to the canon blocks.
    fn mine_next_block<E: Environment>(&mut self, recipient: Address<N>, peers_router: PeersRouter<N, E>) -> Result<()> {
        // Ensure the ledger is not already mining.
        match self.is_mining.load(Ordering::SeqCst) {
            true => return Ok(()),
            false => self.is_mining.store(true, Ordering::SeqCst),
        }

        // Prepare the new block.
        let previous_block_hash = self.latest_block_hash();
        let block_height = self.latest_block_height() + 1;

        // Compute the block difficulty target.
        let previous_timestamp = self.latest_block_timestamp()?;
        let previous_difficulty_target = self.latest_block_difficulty_target()?;
        let block_timestamp = chrono::Utc::now().timestamp();
        let difficulty_target = Blocks::<N>::compute_difficulty_target(previous_timestamp, previous_difficulty_target, block_timestamp);

        // Construct the ledger root and unconfirmed transactions.
        let ledger_root = self.canon.latest_ledger_root();
        let unconfirmed_transactions = self.memory_pool.transactions();
        let terminator = self.terminator.clone();
        let is_mining = self.is_mining.clone();

        task::spawn(async move {
            // Craft a coinbase transaction.
            let amount = Block::<N>::block_reward(block_height);
            let coinbase_transaction = match Transaction::<N>::new_coinbase(recipient, amount, &mut thread_rng()) {
                Ok(coinbase) => coinbase,
                Err(error) => {
                    error!("{}", error);
                    return;
                }
            };

            // Construct the new block transactions.
            let transactions = match Transactions::from(&[vec![coinbase_transaction], unconfirmed_transactions].concat()) {
                Ok(transactions) => transactions,
                Err(error) => {
                    error!("{}", error);
                    return;
                }
            };

            // Mine the next block.
            let block = match Block::mine(
                previous_block_hash,
                block_height,
                block_timestamp,
                difficulty_target,
                ledger_root,
                transactions,
                &terminator,
                &mut thread_rng(),
            ) {
                Ok(block) => {
                    // Set the mining status to off.
                    is_mining.store(false, Ordering::SeqCst);
                    block
                }
                Err(error) => {
                    error!("Failed to mine the next block: {}", error);
                    return;
                }
            };

            // Broadcast the next block.
            let request = PeersRequest::MessageBroadcast(Message::UnconfirmedBlock(block.height(), block));
            if let Err(error) = peers_router.send(request).await {
                error!("Failed to broadcast mined block: {}", error);
            }
        });

        Ok(())
    }

    ///
    /// Adds the given block as the next block in the ledger if the block height increments by one.
    ///
    async fn add_sync_block<E: Environment>(&mut self, block: Block<N>) -> Result<()> {
        if block.height() + 1 == self.latest_block_height() {
            match self.canon.add_next_block(&block) {
                Ok(()) => {
                    // On success, filter the memory pool of its transactions and the block if it exists.
                    // TODO (howardwu): Filter the memory pool, removing any now confirmed transctions.
                    self.memory_pool.clear_transactions();
                    debug!("Advancing ledger to block {}", self.latest_block_height());
                    // TODO (howardwu) - Set the terminator bit to true.
                }
                Err(error) => error!("{}", error),
            };
            Ok(())
        } else {
            return Err(anyhow!("Attempted to add the wrong block to the ledger"));
        }
    }

    ///
    /// Adds the given unconfirmed block:
    ///     1) as the next block in the ledger if the block height increments by one, or
    ///     2) to the memory pool for later use.
    ///
    async fn add_unconfirmed_block<E: Environment>(
        &mut self,
        peer_ip: SocketAddr,
        block: Block<N>,
        peers_router: PeersRouter<N, E>,
    ) -> Result<()> {
        if block.height() + 1 == self.latest_block_height() {
            match self.canon.add_next_block(&block) {
                Ok(()) => {
                    // On success, filter the memory pool of its transactions and the block if it exists.
                    // TODO (howardwu): Filter the memory pool, removing any now confirmed transctions.
                    self.memory_pool.clear_transactions();

                    debug!("Advancing ledger to block {}", self.latest_block_height());
                    // Upon success, propagate the unconfirmed block to the connected peers.
                    let request = PeersRequest::MessagePropagate(peer_ip, Message::UnconfirmedBlock(block.height(), block));
                    peers_router.send(request).await?;
                    // TODO (howardwu) - Set the terminator bit to true.
                }
                Err(error) => error!("{}", error),
            };
            Ok(())
        } else if block.height() + 1 > self.latest_block_height() {
            debug!("Adding unconfirmed block {} to memory pool", block.height());
            // Ensure the unconfirmed block is new.
            if !self.contains_block_hash(&block.block_hash())? {
                // Attempt to add the unconfirmed block to the memory pool.
                match self.memory_pool.add_block(block.clone()) {
                    Ok(()) => {
                        // Upon success, propagate the unconfirmed block to the connected peers.
                        let request = PeersRequest::MessagePropagate(peer_ip, Message::UnconfirmedBlock(block.height(), block));
                        peers_router.send(request).await?;
                    }
                    Err(error) => error!("{}", error),
                }
            }
            Ok(())
        } else {
            return Err(anyhow!("Attempted to add a stale block to the memory pool"));
        }
    }

    ///
    /// Adds the given unconfirmed transaction to the memory pool.
    ///
    async fn add_unconfirmed_transaction<E: Environment>(
        &mut self,
        peer_ip: SocketAddr,
        transaction: Transaction<N>,
        peers_router: PeersRouter<N, E>,
    ) -> Result<()> {
        debug!("Adding unconfirmed transaction {} to memory pool", transaction.transaction_id());
        // Ensure the unconfirmed transaction is new.
        if !self.contains_transaction(&transaction.transaction_id())? {
            // Attempt to add the unconfirmed transaction to the memory pool.
            match self.memory_pool.add_transaction(&transaction) {
                Ok(()) => {
                    // Upon success, propagate the unconfirmed transaction to the connected peers.
                    let request = PeersRequest::MessagePropagate(peer_ip, Message::UnconfirmedTransaction(transaction));
                    peers_router.send(request).await?;
                }
                Err(error) => error!("{}", error),
            }
        }
        Ok(())
    }
}
