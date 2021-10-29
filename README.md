# Encointer Parachain:

This is the repository to run encointer as a parachain in the rococo-v1 testnet. It is forked from the [Cumulus](https://github.com/paritytech/cumulus) repository and only adds the encointer-pallets and configuration.

## Demos

* [Sybil Defense Demo](docs/sybil-demo)
* [Native Downward-/ Upward Token Tx Demo](docs/downward-upward-native-token-tx)

## Polkadot-JS type extensions

[typedefs.json](typedefs.json)

## Launch a local setup including a Relay Chain and a Parachain

### Launch the Relay Chain (Local Rococo Testnet)

```bash
# Compile Polkadot
git clone https://github.com/paritytech/polkadot
git fetch
git checkout 70afaae2cfa279c9cd80e897c76eb5c2386ee017
cargo build --release

# Generate a raw chain spec
./target/release/polkadot build-spec --chain rococo-local --disable-default-bootnode --raw > rococo-local-cfde.json

# Alice
./target/release/polkadot --chain rococo-local-cfde.json --alice --tmp

# Bob (In a separate terminal)
./target/release/polkadot --chain rococo-local-cfde.json --bob --tmp --port 30334
```

### Launch the Parachain

```bash
# Compile
git clone https://github.com/encointer/encointer-parachain.git
git checkout master
cargo build --release

# Export genesis state
# --parachain-id 1862 as an example that can be chosen freely. Make sure to everywhere use the same parachain id
./target/release/encointer-collator export-genesis-state --chain encointer-local --parachain-id 1862 > encointer-local-genesis.state

# Export genesis wasm
./target/release/encointer-collator export-genesis-wasm --chain encointer-local > encointer-local-genesis.wasm

# Collator
./target/release/encointer-collator --collator --tmp --parachain-id 1862 --chain encointer-local --port 40335 --ws-port 9946 -- --execution wasm --chain ../polkadot/rococo-local-cfde-real-overseer.json --port 30337 --ws-port 9981
```

### Register the Parachain
Go to [Polkadot Apps](https://polkadot.js.org/apps/) connect to the default local port (Alice) and register the parachain via the `paraSudoWrapper` pallet. After registering, the collator should start producing blocks when the next era starts.

**Note:** Change the `ParaId` to 1862 when registering the parachain.

![image](https://user-images.githubusercontent.com/2915325/99548884-1be13580-2987-11eb-9a8b-20be658d34f9.png)


### Test Encointer Client
```bash
git clone https://github.com/encointer/encointer-node.git
cargo build --release
./target/release/encointer-client-notee -p 9946 get-phase
# should print the phase
./target/release/encointer-client-notee -p 9946 next-phase
# should progress phase
```

### Deploy on rococo

Prepare genesis state and wasm as follows:

```bash
# Export genesis state
# --parachain-id 1862 as an example that can be chosen freely. Make sure to everywhere use the same parachain id
./target/release/encointer-collator export-genesis-state --chain encointer-rococo --parachain-id 1862 > encointer-rococo-genesis.state

# Export genesis wasm
./target/release/encointer-collator export-genesis-wasm --chain encointer-rococo > encointer-rococo-genesis.wasm

```
then propose the parachain on rococo relay-chain

run collator
```
encointer-collator \
        --collator \
        --chain encointer-rococo \
        --parachain-id 1862 \
        --rpc-cors all \
        --name encointer-rococo-collator-1 \
        -- --execution wasm --chain rococo 

```

### Caveats
* Don't forget to enable file upload if you perform drag and drop for the `genesisHead` and `validationCode`. If it is not enabled, Polkadot-js will interpret the path as a string and won't complain but the registration will fail.
* Don't forget to add the argument `--chain encointer-rococo` for the custom chain config. This argument is omitted in the [Cumulus Workshop](https://substrate.dev/cumulus-workshop/).
* The relay chain and the collator need to be about equally recent. This might require frequent rebasing of this repository on the `rococo-v1` branch.
* Sanity check: The genesis state is printed when starting the collator. Make sure it matches the one from the `genesis.state` file.

### Setup local testnet with polkadot-launch
[polkadot-launch](https://github.com/paritytech/polkadot-launch) lets you easily setup a local testnet. The following procedure will setup a local testnet with three relay chain nodes and two encointer parachains. It will also setup up a XCM (cross chain messaging) channel between the two chains.

**Note 2:** The `polkadot-launch-config.json` and the commands below assume that the polkadot-launch directory is on the same level as this repo's directory.

**Preliminaries:** you need to have yarn and node installed

```bash
# We need to build it from source. The one from the yarn registry does not work with our code.
git clone https://github.com/paritytech/polkadot-launch
cd polkadot-launch
yarn install
yarn build

# In the root directory of this repository simply execute
node ../polkadot-launch/dist/index.js polkadot-launch-config.json
```

This launches the local testnet and creates 5 log files: `alice.log`, `bob.log`, `charlie.log`, which are the logs of the relay chain nodes and `9944.log`, `9955.log`, which are the logs of the two parachains.

### More Resources
* Thorough Readme about Rococo and Collators in general in the original [repository](https://github.com/paritytech/cumulus) of this fork.
* Parachains on Rococo in the [Polkadot Wiki](https://wiki.polkadot.network/docs/en/build-parachains-rococo#rococo-v1-parachain-requirements)
