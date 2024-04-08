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
use reqwest::Error;

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

pub async fn send_post_request(
    url: &str,
    task_id: u64,
    task_type: u64,
    nonce: u64,
    solution: &str,
    proof: &str,
    target: u64,
    zk_type: &str,
    name_space: &str,
) -> Result<(), Error> {
    let client = reqwest::Client::new();

    let json_proof = format!("task_id:{},nonce:{},solution:{},proof:{},target:{}",
        task_id, nonce, solution, proof, target
    );

    let json_data = format!(
        r#"
        {{
            "task_id": "{}",
            "task_type": "{}",
            "proof": "{}",
            "zk_type": "{}",
            "name_space": "{}"
        }}
        "#,
        task_id, task_type, json_proof, zk_type, name_space
    );

    println!("POST: {url}\n data {}", json_data);

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(json_data.to_owned())
        .send()
        .await;

    match response {
        Ok(response) => {
            if response.status().is_success() {
                println!("POST请求成功！");
            } else {
                println!("POST请求失败：{}", response.status());
            }
        },
        Err(error) => {
            println!("POST error: {}", error);
            return Err(error);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::prover::parse::{send_post_request, InputArgs};

    #[test]
    fn test_parse_development_and_genesis() {
        let input_args = r#"{"nonce_ex": 122, "nonce_len": 1, "address": "aleo1n6utz6c8gtgqp4xfkm03ckf4n9fupuv3jf0feea9ggt547skasxq423ajy", "min_proof_target": 100}"#;
        let args: InputArgs = serde_json::from_str(input_args).unwrap();
        assert_eq!(args.nonce_ex, 122);
        assert_eq!(args.nonce_len, 1);
        assert_eq!(args.address, "aleo1n6utz6c8gtgqp4xfkm03ckf4n9fupuv3jf0feea9ggt547skasxq423ajy");
        assert_eq!(args.min_proof_target, 100);
    }

    #[tokio::test]
    async fn test_send_post_request() {
        let url = "https://127.0.0.1:9084/api/v1/computing/cp/receive/ubi";
        let task_id = 10011;
        let task_type = 1;
        let nonce = 2882303761517117440u64;
        let solution = "puzzle1cryng056kcer4duyaxu68uc3w6hsxsajs0wrwu0xepgsqffsx6rm5pk3t0m9a2yq63pnsr7jw64qz9zdwf6";
        let zk_type = "aleo-proof";
        let name_space = "ubi-task-10011";

        let result = send_post_request(url, task_id, task_type, nonce, solution, 100, zk_type, name_space).await;

        assert!(result.is_ok());
    }
}