use crate as pallet_poe;
use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::BoundedVec;

#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0u8; 10]; // 创建一个声明
        let sender = 1u64; // 假设账户ID为1

        // 调用create_claim函数并检查是否成功。
        assert_ok!(PoEModule::create_claim(
            RuntimeOrigin::signed(sender),
            BoundedVec::try_from(claim.clone()).unwrap()
        ));

        // 检查存储中是否有新的声明。
        assert_eq!(
            pallet_poe::Proofs::<Test>::get(&BoundedVec::try_from(claim.clone()).unwrap()),
            Some((sender, frame_system::Pallet::<Test>::block_number()))
        );

        // 检查事件是否被正确发出。
        System::assert_last_event(RuntimeEvent::PoEModule(Event::ClaimCreated {
            owner: sender,
            claim: BoundedVec::try_from(claim).unwrap(),
        }));
    });
}

#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0u8; 10];
        let original_owner = 1u64;
        let new_owner = 2u64;

        // 先创建声明
        assert_ok!(PoEModule::create_claim(
            RuntimeOrigin::signed(original_owner),
            BoundedVec::try_from(claim.clone()).unwrap()
        ));

        // 然后尝试转移声明
        assert_ok!(PoEModule::transfer_claim(
            RuntimeOrigin::signed(original_owner),
            BoundedVec::try_from(claim.clone()).unwrap(),
            new_owner
        ));

        // 检查声明是否已被转移到新所有者
        assert_eq!(
            pallet_poe::Proofs::<Test>::get(&BoundedVec::try_from(claim.clone()).unwrap()),
            Some((new_owner, frame_system::Pallet::<Test>::block_number()))
        );

        // 检查事件是否被正确发出。
        System::assert_last_event(RuntimeEvent::PoEModule(Event::ClaimTransferred {
            old_owner: original_owner,
            new_owner,
            claim: BoundedVec::try_from(claim).unwrap(),
        }));
    });
}

#[test]
fn transfer_claim_fails_when_not_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0u8; 10];
        let original_owner = 1u64;
        let non_owner = 3u64;
        let new_owner = 2u64;

        // 创建声明
        assert_ok!(PoEModule::create_claim(
            RuntimeOrigin::signed(original_owner),
            BoundedVec::try_from(claim.clone()).unwrap()
        ));

        // 尝试由非所有者转移声明
        assert_noop!(
            PoEModule::transfer_claim(
                RuntimeOrigin::signed(non_owner),
                BoundedVec::try_from(claim.clone()).unwrap(),
                new_owner
            ),
            Error::<Test>::NotClaimOwner
        );

        // 检查声明仍然属于原始所有者
        assert_eq!(
            pallet_poe::Proofs::<Test>::get(&BoundedVec::try_from(claim).unwrap()),
            Some((original_owner, frame_system::Pallet::<Test>::block_number()))
        );
    });
}

#[test]
fn transfer_nonexistent_claim_fails() {
    new_test_ext().execute_with(|| {
        let claim = vec![0u8; 10];
        let sender = 1u64;
        let receiver = 2u64;

        // 尝试转移一个不存在的声明
        assert_noop!(
            PoEModule::transfer_claim(
                RuntimeOrigin::signed(sender),
                BoundedVec::try_from(claim.clone()).unwrap(),
                receiver
            ),
            Error::<Test>::ProofDoesNotExist
        );

        // 检查声明确实不存在
        assert_eq!(
            pallet_poe::Proofs::<Test>::get(&BoundedVec::try_from(claim).unwrap()),
            None
        );
    });
}

#[test]
fn create_duplicate_claim_fails() {
    new_test_ext().execute_with(|| {
        let claim = vec![0u8; 10];
        let sender = 1u64;

        // 第一次创建声明
        assert_ok!(PoEModule::create_claim(
            RuntimeOrigin::signed(sender),
            BoundedVec::try_from(claim.clone()).unwrap()
        ));

        // 尝试再次创建相同的声明
        assert_noop!(
            PoEModule::create_claim(
                RuntimeOrigin::signed(sender),
                BoundedVec::try_from(claim).unwrap()
            ),
            Error::<Test>::ProofAlreadyExists
        );
    });
}
