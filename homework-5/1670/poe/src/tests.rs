
use super::*;
use crate as pallet_poe;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};
#[test]
#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1, 2, 3];
        let bounded_claim = BoundedVec::try_from(claim.clone()).unwrap();
        let origin = RuntimeOrigin::signed(1);

        assert_ok!(Pallet::<Test>::initiate_claim(origin, bounded_claim.clone()));

        assert_eq!(Proofs::<Test>::get(&bounded_claim), Some((1, 0)));
    });
}

 // 测试当证明已存在时，创建证明应该失败
 #[test]
 fn create_claim_fails_when_claim_already_exists() {
     new_test_ext().execute_with(|| {
         let claim = vec![0, 1, 2, 3];
         let bounded_claim = BoundedVec::try_from(claim.clone()).unwrap();
         let origin = RuntimeOrigin::signed(1);

         assert_ok!(Pallet::<Test>::initiate_claim(origin.clone(), bounded_claim.clone()));
         assert_noop!(
             Pallet::<Test>::initiate_claim(origin, bounded_claim),
             Error::<Test>::ProofAlreadyExist
         );
     });
 }

  // 测试撤销证明的功能
  #[test]
  fn revoke_claim_works() {
      new_test_ext().execute_with(|| {
          let claim = vec![0, 1, 2, 3];
          let bounded_claim = BoundedVec::try_from(claim.clone()).unwrap();
          let origin = RuntimeOrigin::signed(1);

          assert_ok!(Pallet::<Test>::initiate_claim(origin.clone(), bounded_claim.clone()));
          assert_ok!(Pallet::<Test>::terminate_claim(origin, bounded_claim.clone()));

          assert!(Proofs::<Test>::get(&bounded_claim).is_none());
      });
  }


  // 测试当证明不存在时，撤销证明应该失败
  #[test]
  fn revoke_claim_fails_when_claim_does_not_exist() {
      new_test_ext().execute_with(|| {
          let claim = vec![0, 1, 2, 3];
          let bounded_claim = BoundedVec::try_from(claim.clone()).unwrap();
          let origin = RuntimeOrigin::signed(1);

          assert_noop!(
              Pallet::<Test>::terminate_claim(origin, bounded_claim),
              Error::<Test>::ClaimNotExist
          );
      });
  }


  // 测试非证明所有者尝试撤销证明应该失败
  #[test]
  fn revoke_claim_fails_when_not_owner() {
      new_test_ext().execute_with(|| {
          let claim = vec![0, 1, 2, 3];
          let bounded_claim = BoundedVec::try_from(claim.clone()).unwrap();
          let origin = RuntimeOrigin::signed(1);
          let other_origin = RuntimeOrigin::signed(2);

          assert_ok!(Pallet::<Test>::initiate_claim(origin, bounded_claim.clone()));
          assert_noop!(
              Pallet::<Test>::terminate_claim(other_origin, bounded_claim),
              Error::<Test>::NotClaimOwner
          );
      });
  }

  // 测试转移证明的所有权功能
  #[test]
  fn transfer_claim_ownership_works() {
      new_test_ext().execute_with(|| {
          let claim = vec![0, 1, 2, 3];
          let bounded_claim = BoundedVec::try_from(claim.clone()).unwrap();
          let origin = RuntimeOrigin::signed(1);
          let new_owner = 2;

          assert_ok!(Pallet::<Test>::initiate_claim(origin.clone(), bounded_claim.clone()));
          assert_ok!(Pallet::<Test>::transfer_claim_ownership(origin, bounded_claim.clone(), new_owner));

          assert_eq!(Proofs::<Test>::get(&bounded_claim), Some((new_owner, 0)));
      });
  }

    // 测试当证明不存在时，转移证明应该失败
    #[test]
    fn transfer_claim_fails_when_claim_does_not_exist() {
        new_test_ext().execute_with(|| {
            let claim = vec![0, 1, 2, 3];
            let bounded_claim = BoundedVec::try_from(claim.clone()).unwrap();
            let origin = RuntimeOrigin::signed(1);
            let new_owner = 2;

            assert_noop!(
                Pallet::<Test>::transfer_claim_ownership(origin, bounded_claim, new_owner),
                Error::<Test>::ClaimNotExist
            );
        });
    }

// 测试非证明所有者尝试转移证明应该失败
#[test]
fn transfer_claim_fails_when_not_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1, 2, 3];
        let bounded_claim = BoundedVec::try_from(claim.clone()).unwrap();
        let owner_origin = RuntimeOrigin::signed(1);
        let non_owner_origin = RuntimeOrigin::signed(2);
        let new_owner = 3;

        // 创建证明
        assert_ok!(Pallet::<Test>::initiate_claim(owner_origin, bounded_claim.clone()));

        // 非所有者尝试转移证明
        assert_noop!(
            Pallet::<Test>::transfer_claim_ownership(non_owner_origin, bounded_claim, new_owner),
            Error::<Test>::NotClaimOwner
        );
    });
}
