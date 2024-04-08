VERSION=0.0.5
NAME=swan-aleo-proof
REPO=martian123

.PHONY: build
build:
	cargo build --release --manifest-path=../../Cargo.toml && \
	cp ../../target/release/snarkos ./swan-aleo-proof

.PHONY: docker
docker: build
	@sudo docker build -t $(REPO)/$(NAME):$(VERSION) . 

.PHONY: push
push: docker
	@sudo docker push $(REPO)/$(NAME):$(VERSION)


.PHONY: run
run: push
	@sudo docker run -it --network=bridge --rm \
	--env RECEIVE_PROOF_URL="https://192.168.11.152:9084/api/v1/computing/cp/receive/ubi" \
	--env TASKID="10001" --env TASK_TYPE="1" --env ZK_TYPE="aleo-proof" --env NAME_SPACE="ubi-task-10001" \
	--env PARAM_URL='{"nonce_ex":122,"nonce_len":1,"address":"aleo1n6utz6c8gtgqp4xfkm03ckf4n9fupuv3jf0feea9ggt547skasxq423ajy","min_proof_target":500}' \
	-v /root/.aleo:/root/.aleo \
	$(REPO)/$(NAME):$(VERSION) \
    /mnt/init/cmd start --prover
  