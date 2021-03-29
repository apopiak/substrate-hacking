#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use sp_std::prelude::*;

use codec::{Encode, Decode};
use sp_runtime::{
	RuntimeDebug, traits::{
		AtLeast32BitUnsigned, Zero, Saturating, CheckedAdd, CheckedSub,
	},
};
use frame_support::dispatch::DispatchResult;
use frame_support::transactional;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default)]
pub struct MetaData<AccountId, Balance> {
	issuance: Balance,
	minter: AccountId,
	burner: AccountId,
}

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

// Definition of the pallet logic, to be aggregated at runtime definition
// through `construct_runtime`.
#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use super::*;

	// Simple declaration of the `Pallet` type. It is a placeholder we use 
	// to implement traits and methods.
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Our pallet's configuration trait. All our types and constants go in here.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		// The type used to store balances.
		type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;

		// The minimum balance necessary for an account to exist.
		type MinBalance: Get<Self::Balance>;
	}

	#[pallet::storage]
	#[pallet::getter(fn meta_data)]
	pub(super) type MetaDataStore<T: Config> = StorageValue<_, MetaData<T::AccountId, T::Balance>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn account)]
	pub(super) type Accounts<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, T::Balance, ValueQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub admin: T::AccountId,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				admin: Default::default(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			MetaDataStore::<T>::put(MetaData {
				issuance: Zero::zero(),
				minter: self.admin.clone(),
				burner: self.admin.clone(),
			});
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	#[pallet::metadata(T::AccountId = "AccountId", T::Balance = "Balance")]
	pub enum Event<T: Config> {
		Created(T::AccountId),
		Killed(T::AccountId),
		Minted(T::AccountId, T::Balance),
		Burned(T::AccountId, T::Balance),
		Transfered(T::AccountId, T::AccountId, T::Balance),
	}

	#[pallet::error]
	pub enum Error<T> {
		// An account would go below the minimum balance if the operation were executed.
		BelowMinBalance,
		// The origin account does not have the required permission for the operation.
		NoPermission,
		/// An operation would lead to an overflow.
		Overflow,
		/// An operation would lead to an underflow.
		Underflow,
		// Cannot burn the balance of a non-existent account.
		CannotBurnEmpty,
		// There is not enough balance in the sender's account for the transfer.
		InsufficientBalance,
	}

	// You can implement the [`Hooks`] trait to define some logic
	// that should be exectued regularly in some context.
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		// `on_initialize` is executed at the beginning of the block before any extrinsics are
		// dispatched.
		//
		// This function must return the weight consumed by `on_initialize` and `on_finalize`.
		fn on_initialize(_n: T::BlockNumber) -> Weight {
			// Anything that needs to be done at the start of the block.
			// We don't do anything here.
			let mut meta = MetaDataStore::<T>::get();
			let value: T::Balance = 50u8.into();
			meta.issuance = meta.issuance.saturating_add(value);
			Accounts::<T>::mutate(&meta.minter, |bal| {
				*bal = bal.saturating_add(value);
			});

			0
		}
	}

	// Extrinsics callable from outside the runtime.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1_000)]
		pub(super) fn mint(
			origin: OriginFor<T>,
			beneficiary: T::AccountId,
			#[pallet::compact] amount: T::Balance,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			ensure!(amount >= T::MinBalance::get(), Error::<T>::BelowMinBalance);
			let mut meta = Self::meta_data();
			ensure!(sender == meta.minter, Error::<T>::NoPermission);

			meta.issuance = meta.issuance.checked_add(&amount).ok_or(Error::<T>::Overflow)?;

			// store the new issuance
			MetaDataStore::<T>::put(meta);

			if Self::increase_balance(&beneficiary, amount) {
				Self::deposit_event(Event::<T>::Created(beneficiary.clone()));
			}
			Self::deposit_event(Event::<T>::Minted(beneficiary, amount));

			Ok(().into())
		}

		#[pallet::weight(1_000)]
		pub(super) fn burn(
			origin: OriginFor<T>,
			burned: T::AccountId,
			#[pallet::compact] amount: T::Balance,
			allow_killing: bool,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let mut meta = Self::meta_data();
			ensure!(sender == meta.burner, Error::<T>::NoPermission);

			let balance = Accounts::<T>::get(&burned);
			ensure!(balance > Zero::zero(), Error::<T>::CannotBurnEmpty);
			let new_balance = balance.saturating_sub(amount);
			let burn_amount = if new_balance < T::MinBalance::get() {
				ensure!(allow_killing, Error::<T>::BelowMinBalance);
				let burn_amount = balance;
				ensure!(meta.issuance.checked_sub(&burn_amount).is_some(), Error::<T>::Underflow);
				Accounts::<T>::remove(&burned);
				Self::deposit_event(Event::<T>::Killed(burned.clone()));
				burn_amount
			} else {
				let burn_amount = amount;
				ensure!(meta.issuance.checked_sub(&burn_amount).is_some(), Error::<T>::Underflow);
				Accounts::<T>::insert(&burned, new_balance);
				burn_amount
			};

			// This is fine because we checked the issuance above.
			meta.issuance = meta.issuance.saturating_sub(burn_amount);
			// store the new issuance
			MetaDataStore::<T>::put(meta);

			Self::deposit_event(Event::<T>::Burned(burned, burn_amount));

			Ok(().into())
		}

		#[pallet::weight(1_000)]
		#[transactional]
		pub(super) fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			#[pallet::compact] amount: T::Balance,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			Accounts::<T>::try_mutate(&sender, |bal| -> DispatchResult {
				let new_bal = bal.checked_sub(&amount).ok_or(Error::<T>::InsufficientBalance)?;
				ensure!(new_bal >= T::MinBalance::get(), Error::<T>::BelowMinBalance);
				*bal = new_bal;
				Ok(())
			})?;
		
			Accounts::<T>::try_mutate(&to, |rec_bal| -> DispatchResult {
				let new_bal = rec_bal.saturating_add(amount);
				ensure!(new_bal >= T::MinBalance::get(), Error::<T>::BelowMinBalance);
				*rec_bal = new_bal;
				Ok(())
			})?;

			Self::deposit_event(Event::<T>::Transfered(sender, to, amount));

			Ok(().into())
		}
	}
}

// Internal functions of the pallet
impl<T: Config> Pallet<T> {
	fn increase_balance(acc: &T::AccountId, amount: T::Balance) -> bool {
		Accounts::<T>::mutate(&acc, |bal| {
			let created = bal == &Zero::zero();
			// fine because we check the issuance for overflow before minting and transfers
			// don't change the issuance
			*bal = bal.saturating_add(amount);
			created
		})
	}
}