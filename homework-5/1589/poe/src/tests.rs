use crate as pallet_poe;
use crate::mock::*;
use crate::Error;
use frame_support::{assert_noop, assert_ok};
use sp_runtime::BoundedVec;

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        assert_ok!(PoeModule::create_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));

        assert_eq!(
            pallet_poe::Proofs::<Test>::get(&claim),
            Some((1_u64, 1_u64))
        );
        System::set_block_number(1);

        assert_noop!(
            PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()),
            Error::<Test>::ProofAlreadyExists
        );
    });
}

#[test]
fn it_works_for_transfer_claim() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

        assert_ok!(PoeModule::create_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));

        assert_ok!(PoeModule::transfer_claim(
            RuntimeOrigin::signed(1),
            claim.clone(),
            2
        ));

        assert_eq!(
            pallet_poe::Proofs::<Test>::get(&claim),
            Some((2_u64, 1_u64))
        );

        assert_noop!(
            PoeModule::transfer_claim(RuntimeOrigin::signed(1), claim.clone(), 3),
            Error::<Test>::ProofNotOwner
        );

        let claim = BoundedVec::try_from(vec![2, 3]).unwrap();

        assert_noop!(
            PoeModule::transfer_claim(RuntimeOrigin::signed(2), claim.clone(), 3),
            Error::<Test>::ProofNotFound
        );
    });
}
