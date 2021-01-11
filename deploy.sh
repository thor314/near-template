#!/bin/bash

near deploy --wasmFile res/dummy.wasm --initFunction "new" --initArgs '' --accountId $1.testnet
