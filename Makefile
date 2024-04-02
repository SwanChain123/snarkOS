.PHONY: build
build:
	cargo build --release

.PHONY: start
start:
	export RECEIVE_PROOF_URL="https://127.0.0.1:9084/api/v1/computing/cp/receive/ubi" && \
	 export TASKID="10001" && export TASK_TYPE="1" && export ZK_TYPE="aleo-proof" && export NAME_SPACE="ubi-task-10001" && \
	 export PARAM_URL='{"nonce_ex":122,"nonce_len":1,"address":"aleo1n6utz6c8gtgqp4xfkm03ckf4n9fupuv3jf0feea9ggt547skasxq423ajy","min_proof_target":100}' && \
	  ./target/release/snarkos start --prover

.PHONY: restart
restart:build
	export RECEIVE_PROOF_URL="https://127.0.0.1:9084/api/v1/computing/cp/receive/ubi" && \
	 export TASKID="10001" && export TASK_TYPE="1" && export ZK_TYPE="aleo-proof" && export NAME_SPACE="ubi-task-10001" && \
	 export PARAM_URL='{"nonce_ex":122,"nonce_len":1,"address":"aleo1n6utz6c8gtgqp4xfkm03ckf4n9fupuv3jf0feea9ggt547skasxq423ajy","min_proof_target":100}' && \
	  ./target/release/snarkos start --prover