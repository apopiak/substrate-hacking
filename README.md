# A Customized Substrate Node that does Stuff

This Substrate node was built following two tutorials from the [Substrate Developer Hub](https://substrate.dev/):

1) [Perform a Forkless Upgrade](https://substrate.dev/docs/en/tutorials/upgrade-a-chain/sudo-upgrade)
2) [Build a Permissioned Network](https://substrate.dev/docs/en/tutorials/build-permission-network/)

:rocket: It's meant to help beginners get a taste for how easy it is to add functionality to a Substrate-built blockchain in a modular way. This is originally a fork from the Substrate Template Node. Please follow Substrate's tutorial [here](https://substrate.dev/docs/en/tutorials/create-your-first-substrate-chain/) if this is your first time working with Substrate.

:bulb: :goal_net: The goal is add new functionality to this permissioned chain :goal_net: :bulb:

:factory: See the runtime modules folder for a history of the changes in this chains runtime _(this is actually useless, only here to play around with forkless upgrades)_

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
```
:tv: Now go to https://polkadot.js.org/apps/ to see what's happening live! This webapp developed by Polkadot allows you to connect your local node by selecting a custom endpoint - make sure it's connected to `127.0.0.1:9944`. While you're there, go to the _Settings_ &rarr; _Developer_ page:
add the following in the:
```bash
{
  "PeerId": "(Vec<u8>)"
}
```
```bash 
// And finally Charlie's
./target/release/node-template --chain=local --base-path ~/tmp/validator3 --name charlie  --node-key=3a9d5b35b9fb4c42aafadeca046f6bf56107bd2579687f069b42646684b94d9e --port 30335 --ws-port=9946 --offchain-worker always
```
To finish adding Charlie to the network, in the apps UI, go to **Developer -> Sudo** and submit the `nodeAuthorization` -> `add_well_known_node` call with the peer id in hex of Charlie's node `002408011220876a7b4984f98006dc8d666e28b60de307309835d775e7755cc770328cdacf2e` and the owner being Charlie.

Head over to _Chainstate_ &rarr; _Storage_ and select `"nodeAuthorization"` and the `"wellKnownNodes()"` function. Hit the `(+)` button and this will allow you to see the well known nodes, Alice, Bob and Charlie.

:vertical_traffic_light: **NOTE:** refresh the page if it's not displaying anything.

See how to add more connections by following the original tutorial. For the purpose of this codebase, we've done what we need and have our permissioned network up and running. Now, let's add in an upgrade right from the UI.

### Adding Extrinsics from UI

The key runtime modules to achieve forkless upgrades in our usage are: Sudo and Schedular. The Sudo pallet is already a part of the FRAME-based node template we're using. The Schedular pallet was added as per [the tutorial referred to above](https://substrate.dev/docs/en/tutorials/upgrade-a-chain/).

Using the *Extrinsic* from the Sudo pallet, you can experiment with adding any of the runtime updates in the form of WASM binaries to this permissioned network. Please refer to the folder: WASM-runtimes.

## Using the FRAME-based Multisig Pallet in our Permissioned Network

:vertical_traffic_light: **NOTE:** if you want to test a forkless upgrade using a WASM binary, the updated runtime for using the Multisig pallet can be found in this projects *'WASM-runtimes'* folder

Here's the code that has been added to include the multisig pallet to our runtime:

```bash
// --snip--
// Configure the runtime's implementation of the Multisig pallet in runtime/src/lib.rs
parameter_types! {
	// One storage item
	pub const DepositBase: Balance = 100;
	// Additional storage item 
	pub const DepositFactor: Balance = 10;
	pub const MaxSignatories: u16 = 100;
}

impl pallet_multisig::Trait for Runtime {
	type Event = Event;
	type Call = Call;
	type Currency = Balances;
	type DepositBase = DepositBase;
	type DepositFactor = DepositFactor;
	type MaxSignatories = MaxSignatories;
	type WeightInfo = ();
}
// --snip--
//And add the following to the construct_runtime! macro
// --snip--
`Multisig: pallet_multisig::{Module, Call, Storage, Event<T>},` 
// --snip--
```
Always check that things resolve correctly when running new dependencies:
```bash
cargo check -p node-template-runtime
```
:rocket:

## Multisig Forkless Runtime Upgrade
1. :construction_worker: Reassign your Sudo key to any multisig address you created (**Go to: _Developer_ $rar; _Sudo_ &rarr; _Set sudo key_**)
2. :warning: Follow the steps in [this tutorial](https://substrate.dev/docs/en/tutorials/upgrade-a-chain/scheduled-upgrade) to schedule an upgrade. 
3. :memo: You'll need to have the minimum threshold of signators sign the scheduled upgrade for it to go through
4. :eyes: If it worked, your version number should update itslef once your chain reaches the scheduled update block :sunglasses:

```
//TODO: Implement different governance instead of Sudo 

//TODO: Implement a type of multisig account that receives runtime update for members to vote on before it //gets included to runtime
```
