# Template for a NEAR project

Contains:
- a setup script
- Cargo.toml setup with simulation testing and NEAR sdk crates
- build and deploy scripts
- a src directory containing lib.rs, with common imports and ignore attributes
- a test directory containing a simulation test setup
- my preferred .gitignore and .rustfmt.toml setup

## Setup script usage:
`./setup.sh my_project_name`
will change all instances of "dummy" to whatever you'd like to call this project, and rename this
README to `instructions.md`.

## Build script usage:
`./build.sh` to build the wasm blob. The wasm blob will be copied into `res/`.

## Deploy script usage:
`./deploy.sh MYADDRESS` to deploy your wasm blob to the NEAR testnet.

Note that if you change the constructor arguments, You will have to modify the arguments int the `deploy.sh`
script. If my constructor is `new(name: String, number: u32) -> Self`, and I want to initialize with ("Todd", 1), I
will change the script to:

`near deploy --wasmFile res/dummy.wasm --initFunction "new" --initArgs '{"name": "Todd", "number": 1}' --accountId $1.testnet`

and call `./deploy.sh MYADDRESS`, or to provide arguments, change `deploy.sh` to:

`near deploy --wasmFile res/dummy.wasm --initFunction "new" --initArgs "{\"name\": $1, \"number\": $2}" --accountId $1.testnet`

and call `./deploy.sh MYADDRESS MYNAME MYNUM`.
