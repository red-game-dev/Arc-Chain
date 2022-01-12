#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.io/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::{
		sp_runtime::traits::Hash,
		traits::{ Randomness, Currency, tokens::ExistenceRequirement },
		transactional
	};
	use sp_io::hashing::blake2_128;
	use scale_info::TypeInfo;

	#[cfg(feature = "std")]
	use frame_support::serde::{Deserialize, Serialize};

	type AccountOf<T> = <T as frame_system::Config>::AccountId;
	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	// Struct for holding NFT information.
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct NFT<T: Config> {
		pub dna: [u8; 16],   // Using 16 bytes to represent a nft DNA
		pub price: Option<BalanceOf<T>>,
		pub gender: Gender,
		pub owner: AccountOf<T>,
	}

	// Set Gender type in NFT struct.
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum Gender {
		Male,
		Female,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The Currency handler for the NFTs pallet.
		type Currency: Currency<Self::AccountId>;

		/// The maximum amount of NFTs a single account can own.
		#[pallet::constant]
		type MaxNFTOwned: Get<u32>;

		/// The type of Randomness we want to specify for this pallet.
		type NFTRandomness: Randomness<Self::Hash, Self::BlockNumber>;
	}

	// Errors.
	#[pallet::error]
	pub enum Error<T> {
		/// Handles arithemtic overflow when incrementing the NFT counter.
		NFTCntOverflow,
		/// An account cannot own more NFTs than `MaxNFTCount`.
		ExceedMaxNFTOwned,
		/// Buyer cannot be the owner.
		BuyerIsNFTOwner,
		/// Cannot transfer a nft to its owner.
		TransferToSelf,
		/// Handles checking whether the NFT exists.
		NFTNotExist,
		/// Handles checking that the NFT is owned by the account transferring, buying or setting a price for it.
		NotNFTOwner,
		/// Ensures the NFT is for sale.
		NFTNotForSale,
		/// Ensures that the buying price is greater than the asking price.
		NFTBidPriceTooLow,
		/// Ensures that an account has enough funds to purchase a NFT.
		NotEnoughBalance,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new NFT was sucessfully created. \[sender, nft_id\]
		Created(T::AccountId, T::Hash),
		/// NFT price was sucessfully set. \[sender, nft_id, new_price\]
		PriceSet(T::AccountId, T::Hash, Option<BalanceOf<T>>),
		/// A NFT was sucessfully transferred. \[from, to, nft_id\]
		Transferred(T::AccountId, T::AccountId, T::Hash),
		/// A NFT was sucessfully bought. \[buyer, seller, nft_id, bid_price\]
		Bought(T::AccountId, T::AccountId, T::Hash, BalanceOf<T>),
	}

	// Storage items.

	#[pallet::storage]
	#[pallet::getter(fn nft_cnt)]
	/// Keeps track of the number of NFTs in existence.
	pub(super) type NFTCnt<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn nfts)]
	/// Stores a NFT's unique traits, owner and price.
	pub(super) type NFTs<T: Config> = StorageMap<_, Twox64Concat, T::Hash, NFT<T>>;

	#[pallet::storage]
	#[pallet::getter(fn nfts_owned)]
	/// Keeps track of what accounts own what NFT.
	pub(super) type NFTsOwned<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, BoundedVec<T::Hash, T::MaxNFTOwned>, ValueQuery>;

    // TODO Part IV: Our pallet's genesis configuration.

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new unique nft.
		///
		/// The actual nft creation is done in the `mint()` function.
		#[pallet::weight(100)]
		pub fn create_nft(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let nft_id = Self::mint(&sender, None, None)?;

			// Logging to the console
			log::info!("🎈 A nft is born with ID ➡ {:?}.", nft_id);
			// Deposit our "Created" event.
			Self::deposit_event(Event::Created(sender, nft_id));
			Ok(())
		}

        // TODO Part IV: set_price

        // TODO Part IV: transfer

        // TODO Part IV: buy_nft

        // TODO Part IV: breed_nft
    }


	//** Our helper functions.**//

	impl<T: Config> Pallet<T> {
		fn gen_gender() -> Gender {
			let random = T::NFTRandomness::random(&b"gender"[..]).0;
			match random.as_ref()[0] % 2 {
				0 => Gender::Male,
				_ => Gender::Female,
			}
		}

		fn gen_dna() -> [u8; 16] {
			let payload = (
				T::NFTRandomness::random(&b"dna"[..]).0,
				<frame_system::Pallet<T>>::block_number(),
			);
			payload.using_encoded(blake2_128)
		}

		// Helper to mint a NFT.
		pub fn mint(
			owner: &T::AccountId,
			dna: Option<[u8; 16]>,
			gender: Option<Gender>,
		) -> Result<T::Hash, Error<T>> {
			let nft = NFT::<T> {
				dna: dna.unwrap_or_else(Self::gen_dna),
				price: None,
				gender: gender.unwrap_or_else(Self::gen_gender),
				owner: owner.clone(),
			};

			let nft_id = T::Hashing::hash_of(&nft);

			// Performs this operation first as it may fail
			let new_cnt = Self::nft_cnt().checked_add(1)
				.ok_or(<Error<T>>::NFTCntOverflow)?;

			// Performs this operation first because as it may fail
			<NFTsOwned<T>>::try_mutate(&owner, |nft_vec| {
				nft_vec.try_push(nft_id)
			}).map_err(|_| <Error<T>>::ExceedMaxNFTOwned)?;

			<NFTs<T>>::insert(nft_id, nft);
			<NFTCnt<T>>::put(new_cnt);
			Ok(nft_id)
		}

        // TODO Part IV: transfer_nft_to
    }
}
