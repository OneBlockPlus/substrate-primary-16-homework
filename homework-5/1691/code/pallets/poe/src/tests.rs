use crate as pallet_poe;
use crate::{mock::*, Error, Event, Proofs};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::BoundedVec;

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let claim = BoundedVec::try_from(vec![0,1]).unwrap();
        assert_ok!(PoEModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

        assert_eq!(pallet_poe::Proofs::<Test>::get(&claim), Some((1_u64, 1_u64)));
        // Go past genesis block so events get deposited
        System::set_block_number(1);
    });
}
