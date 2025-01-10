use crate::Pallet as PoE; // 引入 PoE Pallet
use super::*; // 引入所有内容
use crate::mock::{new_test_ext, Test};
use frame_support::{assert_noop, assert_ok};
use frame_system::Origin; // 引入 Origin
use sp_core::H256;

#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = H256::from_low_u64_be(1); // 创建一个示例 claim
        let sender = 1; // 发送者账户

        // 打印 sender
        println!("Sender: {:?}", sender);

        // 测试创建 claim
        assert_ok!(PoE::<Test>::create_claim(frame_system::Origin::<Test>::Signed(sender).into(), claim));

        // 验证 claim 是否正确存储
        assert_eq!(Claims::<Test>::get(&claim), Some(sender));

        // 测试重复创建 claim 是否失败
        assert_noop!(
            PoE::<Test>::create_claim(frame_system::Origin::<Test>::Signed(sender).into(), claim),
            Error::<Test>::ClaimAlreadyExists
        );
    });
}

#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = H256::from_low_u64_be(1); // 创建一个示例 claim
        let sender = 1; // 发送者账户
        let receiver = 2; // 接收者账户
        let unowner =3; //非拥有者

        // 打印 sender 和 receiver
        println!("Sender: {:?}, Receiver: {:?}", sender, receiver);

        // 创建 claim
        assert_ok!(PoE::<Test>::create_claim(frame_system::Origin::<Test>::Signed(sender).into(), claim));

        // 获取 claim 的 owner
     

        // 测试转移 claim
        assert_ok!(PoE::<Test>::transfer_claim(frame_system::Origin::<Test>::Signed(sender).into(), claim, receiver));

        // 验证 claim 是否正确转移
        assert_eq!(Claims::<Test>::get(&claim), Some(receiver));

        // 测试非所有者尝试转移 claim 是否失败
        assert_noop!(
            PoE::<Test>::transfer_claim(frame_system::Origin::<Test>::Signed(unowner).into(), claim, receiver),
            Error::<Test>::NotClaimOwner
        );

        // 测试转移不存在的 claim 是否失败
        assert_noop!(
            PoE::<Test>::transfer_claim(frame_system::Origin::<Test>::Signed(sender).into(), H256::from_low_u64_be(2), receiver),
            Error::<Test>::ClaimNotFound
        );
    });
}