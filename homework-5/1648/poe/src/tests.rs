use crate as pallet_poe;
use crate::{mock::*, Event};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::BoundedVec;

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let claim = BoundedVec::try_from(vec![10, 1]).unwrap();
        assert_ok!(PoeModule::create_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));

        assert_eq!(
            pallet_poe::Proofs::<Test>::get(&claim),
            Some((1_u64, 1_u64))
        );
    });
}

#[test]
fn it_works_for_transfer() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // NotExists
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        assert_noop!(
            PoeModule::transfer_claim(RuntimeOrigin::signed(1), 0x123, claim.clone()),
            pallet_poe::Error::<Test>::ClaimNotExists,
        );

        // prepare
        assert_ok!(PoeModule::create_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));

        // NotOwner
        assert_noop!(
            PoeModule::transfer_claim(RuntimeOrigin::signed(9), 0x123, claim.clone()),
            pallet_poe::Error::<Test>::NotOwner,
        );

        // transfer
        let to = 0x789;
        assert_ok!(PoeModule::transfer_claim(
            RuntimeOrigin::signed(1),
            to,
            claim.clone()
        ));

        assert_eq!(pallet_poe::Proofs::<Test>::get(&claim), Some((to, 1_u64)));
    });
}

#[test]
fn it_works_for_revoke() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

        // prepare
        assert_ok!(PoeModule::create_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));

        // remove
        assert_ok!(PoeModule::revoke(RuntimeOrigin::signed(1), claim.clone()),);

        assert_noop!(
            PoeModule::revoke(RuntimeOrigin::signed(1), claim.clone()),
            pallet_poe::Error::<Test>::ClaimNotExists,
        );

        System::assert_last_event(Event::ClaimRevoked { owner: 1, claim }.into());
    })
}
