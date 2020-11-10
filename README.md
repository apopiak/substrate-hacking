# Governance on Substrate

The goal here is to document how different components for governance can interact with eachother within Substrate. There's a huge opportunity to use tools for adding governance to Substrate-built systems and understanding how they work and how they can be configured is an important building block for building systems with solid foundations. :hatching_chick:

:bulb: Some use cases for using governance as a core component:

- Collective management layers 
- Social interaction layers 
- Device to device communication 

:memo: The structure of this document is:
1. :hammer: First breakdown the key components for implementing governance with Substrate 
2. :mag_right: Analyzse how these pallets interact inside Polkadots runtime and implement into this Substrate template fork _(IN PROGRESS)_
3. Show examples of how to customize governance based on additional pallet configurations _(TODO)_

## 1. Breaking Things Down

We'll use how Polkadot implements governance as a reference to our guide. Although we won't look at all of these, here are the difference parts to its Governance machine:

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

Treasury is a core component to goverance too. In Polkadot, it's used by a wide array of other pallets such as ``pallet_staking``, ``pallet_identity``, ``pallet_democracy`` and ``pallet_elections_phragmen``. With incentive driven behavior as a key design of any decentralized system, a pot of funds is a useful tool for setting up decision making mechanisms. Systems of governance can be formally specified based on what is at stake and what happens to staked assets once a decision is passed. In this way, Treasury can be linked to (re)distributing funds and reputation as well as enforcing consequences for decisions that have been voted upon.

Below shows how ``pallet_treasury `` is implemented in Polkadot. Notice ``ApproveOrigin``: this is where the approval must come from &mdash; ``pallet_collective`` in Polkadot's case, which has already been defined as ``CouncilCollective``. ( :curious: _TODO: What could be alternative sources for origin if any?_)

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
:inbox_tray: In a Substrate-based system of governance, including an Elections pallet allows the system to specify how the ``Treasury`` and ``Collective`` pallets interact. There are two Elections pallets in Substrate: one more simple (Elections) and one more sophisticated (Elections Phragmen).

Polkadot implements the [Elections Phragmen pallet](https://crates.parity.io/pallet_elections_phragmen/trait.Trait.html#associatedtype.CurrencyToVote) to do its governance magic. It's configured to have weekly council elections with 13 initial members.

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

## 3. Examples for Customizing Governance 


_TODO: Add more cross references; Add examples of different configurations and their functionality_