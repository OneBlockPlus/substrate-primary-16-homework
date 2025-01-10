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
        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::storage]
    pub type Claims<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, T::AccountId>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// 创建凭证
        ClaimCreated(T::Hash, T::AccountId),
        /// 传送凭证
        ClaimTransferred(T::Hash, T::AccountId, T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The claim already exists.
        ClaimAlreadyExists,
        /// The claim does not exist.
        ClaimNotFound,
        /// The caller is not the owner of the claim.
        NotClaimOwner,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(0)]  
        pub fn create_claim(origin: OriginFor<T>, claim: T::Hash) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(!Claims::<T>::contains_key(&claim), Error::<T>::ClaimAlreadyExists);

            Claims::<T>::insert(&claim, &sender);

            Self::deposit_event(Event::ClaimCreated(claim, sender));

            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(0)] 
        pub fn transfer_claim(origin: OriginFor<T>, claim: T::Hash, to: T::AccountId) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let owner = Claims::<T>::get(&claim).ok_or(Error::<T>::ClaimNotFound)?;

            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            Claims::<T>::insert(&claim, &to);

            Self::deposit_event(Event::ClaimTransferred(claim, sender, to));

            Ok(())
        }
    }
}