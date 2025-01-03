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

