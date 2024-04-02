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

use serde::Deserialize;
use std::env;
use anyhow::{anyhow, Result};

#[derive(Debug, Deserialize, Clone)]
pub struct EnvArgs {
    pub receive_proof_url: String,
    pub task_id: u64,
    pub task_type: u64,
    pub zk_type: String,
    pub name_space: String,
    pub input_params: InputArgs,
}

#[derive(Debug, Deserialize, Clone)]
pub struct InputArgs {
    pub nonce_ex: u64,
    pub nonce_len: u64,
    pub address: String,
    pub min_proof_target: u64,
}

pub fn parse_env_args() -> Result<EnvArgs> {
    let receive_proof_url = match env::var("RECEIVE_PROOF_URL") {
        Ok(receive_proof_url) => {
            info!("Env RECEIVE_PROOF_URL: {}", receive_proof_url);
            receive_proof_url
        },
        Err(_) => {
            return Err(anyhow!("Env variable RECEIVE_PROOF_URL is not set"));
        },
    };

    let task_id = match env::var("TASKID") {
        Ok(task_id) => {
            info!("Env TASKID: {}", task_id);
            task_id.parse().unwrap()
        },
        Err(_) => {
            return Err(anyhow!("Env variable TASKID is not set"));
        },
    };

    let task_type = match env::var("TASK_TYPE") {
        Ok(task_type) => {
            info!("Env TASK_TYPE: {}", task_type);
            task_type.parse().unwrap()
        },
        Err(_) => {
            return Err(anyhow!("Env variable TASK_TYPE is not set"));
        },    
    };

    let zk_type = match env::var("ZK_TYPE") {
        Ok(zk_type) => {
            info!("Env ZK_TYPE: {}", zk_type);
            zk_type
        },
        Err(_) => {
            return Err(anyhow!("Env variable ZK_TYPE is not set"));
        },
    };

    let name_space = match env::var("NAME_SPACE") {
        Ok(name_space) => {
            info!("Env NAME_SPACE: {}", name_space);
            name_space
        },
        Err(_) => {
            return Err(anyhow!("Env variable NAME_SPACE is not set"));
        },
    };

    let input_parameter = match env::var("PARAM_URL") {
        Ok(input_parameter) => {
            info!("Input Parameter: {}", input_parameter);
            serde_json::from_str(&input_parameter).unwrap()
        },
        Err(_) => {
            return Err(anyhow!("Env variable PARAM_URL is not set"));
        },
    };

    let args = EnvArgs {
        receive_proof_url: receive_proof_url,
        task_id: task_id,
        task_type: task_type,
        zk_type: zk_type,
        name_space: name_space,
        input_params: input_parameter,
    };

    Ok(args)
}

#[cfg(test)]
mod tests {
    use crate::prover::parse::InputArgs;

    #[test]
    fn test_parse_development_and_genesis() {
        let input_args = r#"{"nonce_ex": 122, "nonce_len": 1, "address": "aleo1n6utz6c8gtgqp4xfkm03ckf4n9fupuv3jf0feea9ggt547skasxq423ajy", "min_proof_target": 100}"#;
        let args: InputArgs = serde_json::from_str(input_args).unwrap();
        assert_eq!(args.nonce_ex, 122);
        assert_eq!(args.nonce_len, 1);
        assert_eq!(args.address, "aleo1n6utz6c8gtgqp4xfkm03ckf4n9fupuv3jf0feea9ggt547skasxq423ajy");
        assert_eq!(args.min_proof_target, 100);
    }
}