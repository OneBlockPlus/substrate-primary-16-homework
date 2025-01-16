use crate as pallet_poe;
use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::BoundedVec;

#[test]
fn it_works_for_create_claim() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        assert_ok!(PoEModule::create_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));

        assert_eq!(
            pallet_poe::Proofs::<Test>::get(&claim),
            Some((1_u64, 1_u64))
        );

        System::set_block_number(1);
    });
}

#[test]
fn it_works_for_transfer_claim() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let owner = RuntimeOrigin::signed(1);
        let to = 2;
        let claim = BoundedVec::try_from(vec![0, 1, 3, 4]).unwrap();

        assert_ok!(PoEModule::create_claim(owner.clone(), claim.clone()));

        assert_ok!(PoEModule::transfer_claim(owner, claim.clone(), to));

        let (c_owner,_block_number) = pallet_poe::Proofs::<Test>::get(&claim).unwrap();

        // 验证owner 已经取不出数 ,数据已经来到了转移的 to 地址
        assert_eq!(c_owner,to);

        // 查看事件是否执行
        System::assert_last_event(Event::ClaimTransfer {from:1,to,claim}.into());

        System::set_block_number(1);
    });
}
