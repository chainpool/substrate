mod chain;
mod contracts;
mod rpc;
mod state;
mod system;
mod tx_builder;

use node_cli::chain_spec::get_account_id_from_seed;
use node_primitives::{AccountId, Balance, BlockNumber, Hash, Index};
use node_runtime::{BalancesCall, Call, Gas, Runtime};
use parity_codec::{Compact, Decode, Encode};
use sr_primitives::generic::Era;
use srml_support::storage::StorageMap;

use std::fs::File;
use std::io::Read;

use srml_contracts::Call as ContractsCall;

use substrate_primitives::blake2_256;
use substrate_primitives::ed25519::Pair;
use substrate_primitives::hexdisplay::HexDisplay;
use substrate_primitives::sr25519;
use substrate_primitives::sr25519::Public as AddressPublic;
use substrate_primitives::Pair as PairT;

use chain::genesis_hash;
use contracts::{instantiate, load_wasm, put_code};
use rpc::post;
use state::{account_nonce, free_balance_of};
use tx_builder::{build_tx, submit_tx};

fn transfer(raw_seed: &str, to: AccountId, amount: Balance) {
    let to = AddressPublic::from_raw(to.0);
    let function = Call::Balances(BalancesCall::transfer(to.into(), amount));
    let utx = build_tx(raw_seed, function);
    let h = submit_tx(utx);
}

fn write_to_file(utx: String) {
    use std::error::Error;
    use std::io::Write;
    use std::path::Path;

    let path = Path::new("utx.txt");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(utx.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
    let alice = get_account_id_from_seed("Alice");
    let bob = get_account_id_from_seed("Bob");

    free_balance_of(&alice);
    free_balance_of(&bob);

    // transfer("Alice", bob.clone(), 200);
    // free_balance_of(&alice);
    // free_balance_of(&bob);

    let wasm = load_wasm();
    let wasm_hash = blake2_256(&wasm);
    println!("WASM hash: {}", HexDisplay::from(&wasm_hash.as_ref()));
    let wasm_h: Hash = wasm_hash.clone().into();
    println!("WASM h: {}", HexDisplay::from(&wasm_h.as_ref()));
    println!("wasm.len(): {:?}", wasm.len());
    // put_code("Alice", 1000_000_000_000, wasm);

    // instantiate("Alice", 10000000, 1000000000, wasm_hash.into(), vec![]);

    Ok(())
}
