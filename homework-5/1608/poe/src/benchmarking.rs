#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as PoeModule;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use frame_support::BoundedVec;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn create_claim(b:Linear<1,{T::MaxClaimLength::get()}>)->Result<(),BenchmarkError> {
        let claim = BoundedVec::try_from(vec![0; b as usize]).unwrap();
        let caller: T::AccountId = whitelisted_caller();
        #[extrinsic_call]
        create_claim(RawOrigin::Signed(caller.clone()), claim.clone());

        assert_eq!(
            Proofs::<T>::get(&claim),
            Some((caller, frame_system::Pallet::<T>::block_number()))
        );
        Ok(())
    }
}
// benchmarks! {
//     create_claim {
//         let d in 0 .. T::MaxClaimLength::get();
//         let claim = BoundedVec::try_from(vec![0; d as usize]).unwrap();
//         let caller: T::AccountId = whitelisted_caller();
//     }: _(RawOrigin::Signed(caller.clone()), claim.clone())
//     verify {
//         assert_last_event::<T>(Event::ClaimCreated(caller, claim).into())
//     }
//
//     revoke_claim {
//         let d in 0 .. T::MaxClaimLength::get();
//         let claim = BoundedVec::try_from(vec![0; d as usize]).unwrap();
//         let caller: T::AccountId = whitelisted_caller();
//         assert!(Pallet::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(), claim.clone()).is_ok());
//     }: _(RawOrigin::Signed(caller.clone()), claim.clone())
//     verify {
//         assert_last_event::<T>(Event::ClaimRevoked(caller, claim).into())
//     }
//
//     transfer_claim {
//         let d in 0 .. T::MaxClaimLength::get();
//         let claim = BoundedVec::try_from(vec![0; d as usize]).unwrap();
//         let caller: T::AccountId = whitelisted_caller();
//         let target: T::AccountId = account("target", 0, 0);
//         assert!(Pallet::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(), claim.clone()).is_ok());
//     }: _(RawOrigin::Signed(caller), claim, target)
//
//     impl_benchmark_test_suite!(PoeModule, crate::mock::new_test_ext(), crate::mock::Test);
// }
