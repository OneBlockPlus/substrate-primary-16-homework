use crate::{mock::*, Error, Event, Proofs};
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
        assert_eq!(Proofs::<Test>::get(&claim), Some((1, 1)));
    });
}

#[test]
fn transfer_claim() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        // create
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        assert_ok!(PoeModule::create_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));

        assert_ok!(PoeModule::transfer_claim(
            RuntimeOrigin::signed(1),
            2,
            claim.clone()
        ));
        //查看二号有没有
        assert_eq!(Proofs::<Test>::get(&claim), Some((2, 1)));

        System::assert_has_event(
            Event::TransferredTo {
                from: 1,
                to: 2,
                claim: claim.clone(),
            }
            .into(),
        );
    });
}

#[test]
fn revoke_claim() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        //create
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        assert_ok!(PoeModule::create_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));
        //revoke
        assert_ok!(PoeModule::revoke_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));

        System::assert_has_event(
            Event::ClaimRevoked {
                owner: 1,
                claim: claim.clone(),
            }
            .into(),
        );
    })
}
