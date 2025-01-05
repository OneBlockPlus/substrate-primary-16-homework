// We make sure this pallet uses `no_std` for compiling to Wasm. 
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
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
        /// Maximum length allowed for a claim.
        #[pallet::constant]
        type MaxClaimLength: Get<u32>;

        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::storage]
    pub type Proofs<T: Config> = StorageMap
        _,
        Blake2_128Concat,
        BoundedVec<u8, T::MaxClaimLength>,
        (<T as frame_system::Config>::AccountId, BlockNumberFor<T>)
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new claim has been created.
        ClaimCreated {
            owner: <T as frame_system::Config>::AccountId,
            claim: BoundedVec<u8, T::MaxClaimLength>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Attempted to create a proof that already exists
        ProofAlreadyExists,
        ProofNotExist,
        NotClaimOwner
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // Create a new claim
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn create_claim(
            origin: OriginFor<T>,
            claim: BoundedVec<u8, T::MaxClaimLength>
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(
                !Proofs::<T>::contains_key(&claim),
                Error::<T>::ProofAlreadyExists
            );

            Proofs::<T>::insert(
                &claim,
                (who.clone(), frame_system::Pallet::<T>::block_number())
            );

            Self::deposit_event(Event::ClaimCreated { owner: who, claim });

            Ok(())
        }
        
        // Transfer ownership of a claim
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn transfer_claim(
            origin: OriginFor<T>,
            claim: BoundedVec<u8, T::MaxClaimLength>,
            dest: T::AccountId
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let (owner, _) = Proofs::<T>::get(&claim)
                .ok_or(Error::<T>::ProofNotExist)?;
            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            Proofs::<T>::mutate(&claim, |proof| {
                if let Some((_, block_number)) = proof {
                    *proof = Some((dest.clone(), *block_number));
                }
            });
            
            Ok(())
        }
        
        // Revoke a claim
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn revoke_claim(
            origin: OriginFor<T>,
            claim: BoundedVec<u8, T::MaxClaimLength>
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let (owner, _) = Proofs::<T>::get(&claim)
                .ok_or(Error::<T>::ProofNotExist)?;
            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            Proofs::<T>::remove(&claim);

            Ok(())
        }
    }
}