#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
pub use pallet::*;
// pub use weights::WeightInfo;

#[cfg(test)]
mod mock;

// This module contains the unit tests for this pallet.
// Learn about pallet unit testing here: https://docs.substrate.io/test/unit-testing/
#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    // Import various useful types required by all FRAME pallets.
    use super::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        #[pallet::constant]
        type MaxClaimLength: Get<u32>;
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        // type WeightInfo: WeightInfo;
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
        /// A user has successfully set a new value.
        ClaimCreated(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
        ClaimRevoked(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
    }

    #[pallet::error]
    pub enum Error<T> {
        ProofAlreadyExists,
        ClaimTooLong,
        ClaimNotExist,
        NotClaimOwner,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        // #[pallet::weight(T::WeightInfo::create_claim(claim.len() as u32))]
        pub fn create_claim(
            origin: OriginFor<T>,
            claim: BoundedVec<u8, T::MaxClaimLength>,
        ) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            let sender = ensure_signed(origin)?;
            ensure!(
                !Proofs::<T>::contains_key(&claim),
                Error::<T>::ProofAlreadyExists
            );

            Proofs::<T>::insert(
                &claim,
                (sender.clone(), frame_system::Pallet::<T>::block_number()),
            );

            // Emit an event.
            Self::deposit_event(Event::ClaimCreated(sender, claim));

            // Return a successful `DispatchResult`
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn revoke_claim(
            origin: OriginFor<T>,
            claim: BoundedVec<u8, T::MaxClaimLength>,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            //从存储中获取声明的所有者和创建时间
            match Proofs::<T>::take(&claim) {
                Some((owner, _)) if owner == sender => {
                    Self::deposit_event(Event::ClaimRevoked(sender, claim));
                    Ok(().into())
                },
                Some(_) => Err(Error::<T>::NotClaimOwner.into()),
                None => Err(Error::<T>::ClaimNotExist.into()),
            }
        }


        #[pallet::call_index(2)]
        #[pallet::weight(0)]
        // 允许声明的所有者将声明转移给另一个账户
        pub fn transfer_claim(
            origin: OriginFor<T>,//调用者的来源
            claim: BoundedVec<u8, T::MaxClaimLength>,//要转移的声明
            new_owner: T::AccountId,//声明的新所有者
        ) -> DispatchResultWithPostInfo {
        let sender = ensure_signed(origin)?;//确保调用者是一个签名的账户

        let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

        ensure!(owner == sender, Error::<T>::NotClaimOwner);
        ensure!(new_owner != owner, Error::<T>::NotClaimOwner);

        Proofs::<T>::insert(
            &claim,
            (new_owner.clone(), frame_system::Pallet::<T>::block_number()),
        );

        Self::deposit_event(Event::ClaimCreated(new_owner, claim));

        Ok(())
        }

    }