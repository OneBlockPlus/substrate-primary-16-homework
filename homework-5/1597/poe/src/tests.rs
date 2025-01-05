use crate::mock::*;
use crate as pallet_poe;
use crate::{mock::*};
use frame_support::{assert_noop, assert_ok, BoundedVec};
use sp_core::crypto::AccountId32;
use super::*;

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let claim = BoundedVec::try_from(vec![1, 2]).unwrap();
        assert_ok!(PoeModule::claim_created(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));

        assert_eq!(pallet_poe::Proofs::<Test>::get(&claim), Some((1, 1)));
        assert_eq!(
            pallet_poe::Proofs::<Test>::get(&claim),
            Some((1_u64, 1_u64))
        );
        // Go past genesis block so events get deposited
        System::set_block_number(1);
    });
}

#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let claim = BoundedVec::try_from(vec![1, 2]).unwrap();
        assert_ok!(PoeModule::claim_created(RuntimeOrigin::signed(1), claim.clone()));

        assert_ok!(PoeModule::transfer_claim(
            RuntimeOrigin::signed(1),
            claim.clone(),
            2
        ));
        assert_eq!(Proofs::<Test>::get(&claim), Some((2, 1)));
    })
}
