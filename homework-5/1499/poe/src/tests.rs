use crate as pallet_poe;
use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, traits::ConstU32};
use sp_runtime::BoundedVec;

// Test for creating a claim
#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create test claim
        let claim: BoundedVec<u8, ConstU32<100>> = BoundedVec::try_from(vec![0, 1]).unwrap();
        
        // Test create claim with account 1
        assert_ok!(PoEModule::create_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));
        
        // Check if claim exists and owned by account 1
        assert_eq!(
            pallet_poe::Proofs::<Test>::get(&claim),
            Some((1_u64, 1_u64))
        );
    });
}

// Test for transferring a claim
#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create test claim
        let claim: BoundedVec<u8, ConstU32<100>> = BoundedVec::try_from(vec![0, 1]).unwrap();
        
        // Create claim with account 1
        assert_ok!(PoEModule::create_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));
        
        // Transfer claim from account 1 to account 2
        assert_ok!(PoEModule::transfer_claim(
            RuntimeOrigin::signed(1),
            claim.clone(),
            2
        ));
        
        // Check if claim is now owned by account 2
        assert_eq!(
            pallet_poe::Proofs::<Test>::get(&claim),
            Some((2_u64, 1_u64))
        );
    });
}

// Test for revoking a claim
#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create test claim
        let claim: BoundedVec<u8, ConstU32<100>> = BoundedVec::try_from(vec![0, 1]).unwrap();
        
        // Create claim with account 1
        assert_ok!(PoEModule::create_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));
        
        // Revoke claim by account 1
        assert_ok!(PoEModule::revoke_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));
        
        // Check if claim is removed
        assert_eq!(
            pallet_poe::Proofs::<Test>::get(&claim),
            None
        );
    });
}