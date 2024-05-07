#!/usr/bin/env bash
set -e

# source ./env.sh

# Set up a trap to catch the SIGINT signal (CTRL-C)
trap 'cleanup' SIGINT
cleanup() {
  echo "Cleaning up..."
  killall clarus-node
  killall tail
  rm -rf data

  exit 1
}

# Insert session keys
cd ./scripts/session_keys
./run.sh
cd ../../

./target/release/clarus-node \
  --chain clarus \
  -d data/validator1 \
  --name validator1 \
  --in-peers 256 \
  --validator \
  --rpc-external \
  --rpc-cors all \
  --rpc-methods=unsafe \
  --detailed-log-output \
  --state-pruning archive \
  --blocks-pruning archive \
  --node-key 0x74a8cfbadb5d2b0178ec124791bfa8346ac3550a4f689923c806428090055277 &> data/log.txt &

./target/release/clarus-node \
  --chain clarus \
  -d data/validator2 \
  --name validator2 \
  --validator \
  --port 30334 \
  --rpc-port 9934 \
  --rpc-external \
  --rpc-cors all \
  --rpc-methods=unsafe \
  --state-pruning archive \
  --blocks-pruning archive \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWRm651Kd5GmsLTHJbgX5chQS5npx9ttLgo46UsegCMoNM &> /dev/null &

./target/release/clarus-node \
  --chain clarus \
  -d data/validator3 \
  --name validator3 \
  --validator \
  --port 30335 \
  --rpc-port 9935 \
  --rpc-external \
  --rpc-cors all \
  --rpc-methods=unsafe \
  --state-pruning archive \
  --blocks-pruning archive \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWRm651Kd5GmsLTHJbgX5chQS5npx9ttLgo46UsegCMoNM &> /dev/null &

tail -f data/log.txt &
read -r

exit 0
