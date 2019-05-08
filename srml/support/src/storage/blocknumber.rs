
static mut BLOCKNUMBER_KEY: &'static [u8] = b"";

pub fn set_blocknumber_key(_key: &'static [u8]) {
	#[cfg(all(feature = "std", any(feature = "msgbus", feature = "cache-lru")))] {
		unsafe {
			BLOCKNUMBER_KEY = _key;
		}
	}
}

pub fn blocknumber_key() -> &'static [u8] {
	unsafe {
		BLOCKNUMBER_KEY
	}
}
