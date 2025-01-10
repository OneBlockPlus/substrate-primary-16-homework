#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        #[pallet::constant]
        type MaxClaimLength: Get<u32>;

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
        ClaimRevoked {
            owner: T::AccountId,
            claim: BoundedVec<u8, T::MaxClaimLength>,
        },
        ClaimTransferred {
            from: T::AccountId,
            to: T::AccountId,
            claim: BoundedVec<u8, T::MaxClaimLength>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        ProofAlreadyExists,
        ClaimNotFound,
        NotClaimOwner,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn create_claim(
            origin: OriginFor<T>,
            claim: BoundedVec<u8, T::MaxClaimLength>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExists);

            Proofs::<T>::insert(&claim, (who.clone(), frame_system::Pallet::<T>::block_number()));
            Self::deposit_event(Event::ClaimCreated { owner: who, claim });
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn revoke_claim(
            origin: OriginFor<T>,
            claim: BoundedVec<u8, T::MaxClaimLength>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotFound)?;
            ensure!(who == owner, Error::<T>::NotClaimOwner);

            Proofs::<T>::remove(&claim);
            Self::deposit_event(Event::ClaimRevoked { owner: who, claim });
            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(0)]
        pub fn transfer_claim(
            origin: OriginFor<T>,
            claim: BoundedVec<u8, T::MaxClaimLength>,
            to: T::AccountId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let (owner, block_number) =
                Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotFound)?;
            ensure!(who == owner, Error::<T>::NotClaimOwner);

            Proofs::<T>::insert(&claim, (to.clone(), block_number));
            Self::deposit_event(Event::ClaimTransferred {
                from: who,
                to,
                claim,
            });
            Ok(())
        }
    }
}
