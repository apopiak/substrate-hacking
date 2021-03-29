use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

use crate::{MetaData, MetaDataStore};

#[test]
fn minting_work() {
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
