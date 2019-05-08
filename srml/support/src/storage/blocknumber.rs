use crate::Twox128;
use super::hashed::generator::StorageHasher;

static mut BLOCKNUMBER_HASHED_KEY: [u8; 16] = [0; 16];

pub fn set_blocknumber_key(_key: &'static [u8]) {
	#[cfg(all(feature = "std", any(feature = "msgbus", feature = "cache-lru")))] {
		unsafe {
			BLOCKNUMBER_HASHED_KEY = Twox128::hash(_key);
		}
	}
}

//pub fn blocknumber_key() -> &'static [u8] {
//	unsafe {
//		BLOCKNUMBER_KEY
//	}
//}

pub fn blocknumber_hashedkey() -> [u8; 16] {
	unsafe {
		BLOCKNUMBER_HASHED_KEY
	}
}