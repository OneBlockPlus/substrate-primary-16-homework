// use crate::{mock::*, Error, Event};
// use frame_support::{assert_noop, assert_ok};
// use sp_runtime::BoundedVec;
// use crate::Pallet as PoeModule;
// use crate as pallet_poe;

use super::*;
use crate as pallet_poe;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};


#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![1, 1]).unwrap();
        assert_ok!(PoeModule::create_claim(
            RuntimeOrigin::signed(2),
            claim.clone()
        ));

        assert_eq!(
            Proofs::<Test>::get(&claim),
            Some((2, frame_system::Pallet::<Test>::block_number()))
        );
        assert_eq!(<<Test as Config>::MaxClaimLength as Get<u32>>::get(), 100);
    })
}
