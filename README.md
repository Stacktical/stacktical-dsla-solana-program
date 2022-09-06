# DSLA on Solana

## Prerequisites
* [Node.js](https://nodejs.org) version 16.13.0 LTS
* [npm](https://npmjs.com) version 7.21.1
* [Yarn](https://yarnpkg.com/getting-started/install) version 1.22.19
* [Rust](https://www.rust-lang.org/tools/install) version 1.63.0 (install using rustup)
* [Solana CLI tools](https://docs.solana.com/cli/install-solana-cli-tools) version 1.11.10
* [avm](https://www.anchor-lang.com/docs/installation) version 0.25.0
* [Anchor](https://www.anchor-lang.com/docs/installation) version 0.25.0

Ensure you have a Solana wallet/account created for your local Solana install. Confirm by running:
`solana address` in your terminal. If no wallet has been created an error will be thrown along with the command to copy, paste and run to create a new wallet.

## Build

To install and build, run the following commands:

`yarn install`
`anchor build`

## Test

To run the built-in tests and confirm install, run the following commands:

`yarn install`
`anchor test`