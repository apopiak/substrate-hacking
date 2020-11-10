# A Customized Substrate Node that does Stuff

This Substrate node was built following two tutorials from the [Substrate Developer Hub](https://substrate.dev/):

1) [Perform a Forkless Upgrade](https://substrate.dev/docs/en/tutorials/upgrade-a-chain/sudo-upgrade)
2) [Build a Permissioned Network](https://substrate.dev/docs/en/tutorials/build-permission-network/)

:rocket: It's meant to help beginners get a taste for how easy it is to add functionality to a Substrate-built blockchain in a modular way. This is originally a fork from the Substrate Template Node. Please follow Substrate's tutorial [here](https://substrate.dev/docs/en/tutorials/create-your-first-substrate-chain/) if this is your first time working with Substrate.

:bulb: :goal_net: The goal is add new functionality to this permissioned chain :goal_net: :bulb:

This guide will walk you through how this chains runtime was built and how to interact with it using [Polkadots Block Explorer UI](https://polkadot.js.org/apps/).

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
:tv: Now head to https://polkadot.js.org/apps/ to see what's happening live! This webapp developed by Polkadot allows you to connect to a local node by selecting a custom endpoint &mdash; make sure it's connected to `127.0.0.1:9944`. While you're there, go to the _Settings_ &rarr; _Developer_ page and add the following:
```bash
{
  "PeerId": "(Vec<u8>)"
}
```
To add Charlie to the network run the following in a separate terminal:

```bash 
// And finally Charlie's
./target/release/node-template --chain=local --base-path ~/tmp/validator3 --name charlie  --node-key=3a9d5b35b9fb4c42aafadeca046f6bf56107bd2579687f069b42646684b94d9e --port 30335 --ws-port=9946 --offchain-worker always
```
Then, head to the apps UI and go to **_Developer_** &rarr; **_Sudo_** and submit the `nodeAuthorization` &rarr; `add_well_known_node` call with the Peer ID of Charlie's node: `002408011220876a7b4984f98006dc8d666e28b60de307309835d775e7755cc770328cdacf2e` and Charlie as owner.

Head over to **_Chainstate_** &rarr; **_Storage_** and select `nodeAuthorization` and the `wellKnownNodes()` function. Hit the `(+)` button and this will allow you to see the well known nodes, Alice, Bob and Charlie.

:vertical_traffic_light: **NOTE:** refresh the page if it's not displaying any changes.

See how to add more connections by following the original tutorial. For the purpose of this codebase, we've done what we need and have our permissioned network up and running. 

## Adding Scheduler and Multisig Pallets to our Runtime

The Scheduler pallet was added as per [the tutorial referred to above](https://substrate.dev/docs/en/tutorials/upgrade-a-chain/). Instead of using Sudo to validate the scheduled runtime upgrade, we're going to use the multisig functionality. 

### Using the FRAME-based Multisig Pallet in our Permissioned Network

:vertical_traffic_light: **NOTE:** if you're following this guide in your own node template and want to test a runtime upgrade, you can use one of the upgraded runtimes in this branches *'WASM-runtimes'* folder.

Like with building any runtime, we have to first go into `runtime/src/lib.rs` and add the pallet we want to use.

Here's the code that has been added to include the Multisig pallet to our runtime:

```bash
// --snip--
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
	
//And add the following to the construct_runtime! macro
	// --snip--
`Multisig: pallet_multisig::{Module, Call, Storage, Event<T>},` 
	// --snip--
```
Following this, we have to tell our runtime about any dependencies it must know about in the `/runtime/Cargo.toml` file:

```
[dependencies]
	// --snip--
pallet-multisig = { default-features = false, version = '2.0.0'}
	// --snip--
std = [
	// --snip--
'pallet-multisig/std',
]
```
Always check that things resolve correctly:
```bash
cargo check -p node-template-runtime
```
:rocket: Now we can try a scheduled multisig chain upgrade!

## Multisig Runtime Upgrade
1. :construction_worker: Reassign your Sudo key to any multisig address you created (Go to: **_Developer_ &rarr; _Sudo_ &rarr; _Set sudo key_**)
![Setmultisig](/screenshots/set-sudo-to-multisig.png)

2. :warning: Follow the steps in [this tutorial](https://substrate.dev/docs/en/tutorials/upgrade-a-chain/scheduled-upgrade) to schedule an upgrade. 

As the screenshots show below, we're aiming to schedule an upgrade from version 1 to 2 (using the WASM file provided in this repo) at block 150 :clock3:
![ScheduleUpgrade](/screenshots/schedule-upgrade-0.png)

Alice is the first to approve it 
![AliceSigns](/screenshots/schedule-updgrade-1.png)

Charlie approves it
![CharlieSigns](/screenshots/schedule-upgrade-2.png)

3. :memo: You'll need to have the minimum threshold of signators sign the scheduled upgrade for it to go through

Great, the transaction is approved (the multisig account's threshold was 2/3) :muscle: &mdash; we can see it in the event calendar :calendar:
![EventCal](/screenshots/calandar-view.png)

4. :eyes: If it worked, your version number should update itslef once your chain reaches the scheduled update block :sunglasses:

![EventsPostUpgrade](/screenshots/events-explorer.png)

```
//TODO: Implement governance for validating runtime upgrade
//TODO: Remove Sudo entirely and make multisig default
//TODO: Make UI for nodes to interact with peers and for new nodes to join
```
