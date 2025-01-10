#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet]
pub mod pallet {
    // Import various useful types required by all FRAME pallets.
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    // The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
    // (`Call`s) in this pallet.
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
        /// A user has successfully set a new value.
        ClaimCreated {
            owner: T::AccountId,
            claim: BoundedVec<u8, T::MaxClaimLength>,
        },

        ClaimTransferred {
            old_owner: T::AccountId,
            new_owner: T::AccountId,
            claim: BoundedVec<u8, T::MaxClaimLength>,
        }
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The value retrieved was `None` as no value was previously set.
        ProofAlreadyExists,
        ProofDoesNotExist,
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

            // Emit an event.
            Self::deposit_event(Event::ClaimCreated { owner: who, claim });

            // Return a successful `DispatchResult`
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn transfer_claim(
            origin: OriginFor<T>,
            claim: BoundedVec<u8, T::MaxClaimLength>,
            receiver: T::AccountId,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
    
            // Ensure the sender is the current owner of the claim.
            let (owner, block_number) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ProofDoesNotExist)?;
            ensure!(owner == sender, Error::<T>::NotClaimOwner);
    
            // Update the proof's owner to the receiver.
            Proofs::<T>::insert(&claim, (receiver.clone(), block_number));
    
            // Emit an event.
            Self::deposit_event(Event::ClaimTransferred { old_owner: sender, new_owner: receiver, claim });
    
            Ok(())
        }
    }
}
