
use frame_support::{decl_module, decl_storage, decl_event, decl_error, ensure, dispatch};
use frame_system::ensure_signed;
use sp_std::vec::Vec;

pub trait Trait: frame_system::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as PoeModule {
        Proofs: map hasher(blake2_128_concat) Vec<u8> => Option<T::AccountId>;
    }
}

decl_event!(
    pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
        ClaimCreated(AccountId, Vec<u8>),
        ClaimRevoked(AccountId, Vec<u8>),
        ClaimTransferred(AccountId, AccountId, Vec<u8>),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        ProofAlreadyClaimed,
        NoSuchProof,
        NotProofOwner,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        #[weight = 10_000]
        pub fn create_claim(origin, proof: Vec<u8>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

            Proofs::<T>::insert(&proof, &sender);

            Self::deposit_event(RawEvent::ClaimCreated(sender, proof));

            Ok(())
        }

        #[weight = 10_000]
        pub fn revoke_claim(origin, proof: Vec<u8>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

            let owner = Proofs::<T>::get(&proof).ok_or(Error::<T>::NoSuchProof)?;
            ensure!(owner == sender, Error::<T>::NotProofOwner);

            Proofs::<T>::remove(&proof);

            Self::deposit_event(RawEvent::ClaimRevoked(sender, proof));

            Ok(())
        }

        #[weight = 10_000]
        pub fn transfer_claim(origin, proof: Vec<u8>, receiver: T::AccountId) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

            let owner = Proofs::<T>::get(&proof).ok_or(Error::<T>::NoSuchProof)?;
            ensure!(owner == sender, Error::<T>::NotProofOwner);

            Proofs::<T>::insert(&proof, &receiver);

            Self::deposit_event(RawEvent::ClaimTransferred(sender, receiver, proof));

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Module, Trait};
    use frame_support::{assert_ok, assert_noop, impl_outer_origin};
    use sp_core::H256;
    use sp_runtime::{
        traits::{BlakeTwo256, IdentityLookup}, testing::Header,
    };
    use frame_system as system;

    impl_outer_origin! {
        pub enum Origin for Test {}
    }

    #[derive(Clone, Eq, PartialEq)]
    pub struct Test;
    impl system::Trait for Test {
        type BaseCallFilter = ();
        type Origin = Origin;
        type Call = ();
        type Index = u64;
        type BlockNumber = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Header = Header;
        type Event = ();
        type BlockHashCount = ();
        type DbWeight = ();
        type BlockLength = ();
        type BlockWeights = ();
        type Version = ();
        type PalletInfo = ();
        type AccountData = ();
        type OnNewAccount = ();
        type OnKilledAccount = ();
        type SystemWeightInfo = ();
    }

    impl Trait for Test {
        type Event = ();
    }

    type PoeModule = Module<Test>;

    #[test]
    fn create_claim_works() {
        new_test_ext().execute_with(|| {
            let proof = vec![0, 1];
            assert_ok!(PoeModule::create_claim(Origin::signed(1), proof.clone()));
            assert_eq!(Proofs::<Test>::get(&proof), Some(1));
        });
    }

    #[test]
    fn revoke_claim_works() {
        new_test_ext().execute_with(|| {
            let proof = vec![0, 1];
            let _ = PoeModule::create_claim(Origin::signed(1), proof.clone());
            assert_ok!(PoeModule::revoke_claim(Origin::signed(1), proof.clone()));
            assert_eq!(Proofs::<Test>::get(&proof), None);
        });
    }

    #[test]
    fn transfer_claim_works() {
        new_test_ext().execute_with(|| {
            let proof = vec![0, 1];
            let _ = PoeModule::create_claim(Origin::signed(1), proof.clone());
            assert_ok!(PoeModule::transfer_claim(Origin::signed(1), proof.clone(), 2));
            assert_eq!(Proofs::<Test>::get(&proof), Some(2));
        });
    }
}

