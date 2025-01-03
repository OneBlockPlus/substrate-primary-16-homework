#![cfg(test)]

use super::*;
use crate::mock::{new_test_ext, Origin, Poe, System, Test};
use frame_support::{assert_noop, assert_ok, BoundedVec};

#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        assert_ok!(Poe::create_claim(Origin::signed(1), claim.clone()));

        assert_eq!(
            Proofs::<Test>::get(&claim),
            Some((1, frame_system::Pallet::<Test>::block_number()))
        );
    });
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        let _ = Poe::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(
            Poe::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ProofAlreadyExist
        );
    });
}

#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        let _ = Poe::create_claim(Origin::signed(1), claim.clone());

        assert_ok!(Poe::revoke_claim(Origin::signed(1), claim.clone()));
    });
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

        assert_noop!(
            Poe::revoke_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ClaimNotExist
        );
    });
}

#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        let _ = Poe::create_claim(Origin::signed(1), claim.clone());

        assert_ok!(Poe::transfer_claim(Origin::signed(1), claim.clone(), 2));

        assert_eq!(
            Proofs::<Test>::get(&claim),
            Some((2, frame_system::Pallet::<Test>::block_number()))
        );
    });
}

#[test]
fn transfer_claim_failed_when_not_owner() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        let _ = Poe::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(
            Poe::transfer_claim(Origin::signed(2), claim.clone(), 3),
            Error::<Test>::NotClaimOwner
        );
    });
}
