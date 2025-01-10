use crate as pallet_poe;
use crate::{mock::*, Error, Event};
use frame_support::{assert_err, assert_noop, assert_ok};
use sp_runtime::{AccountId32, BoundedVec};

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        assert_ok!(PoEModule::create_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));

        assert_eq!(
            pallet_poe::Proofs::<Test>::get(&claim),
            Some((1_u64, 1_u64)) // (owner, block_number)
        );
        // Go past genesis block so events get deposited

        // Assert that the correct event was deposited
        System::assert_last_event(Event::ClaimCreated { owner: 1, claim }.into());
    });
}

#[test]
fn transfer_ok() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        assert_ok!(PoEModule::create_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));
        System::assert_last_event(Event::ClaimCreated { owner: 1, claim: claim.clone() }.into());

        assert_ok!(PoEModule::transfer_claim(
            RuntimeOrigin::signed(1),
            claim.clone(),
            2
        ));

        // Assert that the correct event was deposited
        System::assert_last_event(
            Event::ClaimTransferred {
                from: 1,
                to: 2,
                claim,
            }
            .into(),
        );
    });
}

#[test]
fn transfer_proof_not_found() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        assert_ok!(PoEModule::create_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));
        System::assert_last_event(Event::ClaimCreated { owner: 1, claim }.into());

        assert_err!(
            PoEModule::transfer_claim(
                RuntimeOrigin::signed(1),
                BoundedVec::try_from(vec![0, 1, 2]).unwrap(),
                2
            ),
            Error::<Test>::ProofNotFound
        );
    });
}

#[test]
fn transfer_not_by_owner() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        assert_ok!(PoEModule::create_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));
        System::assert_last_event(Event::ClaimCreated { owner: 1, claim: claim.clone() }.into());

        assert_err!(
            PoEModule::transfer_claim(RuntimeOrigin::signed(2), claim, 2),
            Error::<Test>::TransferNotByOwner
        );
    });
}

#[test]
fn revoke_ok() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        assert_ok!(PoEModule::create_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));
        assert_eq!(
            pallet_poe::Proofs::<Test>::get(&claim),
            Some((1_u64, 1_u64)) // (owner, block_number)
        );
        System::assert_last_event(Event::ClaimCreated { owner: 1, claim: claim.clone() }.into());


        assert_ok!(
            PoEModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone())
        );
        System::assert_last_event(Event::ClaimRevoked { owner: 1, claim: claim.clone() }.into());

    });
}

#[test]
fn revoke_not_by_owner() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        assert_ok!(PoEModule::create_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));
        assert_eq!(
            pallet_poe::Proofs::<Test>::get(&claim),
            Some((1_u64, 1_u64)) // (owner, block_number)
        );
        System::assert_last_event(Event::ClaimCreated { owner: 1, claim: claim.clone() }.into());


        assert_err!(
            PoEModule::revoke_claim(RuntimeOrigin::signed(2), claim.clone()),
            Error::<Test>::RevokeNotByOwner
        );

    }); 
}