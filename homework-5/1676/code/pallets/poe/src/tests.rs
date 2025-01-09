use super::*;
use crate as pallet_poe;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};
use frame_support::pallet_prelude::Get;

#[test]
fn it_works_for_create_claim() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let claim = BoundedVec::try_from(vec![1, 2, 3]).unwrap();
        assert_ok!(PoeModule::create_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));

        assert_eq!(pallet_poe::Proofs::<Test>::get(&claim), Some((1, 1)));
        assert_eq!(
            pallet_poe::Proofs::<Test>::get(&claim),
            Some((1_u64, 1_u64))
        );
        System::set_block_number(1);
    });
}


#[test]
fn it_works_for_revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![1, 1]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

        assert_ok!(PoeModule::revoke_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));
    })
}


#[test]
fn it_works_for_transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![1, 1]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

        assert_ok!(PoeModule::transfer_claim(
            RuntimeOrigin::signed(1),
            claim.clone(),
            2
        ));

        let bounded_claim =
            BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();
        assert_eq!(
            Proofs::<Test>::get(&bounded_claim),
            Some((2, frame_system::Pallet::<Test>::block_number()))
        );
    })
}


