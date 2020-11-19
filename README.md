# Governance on Substrate
:warning: This is work in progress, there's still a bunch of stuff to be added. Feel free to contribute :smiley:

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

## 1. Breaking Things Down

We'll use how Polkadot implements governance as a reference to our guide and see (a) how its governance pallets interact with eachother and (b) how their parameters are configured. Polkadot's governance system is composed of three main elements:

1. _Stake weighted referenda_ - public motions to pass for voting by council 
2. _A treasury_ - a reserve made up of DOT tokens from slashing or sub-optimal staking 
3. _A Council_ - made up of two groups (standard committee and technical committee)

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

## What is governance used for in Polkadot and Kusama?
- To modify parameters of the system like voting periods and cool-off periods
- Deciding on runtime code updates
- Specify how parachains interact

For our purposes, here are the pallets we'll look at:

```Council``` - https://docs.rs/pallet-collective/2.0.0/pallet_collective/ 

```Treasury``` - https://docs.rs/pallet-treasury/2.0.0/pallet_treasury/ 

```ElectionsPhragmen``` -  https://docs.rs/pallet-elections-phragmen/2.0.0/pallet_elections_phragmen/

## 2. Pallet Interactions
### Council 
:family: Polkadot uses what's referred to as a CouncilCollective for their voting members or committees. The way ``CouncilCollective`` is created is by instantiating ``pallet_collective`` in runtime/lib.rs. It's traits define what council rules are enforced and how.

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
	type WeightInfo = ();
}
```
This pallet is key for any setup where governance involves more than one decision maker (i.e. other than using Sudo related dispatches).

### Treasury
:moneybag: The Treasury module provides a "pot" of funds that can be managed by stakeholders in the system and a structure for making spending proposals from this pot.

- treasury::Trait
- Call

[See source code](https://github.com/paritytech/substrate/tree/master/frame/treasury) 

Treasury is a core component to goverance too. In Polkadot, it's used by a wide array of other pallets such as ``pallet_staking``, ``pallet_identity``, ``pallet_democracy`` and ``pallet_elections_phragmen``. With incentive driven behavior as a key design of any decentralized system, a pot of funds is a useful tool for setting up decision making mechanisms. Systems of governance can be formally specified based on what is at stake and what happens to staked assets once a decision is passed. In Polkadot and Kusama, the Treasury collects funds from slashing stakes and non-optimal staking during consensus. The accumulation of these funds are used by the council to invest in and improve the network and ecosystem by [sponsoring research and awareness] (https://polkadot.network/writing-history-the-first-teams-submit-their-proposal-to-the-polkadot-treasury-2/).

Below shows how ``pallet_treasury `` is implemented in Polkadot. Notice ``ApproveOrigin``: this is where the approval must come from &mdash; ``pallet_collective`` in Polkadot's case, which has already been defined as ``CouncilCollective``. ( :thinking: _TODO: Show examples for implementing different sources of origin + why its important_)

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
:inbox_tray: In a Substrate-based system of governance, including an Elections pallet implicitly specifies how the ``Treasury`` and ``Collective`` pallets interact. Election modules central to staking mechanims as well as voting on referenda. There are two Elections pallets in Substrate: one more simple (Elections) and one more sophisticated (Elections Phragmen).

Polkadot implements the [Elections Phragmen pallet](https://crates.parity.io/pallet_elections_phragmen/trait.Trait.html#associatedtype.CurrencyToVote) to do its governance magic. It's an implementation of Elections with an algorithm to allow a more expressive way to represent voter views. It's configured to have weekly council elections with 13 initial members. 

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

:mag_right: Notice how in this example, ``Elections`` interacts with both ``Treasury`` and ``Council``:
- ``ChangeMembers`` defines what to do when members change, which relies on ``Council``
- ``InitializeMembers`` defines what to do with genesis members, which relies on ``Council``
- ``CandidacyBond`` defines how much should  be locked up in order to submit one's candicacy
- ``VotingBond`` defines how much should be locked up in order to be able to submit votes, which requires ``Treasury``
- ``LoserCandidate``, ``BadReport`` and ``KickedMember`` are various handlers for different unbalanced reduction scenarios which each require ``Treasury`` 


_TODO: Difference between the two Elections pallets? Module import issues? Add examples of different configurations_

#### Modifications 

We need to make a few modifications in runtime/lib to have these pallets work in our codebase. These are:

- add ``ModuleId`` and ``Percent`` to ``use sp_runtime::{ }``
- Adding the ``EnsureOneOf`` struct (``use frame_system::{EnsureRoot, EnsureOneOf}``) to give runtime options for authorizing certain properties of the nodes that it can use
- ``EnsureOneOf`` will be required to implement the Council pallet. (:warning: _TODO: A little about Origins and configuring them_ )

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

## 3. Tailoring Governance
Now that we've seen how governance can be configured, let's dive into how different forms of governance can be implemented to address specific goals of a given system. As [Bill Laboon](https://www.youtube.com/watch?v=9B10wX9Mphc) puts it, there will always be a tradeoff when implementing a system of governance &mdash the only alternative would be to appoint a dictator. 

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
- Is there flexibility with locked stake?
- How many proposals can there be in a proposal queue / what's the voting timetable?
- What happens to funds in a treasury?

Other things to consider:
* Who are your stakeholders? For example in Polkadot we have:
	- Node operators
	- Long term hodlers
	- Bonded validators
	- Parachain operators 
	- Dapp teams
	- Client implementers 
	
### Post-genesis Governance
Here are some examples of additional governance mechanisms planned to be added to Kusama:
- **Oracle Committee**: memembers paid to vote on objectively true or false statements
- **Spontaneous Subject Committees**: specialized groups can register to vote on very specialized referenda 



_TODO: Add more cross references; Add examples of different configurations and their functionality_

Sources:
https://wiki.polkadot.network/docs/en/learn-governance

https://github.com/paritytech/polkadot/wiki/Governance

https://polkadot.network/kusama-rollout-and-governance/

