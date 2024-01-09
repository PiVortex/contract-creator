#!/bin/sh

./build.sh

if [ $? -ne 0 ]; then
  echo ">> Error building contract"
  exit 1
fi

echo ">> Deploying contract"

# https://docs.near.org/tools/near-cli#near-dev-deploy
 npx near deploy --accountId contractcreator.testnet --wasmFile /home/pivortex/projects/contract-creator/contract/target/wasm32-unknown-unknown/release/contract.wasm