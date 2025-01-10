use crate as pallet_poe;
use crate::{mock::*, Error, Event};
use frame_support::{assert_err, assert_ok};
use sp_runtime::BoundedVec;

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));
        assert_eq!(pallet_poe::Proofs::<Test>::get(&claim), Some((1_u64, 1_u64,true)));
        // Go past genesis block so events get deposited
        println!("{:?}", System::events());
        System::set_block_number(1);
        
    });
}

#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let claim  = BoundedVec::try_from(b"example_claim".to_vec()).unwrap();
        let sender = 1;

        // 创建声明
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

        // 检查存储内容
        let (owner, _block_number, is_active) = pallet_poe::Proofs::<Test>::get(&claim).unwrap();
        println!("Inserted proof: {:?}", pallet_poe::Proofs::<Test>::get(&claim));  // 打印存储内容
        println!("Owner: {:?}, is_active: {}", owner, is_active); // 打印所有者和激活状态
        assert_eq!(owner, sender);
        assert_eq!(is_active,true);

        // 检查事件触发
        println!("{:?}", System::events());
        System::assert_last_event(Event::ClaimCreated { owner: sender, claim }.into());
    });
}

#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let claim= BoundedVec::try_from(b"example_claim".to_vec()).unwrap();
        let sender = 1;

        // 创建声明
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(sender), claim.clone()));

        // 撤销声明
        assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(sender), claim.clone()));

        // 检查存储内容
        let (owner, _block_number, is_active) = pallet_poe::Proofs::<Test>::get(&claim).unwrap();
        assert_eq!(owner, sender);
        assert!(!is_active);

        // 检查事件触发
        System::assert_last_event(Event::ClaimRevoked { owner: sender, claim }.into());
    });
}

#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let claim = BoundedVec::try_from(b"example_claim".to_vec()).unwrap();
        let sender = 1;
        let new_owner = 2;

        // 创建声明
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(sender), claim.clone()));

        // 转移所有权
        assert_ok!(PoeModule::transfer_claim(RuntimeOrigin::signed(sender), claim.clone(), new_owner));

        // 检查存储内容
        let (owner, _block_number, is_active) = pallet_poe::Proofs::<Test>::get(&claim).unwrap();
        assert_eq!(owner, new_owner);
        assert!(is_active);

        // 检查事件触发
        System::assert_last_event(Event::ClaimTransferred { old_owner: sender, new_owner, claim }.into());
    });
}

#[test]
fn cannot_revoke_nonexistent_claim() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(b"nonexistent_claim".to_vec()).unwrap();
        let sender = 1;

        // 尝试撤销不存在的声明
        assert_err!(
            PoeModule::revoke_claim(RuntimeOrigin::signed(sender), claim.clone()),
            Error::<Test>::ProofNotExist
        );
    });
}

#[test]
fn cannot_transfer_without_ownership() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(b"example_claim".to_vec()).unwrap();
        let sender = 1;
        let new_owner = 2;

        // 创建声明
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(sender), claim.clone()));

        // 尝试非所有者转移
        assert_err!(
            PoeModule::transfer_claim(RuntimeOrigin::signed(new_owner), claim.clone(), sender),
            Error::<Test>::NotProofOwner
        );
    });
}

