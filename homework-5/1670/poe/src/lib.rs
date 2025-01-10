

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



// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet]
pub mod pallet {
    // Import various useful types required by all FRAME pallets.
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

    /// A storage item for this pallet.
    ///
    /// In this template, we are declaring a storage item called `Something` that stores a single
    /// `u32` value. Learn more about runtime storage here: <https://docs.substrate.io/build/runtime-storage/>
    #[pallet::storage]
    pub type Proofs<T: Config> = StorageMap<_,Blake2_128Concat,BoundedVec<u8,T::MaxClaimLength>,(T::AccountId,BlockNumberFor<T>)>;

    /// Events that functions in this pallet can emit.
    ///
    /// Events are a simple means of indicating to the outside world (such as dApps, chain explorers
    /// or other users) that some notable update in the runtime has occurred. In a FRAME pallet, the
    /// documentation for each event field and its parameters is added to a node's metadata so it
    /// can be used by external interfaces or tools.
    ///
    ///	The `generate_deposit` macro generates a function on `Pallet` called `deposit_event` which
    /// will convert the event type of your pallet into `RuntimeEvent` (declared in the pallet's
    /// [`Config`] trait) and deposit it using [`frame_system::Pallet::deposit_event`].
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A user has successfully set a new value.
        ClaimCreated {
            // The new value set.
            owner: T::AccountId,
            claim: BoundedVec<u8, T::MaxClaimLength>,
            
        },
        ClaimRevoked {
           
            owner: T::AccountId,
          
            claim: BoundedVec<u8, T::MaxClaimLength>,
        },
    }

    /// Errors that can be returned by this pallet.
    ///
    /// Errors tell users that something went wrong so it's important that their naming is
    /// informative. Similar to events, error documentation is added to a node's metadata so it's
    /// equally important that they have helpful documentation associated with them.
    ///
    /// This type of runtime error can be up to 4 bytes in size should you want to return additional
    /// information.
    #[pallet::error]
    pub enum Error<T> {
        ClaimNotExist,
        NotClaimOwner,
        ProofAlreadyExist,
    
    }

    
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // 创建存证
        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn initiate_claim(origin: OriginFor<T>, new_claim: BoundedVec<u8, T::MaxClaimLength>) -> DispatchResult {
            // 确保交易由签名者发起并获取签名者信息
            let creator = ensure_signed(origin)?;
    
            // 确保存证尚未存在
            ensure!(!Proofs::<T>::contains_key(&new_claim), Error::<T>::ProofAlreadyExist);
    
            // 插入存证信息到存储
            Proofs::<T>::insert(&new_claim, (creator.clone(), frame_system::Pallet::<T>::block_number()));
    
            // 触发事件
            Self::deposit_event(Event::ClaimCreated { owner: creator, claim: new_claim });
    
            // 返回成功结果
            Ok(())
        }
    
        // 撤销存证
        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn terminate_claim(origin: OriginFor<T>, claim_to_revoke: BoundedVec<u8, T::MaxClaimLength>) -> DispatchResult {
            // 确保交易由签名者发起并获取签名者信息
            let revoker = ensure_signed(origin)?;
    
            // 获取存证信息并确保存证存在
            let (owner, _) = Proofs::<T>::get(&claim_to_revoke).ok_or(Error::<T>::ClaimNotExist)?;
    
            // 确保撤销请求由存证拥有者发起
            ensure!(owner == revoker, Error::<T>::NotClaimOwner);
    
            // 从存储中移除存证信息
            Proofs::<T>::remove(&claim_to_revoke);
    
            // 触发事件
            Self::deposit_event(Event::ClaimRevoked { owner: revoker, claim: claim_to_revoke });
    
            // 返回成功结果
            Ok(())
        }
    
        // 转移存证
        #[pallet::call_index(2)]
        #[pallet::weight(0)]
        pub fn transfer_claim_ownership(
            origin: OriginFor<T>,
            claim_to_transfer: BoundedVec<u8, T::MaxClaimLength>,
            new_owner_account: T::AccountId,
        ) -> DispatchResult {
            // 确保交易由签名者发起并获取签名者信息
            let current_owner = ensure_signed(origin)?;
    
            // 获取存证信息并确保存证存在
            let (owner, _) = Proofs::<T>::get(&claim_to_transfer).ok_or(Error::<T>::ClaimNotExist)?;
    
            // 确保转移请求由存证拥有者发起
            ensure!(owner == current_owner, Error::<T>::NotClaimOwner);
    
            // 确保新拥有者不是当前拥有者
            ensure!(new_owner_account != current_owner, Error::<T>::NotClaimOwner);
    
            // 更新存储中的存证信息，将拥有者设置为新的账户
            Proofs::<T>::insert(
                &claim_to_transfer,
                (new_owner_account.clone(), frame_system::Pallet::<T>::block_number()),
            );
    
            // 触发事件
            Self::deposit_event(Event::ClaimCreated { owner: new_owner_account, claim: claim_to_transfer });
    
            // 返回成功结果
            Ok(())
        }
    }
    
}
