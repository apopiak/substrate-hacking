# A Customized Substrate Node that does Stuff

This Substrate node was built following two tutorials from the [Substrate Developer Hub](https://substrate.dev/):

1) [Perform a Forkless Upgrade](https://substrate.dev/docs/en/tutorials/upgrade-a-chain/sudo-upgrade)
2) [Build a Permissioned Network](https://substrate.dev/docs/en/tutorials/build-permission-network/)

:rocket: It's meant to help beginners get a taste for how easy it is to add functionality to a Substrate-built blockchain in a modular way. This is originally a fork from the Substrate Template Node. Please follow Substrate's tutorial [here](https://substrate.dev/docs/en/tutorials/create-your-first-substrate-chain/) if this is your first time working with Substrate.

:bulb: :goal_net: The goal is to pass in a WASM binaries of compiles runtimes for new modules / pallet functionality :goal_net: :bulb:

# Build, Run and Try Things

The [Compiling Substrate](https://substrate.dev/docs/en/tutorials/create-your-first-substrate-chain/) section teaches everything you need to know to get this node up and running :hammer_and_wrench:

## Run
### Getting the Permissioned Network Up and Running
:bulb: :bank: Let's start by running our permissioned network. Clone this directory, cd into it and run the following (don't sweat it, it's normal that this might take a little while):

```bash
cargo build --release
```

Based on the [tutorial](https://substrate.dev/docs/en/tutorials/build-permission-network/), let's launch 3 well-known nodes. Paste the following commands in separate terminals under the same directory:

```bash
// Start with Alice's node 
./target/release/node-template --chain=local --base-path ~/tmp/validator1 --alice --node-key=c12b6d18942f5ee8528c8e2baf4e147b5c5c18710926ea492d09cbd9f6c9f82a --port 30333 --ws-port 9944
```
```bash
// Now with Bob's node 
./target/release/node-template --chain=local --base-path ~/tmp/validator2 --bob --node-key=6ce3be907dbcabf20a9a5a60a712b4256a54196000a8ed4050d352bc113f8c58 --port 30334 --ws-port 9945

// And finally Charlie's
./target/release/node-template --chain=local --base-path ~/tmp/validator3 --name charlie  --node-key=3a9d5b35b9fb4c42aafadeca046f6bf56107bd2579687f069b42646684b94d9e --port 30335 --ws-port=9946 --offchain-worker always
```
:tv: Now go to https://polkadot.js.org/apps/ to see what's happening live! This webapp developed by Polkadot allows you to connect your local node by selecting a custom endpoint - make sure it's connected to `127.0.0.1:9944`. While you're there, go to the _Settings_ &rarr; _Developer_ page:
add the following in the:
```bash
{
  "PeerId": "(Vec<u8>)"
}
```
Head over to _Chainstate_ &rarr; _Storage_ and select `"nodeAuthorization"` and the `"wellKnownNodes()"` function. Hit the `(+)` button and this will allow you to see the well known nodes, Alice, Bob and Charlie.
:vertical_traffic_light: **Note**: refresh the page if it's not displaying anything.

See how to add connections by following the original tutorial. For the purpose of this codebase, we've done what we need and have our permissioned network up and running. Now, let's add in an upgrade right from the UI.

### Adding Extrinsics from UI

The key runtime modules to achieve forkless upgrades in our usage are: Sudo and Schedular. The Sudo pallet is already a part of the FRAME-based node template we're using. The Schedular pallet was added as per [the tutorial referred to above](https://substrate.dev/docs/en/tutorials/upgrade-a-chain/).

Using the *Extrinsic* from the Sudo pallet, you can experiment with adding any of the runtime updates in the form of WASM binaries to this permissioned network. Please refer to the folder: WASM-runtimes.


//TODO: Implement different governance instead of Sudo
//TODO: Implement a type of multisig account that receives runtime update for members to vote on before it //gets included to runtime


### Single Node Development Chain

Purge any existing dev chain state:

```bash
./target/release/node-template purge-chain --dev
```

Start a dev chain:

```bash
./target/release/node-template --dev
```

Or, start a dev chain with detailed logging:

```bash
RUST_LOG=debug RUST_BACKTRACE=1 ./target/release/node-template -lruntime=debug --dev
```

### Multi-Node Local Testnet

If you want to see the multi-node consensus algorithm in action, refer to
[our Start a Private Network tutorial](https://substrate.dev/docs/en/tutorials/start-a-private-network/).

## Template Structure

A Substrate project such as this consists of a number of components that are spread across a few
directories.

### Node

A blockchain node is an application that allows users to participate in a blockchain network.
Substrate-based blockchain nodes expose a number of capabilities:

-   Networking: Substrate nodes use the [`libp2p`](https://libp2p.io/) networking stack to allow the
    nodes in the network to communicate with one another.
-   Consensus: Blockchains must have a way to come to
    [consensus](https://substrate.dev/docs/en/knowledgebase/advanced/consensus) on the state of the
    network. Substrate makes it possible to supply custom consensus engines and also ships with
    several consensus mechanisms that have been built on top of
    [Web3 Foundation research](https://research.web3.foundation/en/latest/polkadot/NPoS/index.html).
-   RPC Server: A remote procedure call (RPC) server is used to interact with Substrate nodes.

There are several files in the `node` directory - take special note of the following:

-   [`chain_spec.rs`](./node/src/chain_spec.rs): A
    [chain specification](https://substrate.dev/docs/en/knowledgebase/integrate/chain-spec) is a
    source code file that defines a Substrate chain's initial (genesis) state. Chain specifications
    are useful for development and testing, and critical when architecting the launch of a
    production chain. Take note of the `development_config` and `testnet_genesis` functions, which
    are used to define the genesis state for the local development chain configuration. These
    functions identify some
    [well-known accounts](https://substrate.dev/docs/en/knowledgebase/integrate/subkey#well-known-keys)
    and use them to configure the blockchain's initial state.
-   [`service.rs`](./node/src/service.rs): This file defines the node implementation. Take note of
    the libraries that this file imports and the names of the functions it invokes. In particular,
    there are references to consensus-related topics, such as the
    [longest chain rule](https://substrate.dev/docs/en/knowledgebase/advanced/consensus#longest-chain-rule),
    the [Aura](https://substrate.dev/docs/en/knowledgebase/advanced/consensus#aura) block authoring
    mechanism and the
    [GRANDPA](https://substrate.dev/docs/en/knowledgebase/advanced/consensus#grandpa) finality
    gadget.

After the node has been [built](#build), refer to the embedded documentation to learn more about the
capabilities and configuration parameters that it exposes:

```shell
./target/release/node-template --help
```

### Runtime

In Substrate, the terms
"[runtime](https://substrate.dev/docs/en/knowledgebase/getting-started/glossary#runtime)" and
"[state transition function](https://substrate.dev/docs/en/knowledgebase/getting-started/glossary#stf-state-transition-function)"
are analogous - they refer to the core logic of the blockchain that is responsible for validating
blocks and executing the state changes they define. The Substrate project in this repository uses
the [FRAME](https://substrate.dev/docs/en/knowledgebase/runtime/frame) framework to construct a
blockchain runtime. FRAME allows runtime developers to declare domain-specific logic in modules
called "pallets". At the heart of FRAME is a helpful
[macro language](https://substrate.dev/docs/en/knowledgebase/runtime/macros) that makes it easy to
create pallets and flexibly compose them to create blockchains that can address
[a variety of needs](https://www.substrate.io/substrate-users/).

Review the [FRAME runtime implementation](./runtime/src/lib.rs) included in this template and note
the following:

-   This file configures several pallets to include in the runtime. Each pallet configuration is
    defined by a code block that begins with `impl $PALLET_NAME::Trait for Runtime`.
-   The pallets are composed into a single runtime by way of the
    [`construct_runtime!`](https://crates.parity.io/frame_support/macro.construct_runtime.html)
    macro, which is part of the core
    [FRAME Support](https://substrate.dev/docs/en/knowledgebase/runtime/frame#support-library)
    library.

### Pallets

The runtime in this project is constructed using many FRAME pallets that ship with the
[core Substrate repository](https://github.com/paritytech/substrate/tree/master/frame) and a
template pallet that is [defined in the `pallets`](./pallets/template/src/lib.rs) directory.

A FRAME pallet is compromised of a number of blockchain primitives:

-   Storage: FRAME defines a rich set of powerful
    [storage abstractions](https://substrate.dev/docs/en/knowledgebase/runtime/storage) that makes
    it easy to use Substrate's efficient key-value database to manage the evolving state of a
    blockchain.
-   Dispatchables: FRAME pallets define special types of functions that can be invoked (dispatched)
    from outside of the runtime in order to update its state.
-   Events: Substrate uses [events](https://substrate.dev/docs/en/knowledgebase/runtime/events) to
    notify users of important changes in the runtime.
-   Errors: When a dispatchable fails, it returns an error.
-   Trait: The `Trait` configuration interface is used to define the types and parameters upon which
    a FRAME pallet depends.

### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
./scripts/docker_run.sh
```

This command will firstly compile your code, and then start a local development network. You can
also replace the default command (`cargo build --release && ./target/release/node-template --dev --ws-external`)
by appending your own. A few useful ones are as follow.

```bash
# Run Substrate node without re-compiling
./scripts/docker_run.sh ./target/release/node-template --dev --ws-external

# Purge the local dev chain
./scripts/docker_run.sh ./target/release/node-template purge-chain --dev

# Check whether the code is compilable
./scripts/docker_run.sh cargo check
```

