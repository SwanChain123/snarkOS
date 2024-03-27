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

/// A set of modes, indicating distinct forms of malicious behavior.
pub enum MaliceMode {
    /// As a malicious validator, allow already seen transactions.
    AllowSeenTransaction,
    /// As a malicious validator, flood the network with large BatchProposals.
    BatchProposalFlood,
    /// As a malicious validator, randomly inject confirmed transactions into the ledger.
    DoubleSpend,
    /// As a malicious validator, flood the network with PeerResponse.
    PeerResponseFlood,
    /// As a malicious validator, resend a confirmed transaction.
    ResendConfirmed,
    /// As a malicious validator, try to confuse the network with a round for in advance.
    RoundsFarAhead,
    /// As a malicious validator, skip transaction uniqueness and correctness checks.
    SkipTransactionCheck,
    /// As a malicious validator, withhold leader certificate.
    WithholdLeaderCertificate,
    /// As a (malicious) validator, use the bootstrap peers from the file /tmp/bootstrap_peers.txt (one per line ip:port).
    UseBootstrapPeers,
}
