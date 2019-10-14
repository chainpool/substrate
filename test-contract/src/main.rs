mod chain;
mod cmd;
mod contracts;
mod rpc;
mod state;
mod system;
mod tx_builder;

use node_cli::chain_spec::get_account_id_from_seed;
use node_primitives::{AccountId, AccountIndex, Balance, BlockNumber, Hash, Index};
use node_runtime::{BalancesCall, Call, Gas, Runtime};
use parity_codec::{Compact, Decode, Encode};
use sr_primitives::generic::Era;
use srml_contracts::ContractAddressFor;
use srml_support::storage::StorageMap;

use std::fs::File;
use std::io::Read;

use structopt::StructOpt;

use srml_contracts::Call as ContractsCall;

use substrate_primitives::blake2_256;
use substrate_primitives::ed25519::Pair;
use substrate_primitives::hexdisplay::HexDisplay;
use substrate_primitives::sr25519;
use substrate_primitives::sr25519::Public as AddressPublic;
use substrate_primitives::Pair as PairT;

use chain::genesis_hash;
use cmd::*;
use contracts::{call, instantiate, load_wasm, put_code};
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

    // free_balance_of(&alice);
    // free_balance_of(&bob);

    // transfer("Alice", bob.clone(), 200);
    // free_balance_of(&alice);
    // free_balance_of(&bob);

    // let input_data = Compact::from(vec![4266279973u32]).encode();
    // let input_data = vec![4266279973u32].encode();
    // println!("input_data: vec {:?}", input_data);
    let opt = cmd::Substrate::from_args();

    // parity/Substrate
    // Instantiate::
    // code_hash: 0xf4b1df2b2d11c7be74144b734e9cb207856c2a8e71108c92d89769b0cf517413
    // input_data: []
    // Alice
    // caller: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d (5GrwvaEF...)
    // dest: df55f0e4e45118902a3f732a29a156bbd5f0ace82da3f749e44d736fcc1fc101 (5H7Y4jn4...)

    match opt.cmd {
        Command::Flipper(flipper) => match flipper {
            Flipper::PutCode => {
                let wasm = load_wasm("/home/xlc/contract/flipper.wasm");
                let wasm_hash = blake2_256(&wasm);
                println!("code_hash: 0x{}", HexDisplay::from(&wasm_hash.as_ref()));
                let wasm_h: Hash = wasm_hash.clone().into();
                println!("WASM h: {}", HexDisplay::from(&wasm_h.as_ref()));

                put_code("Alice", 1000_000_000_000, wasm);
            }
            Flipper::Instantiate => {
                let wasm = load_wasm("/home/xlc/contract/flipper.wasm");
                let wasm_hash = blake2_256(&wasm);
                let wasm_h: Hash = wasm_hash.clone().into();
                let dest =
                    srml_contracts::SimpleAddressDeterminator::<Runtime>::contract_address_for(
                        &wasm_h,
                        &vec![],
                        &alice,
                    );
                // 100000000000500
                println!("dest: {:?}", dest);
                instantiate(
                    "Alice",
                    10000000000050000000,
                    1000000000,
                    wasm_hash.into(),
                    vec![],
                );
            }
            Flipper::Get => {
                let dest = flipper_dest(&alice);
                // flipper get: [37, 68, 74, 254]
                let input_data = 4266279973u32.encode();
                println!("get input_data: u32 {:?}", input_data);
                call("Alice", dest, 1000000000005000000, 1000000, input_data);
            }
            Flipper::Flip => {
                let dest = flipper_dest(&alice);
                let input_data = 970692492u32.encode();
                println!("flip input_data: u32 {:?}", input_data);
                call("Alice", dest, 1000000000005000000, 1000000, input_data);
            }
        },
    }

    Ok(())
}

fn flipper_dest(owner: &AccountId) -> AccountId {
    let wasm = load_wasm("/home/xlc/contract/flipper.wasm");
    let wasm_hash = blake2_256(&wasm);
    let wasm_h: Hash = wasm_hash.clone().into();
    let dest = srml_contracts::SimpleAddressDeterminator::<Runtime>::contract_address_for(
        &wasm_h,
        &vec![],
        owner,
    );
    dest
}
