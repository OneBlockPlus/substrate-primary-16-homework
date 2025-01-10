use crate as pallet_poe;
use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use sp_core::ConstU32;
use sp_runtime::BoundedVec;

#[test]
fn poe_test() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let claim = BoundedVec::<u8, ConstU32<100>>::try_from(vec![1, 2, 3]).unwrap();
        // Go past genesis block so events get deposited
        assert_ok!(PoeModule::create_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));
        assert_eq!(pallet_poe::Proof::<Test>::get(&claim), Some((1_u64, 1_u64)));
        assert_ok!(PoeModule::verify_proof(
            RuntimeOrigin::signed(1),
            1,
            claim.clone()
        ));
        System::set_block_number(2);
        assert_ok!(PoeModule::transfer_claim(
            RuntimeOrigin::signed(1),
            claim.clone(),
            2
        ));
        assert_eq!(pallet_poe::Proof::<Test>::get(&claim), Some((2_u64, 2_u64)));
        System::set_block_number(3);
        assert_ok!(PoeModule::withdraw_claim(
            RuntimeOrigin::signed(2),
            claim.clone()
        ));
        assert_eq!(pallet_poe::Proof::<Test>::get(&claim), None);
    });
}
