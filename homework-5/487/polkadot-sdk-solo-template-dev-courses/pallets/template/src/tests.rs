use super::*;
use crate::{mock::*, Error};
// assert_noop 没有任何操作
use frame_support::{assert_noop, assert_ok, BoundedVec};

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		// 1. 创建一个声明 vec->BoundedVec
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

		// 2. 创建声明成功
		// Origin::signed(1) 代表第一个用户
		// claim.clone() 代表声明
		assert_ok!(TemplateModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

		// 3. 读取声明
		assert_eq!(
			Claims::<Test>::get(&claim),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);
	});
}

#[test]
fn create_claim_claim_already_exist() {
	new_test_ext().execute_with(|| {
		// 创建声明
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

		// 创建声明成功
		assert_ok!(TemplateModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

		// 重复创建声明
		assert_noop!(
			TemplateModule::create_claim(RuntimeOrigin::signed(1), claim.clone()),
			Error::<Test>::AlreadyClaimed
		);
	});
}

#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		// 创建声明
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

		// 创建声明成功
		assert_ok!(TemplateModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

		// 撤销声明
		assert_ok!(TemplateModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()));

		// 读取声明
		assert_eq!(Claims::<Test>::get(&claim), None);
	});
}

#[test]
fn revoke_claim_failed_with_claim_not_exist() {
	new_test_ext().execute_with(|| {
		// 创建声明
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

		// 撤销声明
		assert_noop!(
			TemplateModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()),
			Error::<Test>::NoSuchClaim
		);
	});
}

#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		// 创建声明
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

		// 创建声明成功
		assert_ok!(TemplateModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

		// 转移声明
		assert_ok!(TemplateModule::transfer_claim(RuntimeOrigin::signed(1), claim.clone(), 2));

		// 读取声明
		assert_eq!(
			Claims::<Test>::get(&claim),
			Some((2, frame_system::Pallet::<Test>::block_number()))
		);
	});
}

#[test]
fn transfer_claim_failed_with_claim_not_exist() {
	new_test_ext().execute_with(|| {
		// 创建声明
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

		// 转移声明
		assert_noop!(
			TemplateModule::transfer_claim(RuntimeOrigin::signed(1), claim.clone(), 2),
			Error::<Test>::NoSuchClaim
		);
	});
}
