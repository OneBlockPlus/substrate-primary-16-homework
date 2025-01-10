use super::*;
use crate as pallet_poe;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};
use sp_core::Get;
// use sp_runtime::BoundedVec;

#[test]
fn it_works_for_create_claim() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let claim = BoundedVec::try_from(vec![1, 2, 3]).unwrap();
        assert_ok!(PoEModule::create_claim(
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
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![1, 1]).unwrap();
        assert_ok!(PoEModule::create_claim(
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

#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![1, 1]).unwrap();
        let _ = PoEModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

        assert_ok!(PoEModule::revoke_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));
    })
}
