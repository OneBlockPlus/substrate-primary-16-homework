// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

// FRAME pallets require their own "mock runtimes" to be able to run unit tests. This module
// contains a mock runtime specific for testing this pallet's functionality.
#[cfg(test)]
mod mock;

// This module contains the unit tests for this pallet.
// Learn about pallet unit testing here: https://docs.substrate.io/test/unit-testing/
#[cfg(test)]
mod tests;

// Every callable function or "dispatchable" a pallet exposes must have weight values that correctly
// estimate a dispatchable's execution time. The benchmarking module is used to calculate weights
// for each dispatchable and generates this pallet's weight.rs file. Learn more about benchmarking here: https://docs.substrate.io/test/benchmark/
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{dispatch::DispatchResult, ensure, pallet_prelude::*, Blake2_128Concat};
    use frame_system::pallet_prelude::{BlockNumberFor, *};

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        #[pallet::constant]
        type MaxClaimLength: Get<u32>;

        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::storage]
    pub type Proofs<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        BoundedVec<u8, T::MaxClaimLength>,
        (T::AccountId, BlockNumberFor<T>),
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ClaimCreated {
            owner: T::AccountId,
            claim: BoundedVec<u8, T::MaxClaimLength>,
        },
        ClaimTransfered {
            owner: T::AccountId,
            claim: BoundedVec<u8, T::MaxClaimLength>,
        },
        ClaimRevoked {
            owner: T::AccountId,
            claim: BoundedVec<u8, T::MaxClaimLength>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        ProofAlreadyExists,
        NotOwner,
        ClaimNotExists,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight({0})]
        pub fn create_claim(
            origin: OriginFor<T>,
            claim: BoundedVec<u8, T::MaxClaimLength>,
        ) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            let who = ensure_signed(origin)?;

            ensure!(
                !Proofs::<T>::contains_key(&claim),
                Error::<T>::ProofAlreadyExists
            );
            Proofs::<T>::insert(
                &claim,
                (who.clone(), frame_system::Pallet::<T>::block_number()),
            );

            Self::deposit_event(Event::ClaimCreated { owner: who, claim });

            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight({0})]
        pub fn transfer_claim(
            origin: OriginFor<T>,
            to: T::AccountId,
            claim: BoundedVec<u8, T::MaxClaimLength>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let (owner, block_number) =
                Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExists)?;
            ensure!(who == owner, Error::<T>::NotOwner);

            // remove old relation
            Proofs::<T>::remove(&claim);

            Proofs::<T>::insert(&claim, (&to, block_number));

            Self::deposit_event(Event::ClaimTransfered { owner: to, claim });

            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight({0})]
        pub fn revoke(
            origin: OriginFor<T>,
            claim: BoundedVec<u8, T::MaxClaimLength>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExists)?;
            ensure!(who == owner, Error::<T>::NotOwner);

            Proofs::<T>::remove(&claim);

            Self::deposit_event(Event::ClaimRevoked { owner, claim });

            Ok(())
        }
    }
}
