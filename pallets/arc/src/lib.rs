#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        sp_runtime::traits::{Hash, Zero},
        dispatch::{DispatchResultWithPostInfo, DispatchResult},
        traits::{Currency, ExistenceRequirement, Randomness},
        pallet_prelude::*
    };
    use frame_system::pallet_prelude::*;
    use sp_io::hashing::blake2_128;

    // TODO Part II: Struct for holding Arc information.

    // TODO Part II: Enum and implementation to handle Gender type in Arc struct.

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types it depends on.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The Currency handler for the Arc pallet.
        type Currency: Currency<Self::AccountId>;

        // TODO Part II: Specify the custom types for our runtime.

    }

    // Errors.
    #[pallet::error]
    pub enum Error<T> {
        // TODO Part III
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // TODO Part III
    }

    // ACTION: Storage item to keep a count of all existing Arc.
	#[pallet::storage]
	#[pallet::getter(fn arc_cnt)]
	/// Keeps track of the number of Arc in existence.
	pub(super) type ArcCnt<T: Config> = StorageValue<_, u64, ValueQuery>;

    // TODO Part II: Remaining storage items.

    // TODO Part III: Our pallet's genesis configuration.

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        // TODO Part III: create_arc

        // TODO Part III: set_price

        // TODO Part III: transfer

        // TODO Part III: buy_arc

        // TODO Part III: breed_arc
    }

    // TODO Parts II: helper function for Arc struct

    impl<T: Config> Pallet<T> {
        // TODO Part III: helper functions for dispatchable functions

        // TODO: increment_nonce, random_hash, mint, transfer_from

    }
}
