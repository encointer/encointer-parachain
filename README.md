# Encointer Parachain:

This is the repository for the encointer collator to operate a parachain. 
It is forked from the [Cumulus](https://github.com/paritytech/cumulus) repository.

## Demos

* [Sybil Defense Demo](docs/sybil-demo)
* [Native Downward-/ Upward Token Tx Demo](docs/downward-upward-native-token-tx)

## Launch a local setup with polkadot-launch

###Build sudo-patched polkadot binary  
Inside the parent directory of this repo, execute the following commands:
```bash
git clone https://github.com/paritytech/polkadot
cd polkadot
git fetch
git checkout v0.9.18
# add sudo to all types of relaychain to simplify and accelerate testing
git cherry-pick 755a4b69c94ba75618080c76d25133b11c1d58db
# use `cargo install diener` if not done already
diener update --substrate --branch polkadot-v0.9.18
cargo build --release
```

Run:
`cp target/release/polkadot ../../bin/polkadot-0.9.24-sudo`  
It is assumed that the bin directory exists at this relative path.
Note: When using the `launch-rococo-local-with-encointer` setup it is assumed that the `bin` folder also contains a `polkadot-v0.9.37` binary.


### Setup local testnet with polkadot-launch
[polkadot-launch](https://github.com/paritytech/polkadot-launch) lets you easily setup a local testnet. The following procedure will setup a local testnet with three relay chain nodes and two encointer parachains. It will also setup up a XCM (cross chain messaging) channel between the two chains.

**Note 2:** The `polkadot-launch-config.json` and the commands below assume that the polkadot-launch directory is on the same level as this repo's directory.

**Preliminaries:** you need to have yarn and node installed

Inside the parent directory of this repo, execute the following commands:
```bash
# We need to build it from source. The one from the yarn registry does not work with our code.
git clone https://github.com/paritytech/polkadot-launch
cd polkadot-launch
yarn install
yarn build

cd ../encointer-parachain
cargo build --release
node ../polkadot-launch/dist/cli.js ./polkadot-launch/launch-rococo-local-with-launch.json
```

This launches the local testnet and creates 5 log files: `alice.log`, `bob.log`, `charlie.log`, which are the logs of the relay chain nodes and `9944.log`, `9955.log`, which are the logs of the two parachains.

Executing the `./tmux_setup.sh` script at the project root will set up tmux with three panes that tailing the logs of `alice.log`, `9944.log` and `9955.log`.

## manual test setup step by step

```
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
./target/release/encointer-collator export-genesis-state --chain launch-rococo-local --parachain-id 2000 > encointer-local-genesis.state

# Export genesis wasm
./target/release/encointer-collator export-genesis-wasm --chain launch-rococo-local > encointer-local-genesis.wasm

# Collator
./target/release/encointer-collator --collator --tmp --parachain-id 2000 --chain launch-rococo-local --port 40335 --ws-port 9946 -- --execution wasm --chain ../polkadot/rococo-local-cfde.json --port 30337 --ws-port 9981
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

## Test state migrations on live data
Compile the node with

```bash
cargo build --release --features try-runtime
```

Then run the state migrations with live data from the encointer-kusama parachain

```bash
./target/release/encointer-collator try-runtime \
  --chain encointer-kusama-local \
  --runtime ./target/release/wbuild/encointer-runtime/encointer_runtime.wasm \
  on-runtime-upgrade --checks=all \
  live --uri wss://kusama.api.encointer.org:443
```

## Benchmark the runtimes
In `./scripts` we have two scripts for benchmarking the runtimes.

### Current benchmark
The current weights have been benchmarked with the following reference hardware:

    2.3 GHz 8-Core Intel Core i9
    16 GB 2400 MHz DDR4

### Running benchmark
1. Compile the node with: `cargo build --release --features runtime-benchmarks`
2. run: `./scripts/benchmark_launch_runtime.sh` and `./scripts/benchmark_encointer_runtime.sh`.
3. If changed, update the reference hardware above.

### Adding new pallets to be benchmarked
Every pallet with a `type WeightInfo` parameter in its config must be benchmarked.

1. [Cargo.toml] add `<new_pallet>/runtime-benchmarks` in the `runtime-benchmarks` feature section.
2. [runtime] Add the new pallet to be benchmarked to the `define_benchmarks!` macro in the runtime.
3. Run the benchmark as advised above.
4. [runtime/src/weights] add the new file to the modules
4. [runtime] replace the placeholder `type WeightInfo = ()` with `type WeightInfo = weights::<new_pallet>::WeightInfo<Runtime>`

### Update hardcoded chain-specs
We have some hardcoded chain-specs to be sure that independent binaries can work with the same `genesis_hash`. However,
sometimes we need to update the nodes' (i.e. the client's) spec, when we perform a substrate update. The following script
creates new hard-coded chain-specs while migrating relevant data.

`./scripts/update_hardcoded_chain_specs.py --migrate-genesis`


### More Resources
* Thorough Readme about Rococo and Collators in general in the original [repository](https://github.com/paritytech/cumulus) of this fork.
* Parachains on Rococo in the [Polkadot Wiki](https://wiki.polkadot.network/docs/en/build-parachains-rococo#rococo-v1-parachain-requirements)
