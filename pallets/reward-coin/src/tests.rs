use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

use crate::{MetaData, MetaDataStore};

#[test]
fn minting_works() {
	new_test_ext().execute_with(|| {
		MetaDataStore::<Test>::put(MetaData {
			issuance: 0,
			minter: 1,
			burner: 1,
		});
		// Dispatch a signed extrinsic.
		assert_ok!(RewardCoin::mint(Origin::signed(1), 2, 42));
	});
}

#[test]
fn transfer_works() {
	new_test_ext().execute_with(|| {
		MetaDataStore::<Test>::put(MetaData {
			issuance: 0,
			minter: 1,
			burner: 1,
		});
		// Dispatch a signed extrinsic.
		assert_ok!(RewardCoin::mint(Origin::signed(1), 2, 42));

		assert_noop!(RewardCoin::transfer(Origin::signed(2), 3, 50), Error::<Test>::InsufficientBalance);

		assert_noop!(RewardCoin::transfer(Origin::signed(2), 4, 41), Error::<Test>::BelowMinBalance);

		assert_noop!(RewardCoin::transfer(Origin::signed(2), 4, 1), Error::<Test>::BelowMinBalance);

		assert_ok!(RewardCoin::transfer(Origin::signed(2), 3, 15));
	});
}