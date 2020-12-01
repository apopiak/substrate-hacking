:warning: This is work in progress, there's still a bunch of stuff to be added. Feel free to contribute :smiley:

Check out [this document](https://docs.google.com/document/d/1Y61Fd_uZE3y1jeC_4EYe-ZOqaIfI1OAZYe9Xh75a8cs/edit?usp=sharing) for a list of ressources to learn more about Substrate and Polkadot Governance stuff.

## Table of contents
1. [Overview](#1-overview)
2. [Breaking Things Down](#2-breaking-things-down)
   * [What is governance used for in Polkadot?](#what-is-governance-used-for-in-polkadot) 
3. [Pallet Interactions](#3-pallet-interactions)
   * [Council](#council)
   * [Treasury](#treasury)
   * [Elections](#elections)
4. [Modifications](#4-modifications)
5. [Tailoring Governance](#5-tailoring-governance)
   * [Setting Goals and Parameters](#Setting-Goals-and-Parameters) 
   * [Demo](#governance-demo)
6. [Other](#6-Other)

# 1. Overview
The goal here is to document how different components for governance can interact with eachother within Substrate. There's a huge opportunity to use tools for adding governance to Substrate-built systems and understanding how they work and how they can be configured is an important building block for creating systems with solid foundations to evolve. :hatching_chick:

:bulb: Some use cases for thinking of governance as a core component to a given system:

- Collective management layers 
- Social interaction layers 
- Device to device communication or multi-agent systems
- Content up-voting 

:memo: The structure of this document is:
1. :hammer: First breakdown the key components for implementing governance with Substrate 
2. :mag_right: Analyzse how these pallets interact inside Polkadots runtime and implement into this Substrate template fork _(IN PROGRESS)_
3. Show examples of how to customize governance based on additional pallet configurations _(TODO)_

## 2. Breaking Things Down

We'll use how Polkadot implements governance as a reference to our guide and see (a) how its governance pallets interact with eachother and (b) how their parameters are configured. Polkadot's governance system is composed of three main elements:

1. _Stake weighted referenda_ - public motions to pass for voting by council 
2. _A treasury_ - a reserve made up of DOT tokens from slashing or sub-optimal staking 
3. _A Council_ - made of collectives, of which Polkadot currently has two (the "standard" council and the "technical committee")

### What is governance used for in Polkadot? 
:crown:
- To modify parameters of the system like voting periods and cool-off periods
- Deciding on runtime code updates
- To specify how parachains interact

Although we won't look at all of these (for now), here are the difference parts to its Governance machine:

```bash
// Governance stuff.
		Democracy: pallet_democracy::{Module, Call, Storage, Config, Event<T>} = 14,
		Council: pallet_collective::<Instance1>::{Module, Call, Storage, Origin<T>, Event<T>, Config<T>} = 15,
		TechnicalCommittee: pallet_collective::<Instance2>::{Module, Call, Storage, Origin<T>, Event<T>, Config<T>} = 16,
		ElectionsPhragmen: pallet_elections_phragmen::{Module, Call, Storage, Event<T>, Config<T>} = 17,
		TechnicalMembership: pallet_membership::<Instance1>::{Module, Call, Storage, Event<T>, Config<T>} = 18,
		Treasury: pallet_treasury::{Module, Call, Storage, Event<T>} = 19,
```

For our purposes, here are the pallets we'll look at:

```Council``` - https://docs.rs/pallet-collective/2.0.0/pallet_collective/ 

```Treasury``` - https://docs.rs/pallet-treasury/2.0.0/pallet_treasury/ 

```ElectionsPhragmen``` -  https://docs.rs/pallet-elections-phragmen/2.0.0/pallet_elections_phragmen/

## 3. Pallet Interactions
### Council 
:family: Polkadot uses what's referred to as a CouncilCollective for their voting members or committees. The way ``CouncilCollective`` and ``TechnicalCollective`` are created is by instantiating ``pallet_collective`` in runtime/lib.rs. Each make use of different instances of the pallet, by way of its structs. For both collectives, the traits define what the council and committee rules are and how they are enforced.

```bash
parameter_types! {
	pub const CouncilMotionDuration: BlockNumber = 7 * DAYS;
	pub const CouncilMaxProposals: u32 = 100;
	pub const CouncilMaxMembers: u32 = 100;
}

type CouncilCollective = pallet_collective::Instance1;

impl pallet_collective::Trait<CouncilCollective> for Runtime {
	type Origin = Origin;
	type Proposal = Call;
	type Event = Event;
	type MotionDuration = CouncilMotionDuration;
	type MaxProposals = CouncilMaxProposals;
	type MaxMembers = CouncilMaxMembers;
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type WeightInfo = weights::pallet_collective::WeightInfo<Runtime>;;  
}

parameter_types! {
	pub const TechnicalMotionDuration: BlockNumber = 7 * DAYS;
	pub const TechnicalMaxProposals: u32 = 100;
	pub const TechnicalMaxMembers: u32 = 100;
}

type TechnicalCollective = pallet_collective::Instance2;

impl pallet_collective::Trait<TechnicalCollective> for Runtime {
	type Origin = Origin;
	type Proposal = Call;
	type Event = Event;
	type MotionDuration = TechnicalMotionDuration;
	type MaxProposals = TechnicalMaxProposals;
	type MaxMembers = TechnicalMaxMembers;
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type WeightInfo = weights::pallet_collective::WeightInfo<Runtime>;
}
```

The ``pallet_collective`` pallet is key for any setup where governance involves more than one group of decision makers. Other things to note:

- Different collectives are instantiated using the structs (Instance1, Instance2 etc.) from ``pallet_collective``
- Each collective can be configured with their own parameters

*TODO: Add where these collectives are used in other pallets like Members and Democracy; how pallet_membership makes use of pallet_collective and why*

### Treasury
:moneybag: The Treasury module provides a "pot" of funds that can be managed by stakeholders in the system and a structure for making spending proposals from this pot.

It can be used for: 
- collecting funds 
- setting bounties 
- tipping an address (or group of addresses) 

[See source code](https://github.com/paritytech/substrate/tree/master/frame/treasury) 

Treasury is a core component to goverance too. In Polkadot, it's used by a wide array of other pallets such as ``pallet_staking``, ``pallet_identity``, ``pallet_democracy`` and ``pallet_elections_phragmen``. With incentive driven behavior as a key design of any decentralized system, a pot of funds is a useful tool for storing and making use of value that can motivate benevolant activity. Treasuries can have their own systems of governance attached to them, governing who has access to funds and under what conditions. By specifying different membership sets (using [``pallet_membership``](https://crates.parity.io/pallet_membership/index.html)), a system can be designed to have a treasury for each membership group, like treasuries across different municipalities for example.

In Polkadot, there is a single Treasury configured to collect funds from slashing stakes and non-optimal staking during consensus. The accumulation of these funds are used by the council to invest in and improve the network and ecosystem by [sponsoring research and awareness](https://polkadot.network/writing-history-the-first-teams-submit-their-proposal-to-the-polkadot-treasury-2/).

Below shows how ``pallet_treasury `` is implemented in Polkadot. Notice ``ApproveOrigin``, which is where the approval must come from &#151; ``CouncilCollective`` in Polkadot's case.

 :thinking: _TODO: Show examples for implementing different sources of origin + why its important_

```bash
type ApproveOrigin = EnsureOneOf<
	AccountId,
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<_3, _5, AccountId, CouncilCollective>
>;

impl pallet_treasury::Trait for Runtime {
	type ModuleId = TreasuryModuleId;
	type Currency = Balances;
	type ApproveOrigin = ApproveOrigin;
	type RejectOrigin = MoreThanHalfCouncil;
	type Tippers = ElectionsPhragmen;
	type TipCountdown = TipCountdown;
	type TipFindersFee = TipFindersFee;
	type TipReportDepositBase = TipReportDepositBase;
	type DataDepositPerByte = DataDepositPerByte;
	type Event = Event;
	type OnSlash = Treasury;
	type ProposalBond = ProposalBond;
	type ProposalBondMinimum = ProposalBondMinimum;
	type SpendPeriod = SpendPeriod;
	type Burn = Burn;
	type BountyDepositBase = BountyDepositBase;
	type BountyDepositPayoutDelay = BountyDepositPayoutDelay;
	type BountyUpdatePeriod = BountyUpdatePeriod;
	type MaximumReasonLength = MaximumReasonLength;
	type BountyCuratorDeposit = BountyCuratorDeposit;
	type BountyValueMinimum = BountyValueMinimum;
	type BurnDestination = ();
	type WeightInfo = weights::pallet_treasury::WeightInfo<Runtime>;
}
```

### Elections
:inbox_tray: In a Substrate-based system of governance, the Elections-Phragmén pallet interacts closely with the ``Collective`` pallet. In Polkadot, its what facilitates electing members of council. It can have other useful applications too, like community voting on NFT assets or electing other types of content.

_TODO: Include an example showing the Elections Phragmén pallet being used in a non-political context_

Polkadot implements the [Elections-Phragmén pallet](https://crates.parity.io/pallet_elections_phragmen/trait.Trait.html#associatedtype.CurrencyToVote) to do its governance magic. It's an implementation of [Elections](https://crates.parity.io/pallet_elections/index.html) (now depreciated) with an algorithm to allow a more expressive way to represent voter views. 

The algorithm basically fills up a specified number of seats in several election rounds by allowing voters to vote for multiple candidates. The candidate with the most amount of votes gets the seat for each round, while remaining candidates carry their past votes with them in the next round. Learn more about how it works [here](https://wiki.polkadot.network/docs/en/learn-phragmen).

In Polkadot, it's configured to have weekly council elections with 13 initial members (also the number of seats).

```bash
parameter_types! {
	pub const CandidacyBond: Balance = 100 * DOLLARS;
	pub const VotingBond: Balance = 5 * DOLLARS;
	/// Weekly council elections; scaling up to monthly eventually.
	pub const TermDuration: BlockNumber = 7 * DAYS;
	/// 13 members initially, to be increased to 23 eventually.
	pub const DesiredMembers: u32 = 13;
	pub const DesiredRunnersUp: u32 = 20;
	pub const ElectionsPhragmenModuleId: LockIdentifier = *b"phrelect";
}
// Make sure that there are no more than `MaxMembers` members elected via phragmen.
const_assert!(DesiredMembers::get() <= CouncilMaxMembers::get());

impl pallet_elections_phragmen::Trait for Runtime {
	type Event = Event;
	type ModuleId = ElectionsPhragmenModuleId;
	type Currency = Balances;
	type ChangeMembers = Council;
	type InitializeMembers = Council;
	type CurrencyToVote = frame_support::traits::U128CurrencyToVote;
	type CandidacyBond = CandidacyBond;
	type VotingBond = VotingBond;
	type LoserCandidate = Treasury;
	type BadReport = Treasury;
	type KickedMember = Treasury;
	type DesiredMembers = DesiredMembers;
	type DesiredRunnersUp = DesiredRunnersUp;
	type TermDuration = TermDuration;
	type WeightInfo = ();
}
```

:mag_right: Notice how in this example, ``Elections-Phragmén`` interacts with both ``Treasury`` and ``Council``:
- ``ChangeMembers`` defines what to do when members change, which relies on ``Council``
- ``InitializeMembers`` defines what to do with genesis members, which relies on ``Council``
- ``CandidacyBond`` defines how much should  be locked up in order to submit one's candicacy
- ``VotingBond`` defines how much should be locked up in order to be able to submit votes, which requires ``Treasury``
- ``LoserCandidate``, ``BadReport`` and ``KickedMember`` are handlers for different unbalanced reduction scenarios which interact with ``Treasury`` to handle an address getting slashed


_TODO: Add examples of different configurations for Elections; how Elections is used in Democracy; how Elections works in pallet_treasury::Tippers _

## 4. Modifications 

We need to make a few modifications in runtime/lib to have these pallets work in our codebase. These are:

- in ``lib.rs``:
	- add ``ModuleId`` and ``Percent`` to ``use sp_runtime::{ }`` and ``u32_trait::{_1, _2, _3, _4, _5}`` to ``use sp_core::{ }``
	- add ``use pallet_collective::EnsureProportionAtLeast`` 
- in ``chain_spec`` add ``Balance`` to ``use node_runtime::{ }``

- Adding the ``EnsureOneOf`` struct (``use frame_system::{EnsureRoot, EnsureOneOf}``) to give runtime options for authorizing certain properties of the nodes that it can use
- ``EnsureOneOf`` will be required to implement the Council pallet. 

e.g.:
			```bash 
			type ApproveOrigin = EnsureOneOf<
				AccountId,
				EnsureRoot<AccountId>,
				pallet_collective::EnsureProportionAtLeast<_3, _5, AccountId, CouncilCollective>
			>;

			type MoreThanHalfCouncil = EnsureOneOf<
				AccountId,
				EnsureRoot<AccountId>,
				pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, CouncilCollective>
			>;
			```

- The ``elections_phragmen`` pallet requires a ``LockIdentifier`` type from a trait in ``frame_support``. Refer to its documentation [here](https://docs.rs/frame-support/2.0.0/frame_support/traits/type.LockIdentifier.html).

- In node/src/chain_spec.rs, add the ``GenesisConfig { }`` for each pallet. Look at each pallets' documentation for reference: [Collective](https://docs.rs/pallet-collective/2.0.0/pallet_collective/struct.GenesisConfig.html) and [Elections](https://docs.rs/pallet-elections-phragmen/2.0.0/pallet_elections_phragmen/struct.GenesisConfig.html).

(:warning: _TODO: Add a little about Origins and configuring them_ )


## 5. Tailoring Governance
Now that we've seen how governance can be configured, let's dive into how different forms of governance can be implemented to address specific goals of a given system. As [Bill Laboon](https://www.youtube.com/watch?v=9B10wX9Mphc) puts it, there will always be a tradeoff when implementing a system of governance &#151; the only alternative would be to appoint a dictator. 

### Setting Goals and Parameters
Step 0 is  to outline what governance goals need to be set. For example, in Polkadot the existance of the Technical Committee addresses the goal that there needs to be a way for fast-tracking sytsem-critical issues when they arise. When designing an infrastructure for blockchain governance, goals need to be aligned with the possibility of things going terribly wrong.

:thinking: A few things that could go wrong...
- Sybil attacks 
- Stealing funds 
- 51% malicious votes

:bulb: Parameters to consider:
- Who can vote? What power does each vote hold?
- How long is the voting period? 
- How long is the enactment period?
- How long is stake locked up for?
- What are the sanctions for bad actors?
- What percentage of stake approves a vote?
- What sort of flexibility exists for locked stake?
- How many proposals can there be in a proposal queue?
- What's the voting timetable?
- What happens to funds in a treasury?

:8ball: Other things to consider:
* Who are your stakeholders? For example in Polkadot we have:
	- Node operators
	- Long term hodlers
	- Bonded validators
	- Parachain operators 
	- Dapp teams
	- Client implementers 

### Governance demo
The original goal was to explore how to add governance capabilities to the "permissioned-net" branch of this repo. The idea was that the permissioned network can be initialized with Council members (as the first 3 nodes) and have any additional node propose Treasury spends for the Council to vote on. Multi-sig accounts can be used for where funds are sent to.

In practice, it can be thought of a rudimentary bicameral legislative body for budget proposals in the following types of scenarios:
- :clubs: A sports club, where members can propose activities and there must be majority of directors that vote in favor for a proposal to be exectuted;
- :mortar_board: A student council, where budget spends are proposed by special accounts (e.g. Events, Community Engagement etc.) and exected by vote of the Council
- :calendar: An event management layer, for an executive committee and their event teams to coordinate budgets and engagement 

Compile:

```bash
WASM_BUILD_TOOLCHAIN=nightly-2020-10-05 cargo build --release
```
To simply run this on a single node in developer mode do:

```bash
./target/release/node-template --dev --tmp
```

Run Alice's node in one terminal:

```bash
// Start with Alice's node 
./target/release/node-template --chain=local --base-path ~/tmp/validator1 --alice --node-key=c12b6d18942f5ee8528c8e2baf4e147b5c5c18710926ea492d09cbd9f6c9f82a --port 30333 --ws-port 9944
```

And Bob's in another:

```bash
// Now with Bob's node 
./target/release/node-template --chain=local --base-path ~/tmp/validator2 --bob --node-key=6ce3be907dbcabf20a9a5a60a712b4256a54196000a8ed4050d352bc113f8c58 --port 30334 --ws-port 9945
```

Make sure you've added your additional types for ``PeerId``s:
```bash
{
  "PeerId": "(Vec<u8>)"
}
```

_Still WIP ..._ 

## 6. Other
### Post-genesis Governance
Here are some examples of additional governance mechanisms planned to be added to Kusama:
- **Oracle Committee**: memembers paid to vote on objectively true or false statements
- **Spontaneous Subject Committees**: specialized groups can register to vote on very specialized referenda 


_TODO: Add more cross references; Add examples of different configurations and their functionality_

A quote from a Polkadot core developer and council member: _"[Wei] believes that voting is only the last step of democracy. Detailed discussions and community engagements are essential before voting, in order to better understand the proposals, look into any potential issues, and avoid unnecessary contentions."_  [See reference](https://that.world/~wei/polkadot/council/)

Sources:

https://wiki.polkadot.network/docs/en/learn-governance

https://github.com/paritytech/polkadot/wiki/Governance

https://polkadot.network/kusama-rollout-and-governance/