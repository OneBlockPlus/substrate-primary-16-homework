use crate as pallet_poe;
use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::BoundedVec;
use crate::Proofs;

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        
        System::set_block_number(1);
        // Go past genesis block so events get deposited
        let claim = BoundedVec::try_from(vec![0, 1 ]).unwrap();
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));
        
        assert_eq!(pallet_poe::Proofs::<Test>::get(&claim), Some((1, 1)));


        System::set_block_number(1);
       
    });
}

#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

        assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim), None);

        System::assert_has_event(Event::ClaimRevoked { owner: 1, claim }.into());
    });
}

#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

        assert_ok!(PoeModule::transfer_claim(
            RuntimeOrigin::signed(1),
            claim.clone(),
            2
        ));
        assert_eq!(Proofs::<Test>::get(&claim), Some((2, 1)));

        System::assert_has_event(Event::ClaimTransferred {
            from: 1,
            to: 2,
            claim,
        }
        .into());
    });
}

#[test]
fn revoke_claim_fails_if_not_owner() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

        assert_noop!(
            PoeModule::revoke_claim(RuntimeOrigin::signed(2), claim.clone()),
            Error::<Test>::NotClaimOwner
        );
    });
}

#[test]
fn transfer_claim_fails_if_not_owner() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

        assert_noop!(
            PoeModule::transfer_claim(RuntimeOrigin::signed(2), claim.clone(), 3),
            Error::<Test>::NotClaimOwner
        );
    });
}


