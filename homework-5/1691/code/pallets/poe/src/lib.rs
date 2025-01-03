
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

    /// The pallet's configuration trait.
    ///
    /// All our types and constants a pallet depends on must be declared here.
    /// These types are defined generically and made concrete when the pallet is declared in the
    /// `runtime/src/lib.rs` file of your chain.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        #[pallet::constant]
        type MaxClaimLength: Get<u32>;
        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    /// A storage item for this pallet.
    ///
    /// In this template, we are declaring a storage item called `Something` that stores a single
    /// `u32` value. Learn more about runtime storage here: <https://docs.substrate.io/build/runtime-storage/>
    #[pallet::storage]
    pub type Proofs<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        BoundedVec<u8, T::MaxClaimLength>,
        (T::AccountId, BlockNumberFor<T>),
    >;

    // 事件
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A user has successfully set a new value.
        ClaimCreated {
            owner: T::AccountId,
            claim: BoundedVec<u8, T::MaxClaimLength>,
        },
        ClaimRevoked{
            owner: T::AccountId,
            claim: BoundedVec<u8, T::MaxClaimLength>,
        },
        ClaimTransfer{
            old_owner: T::AccountId,
            new_owner: T::AccountId,
            claim: BoundedVec<u8, T::MaxClaimLength>,
        }
    }

    // 报错
    #[pallet::error]
    pub enum Error<T> {

        ProofAlreadyExist, // 已存在了
        NotAuthorization, // 没有权利删除
        NotExistProof, // 不存在，不能删除
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// An example dispatchable that takes a single u32 value as a parameter, writes the value
        /// to storage and emits an event.
        ///
        /// It checks that the _origin_ for this call is _Signed_ and returns a dispatch
        /// error if it isn't. Learn more about origins here: <https://docs.substrate.io/build/origins/>
        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn create_claim(origin: OriginFor<T>, claim: BoundedVec<u8, T::MaxClaimLength>) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            let who = ensure_signed(origin)?;
            ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist); // 检查Proofs里还没有这个claim地址

            let current_block = frame_system::Pallet::<T>::block_number();
            Proofs::<T>::insert(&claim, (who.clone(), current_block));

            // Emit an event.
            Self::deposit_event(Event::ClaimCreated {owner: who, claim });

            // Return a successful `DispatchResult`
            Ok(())
        }

        #[pallet::weight(0)]
        pub fn revoked_claim(origin: OriginFor<T>, claim: BoundedVec<u8, T::MaxClaimLength>) -> DispatchResult {

            let who = ensure_signed(origin)?;
            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::NotExistProof); // 判断是否存在

            let (owner, _) = Proofs::<T>::get(&claim).unwrap();
            ensure!(who == owner, Error::<T>::NotAuthorization); // 确保是同一人

            Proofs::<T>::remove(&claim);
            Self::deposit_event(Event::ClaimRevoked {owner: who, claim });

            Ok(())
        }

        #[pallet::weight(0)]
        pub fn transfer_ownership(origin: OriginFor<T>, claim: BoundedVec<u8, T::MaxClaimLength>, to: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::NotExistProof); // 判断是否存在

            let (owner, _) = Proofs::<T>::get(&claim).unwrap();
            ensure!(who == owner, Error::<T>::NotAuthorization); // 确保是同一人

            let current_block = frame_system::Pallet::<T>::block_number();
            Proofs::<T>::remove(&claim);
            Proofs::<T>::insert(&claim, (to.clone(), current_block));
            Self::deposit_event(Event::ClaimTransfer {old_owner: who, new_owner: to, claim });

            Ok(())
        }
    }
}
