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

fn build_tx(raw_seed: &str, function: Call) -> String {
    let signer = sr25519::Pair::from_string(&format!("//{}", raw_seed), None)
        .expect("static values are valid; qed");

    let from = signer.public();

    let genesis_hash = genesis_hash();
    let era = Era::immortal();
    let index = account_nonce(&from);

    let raw_payload = (Compact(index), function, era, genesis_hash);
    let signature = raw_payload.using_encoded(|payload| {
        if payload.len() > 256 {
            signer.sign(&blake2_256(payload)[..])
        } else {
            signer.sign(payload)
        }
    });
    let xt = node_runtime::UncheckedExtrinsic::new_signed(
        index,
        raw_payload.1,
        from.into(),
        signature.into(),
        era,
    )
    .encode();
    let utx = format!("0x{:}", HexDisplay::from(&xt));
    utx
}

fn submit_tx(utx: String) -> Option<Hash> {
    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "author_submitExtrinsic",
        "id": 1,
        "params": [utx],
    });
    let result = post(request).unwrap();
    let result: String = serde_json::from_str(&result.to_string()).unwrap();
    let blob = hex::decode(&result[2..]).unwrap();
    let h: Option<Hash> = Decode::decode(&mut blob.as_slice());
    h
}

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

fn put_code(raw_seed: &str, gas_limit: Gas, code: Vec<u8>) {
    let function = Call::Contracts(ContractsCall::put_code(gas_limit, code));
    let utx = build_tx(raw_seed, function);
    println!("utx len: {}", utx.len());
    println!("utx: {:#?}", utx);
    write_to_file(utx.clone());
    let h = submit_tx(utx);
    println!("put_code hash: {:?}", h);
}

fn block_hash(block_number: BlockNumber) -> Hash {
    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "chain_getBlockHash",
        "id": 1,
        "params": vec![block_number]
    });

    let result = post(request).unwrap();
    let result: String = serde_json::from_str(&result.to_string()).unwrap();
    let blob = hex::decode(&result[2..]).unwrap();
    let h: Option<Hash> = Decode::decode(&mut blob.as_slice());
    println!("hash: {:?}", h);
    h.unwrap()
}

fn post(req: serde_json::Value) -> Option<serde_json::Value> {
    let client = reqwest::Client::new();
    let resp = client.post("http://127.0.0.1:9933").json(&req).send();
    match resp {
        Ok(mut resp) => match resp.json::<serde_json::Value>() {
            Ok(result) => {
                if let Some(error) = result.get("error") {
                    println!("ERROR: {:?}", error);
                    return None;
                }
                println!("result: {:#?}", result.get("result"));
                log::debug!("result: {:#?}", result.get("result"));
                let default = serde_json::value::Value::String("0x00".to_string());
                Some(result.get("result").unwrap_or(&default).clone())
            }
            Err(e) => {
                log::info!("error: {:#?}", e);
                println!("error: {:#?}", e);
                None
            }
        },
        Err(e) => {
            println!("resp error, {:#?}", e);
            None
        }
    }
}

fn free_balance_of(who: &AccountId) -> Balance {
    let key = <srml_balances::FreeBalance<Runtime>>::key_for(who);
    // let hashed = substrate_primitives::twox_128(&key);
    let hashed = substrate_primitives::blake2_256(&key);
    let hexed = format!("0x{:}", HexDisplay::from(&hashed));

    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "state_getStorage",
        "id": 1,
        "params": [hexed]
    });

    let result = post(request).unwrap();
    let result: String = serde_json::from_str(&result.to_string()).unwrap();
    let blob = hex::decode(&result[2..]).unwrap();
    let balance: Option<Balance> = Decode::decode(&mut blob.as_slice());
    println!("free balance of {:#?}: {:?}", who, balance);

    balance.unwrap_or(0)
}

fn account_nonce(who: &AccountId) -> Index {
    let key = <srml_system::AccountNonce<Runtime>>::key_for(who);
    let hashed = substrate_primitives::blake2_256(&key);
    let hexed = format!("0x{:}", HexDisplay::from(&hashed));

    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "state_getStorage",
        "id": 1,
        "params": [hexed]
    });

    let result = post(request).unwrap_or(serde_json::value::Value::String("0x00".to_string()));
    if result == serde_json::Value::Null {
        println!("------------- result: {:?}", result);
        println!("account nonce of {:#?} is 0", who);
        return 0;
    }
    let result: String = serde_json::from_str(&result.to_string()).unwrap();
    let blob = hex::decode(&result[2..]).unwrap();
    let index: Option<Index> = Decode::decode(&mut blob.as_slice());
    println!("account nonce of {:#?}: {:?}", who, index);
    index.unwrap_or(0)
}

fn genesis_hash() -> Hash {
    block_hash(0u64)
}

fn load_wasm() -> Vec<u8> {
    let mut file = File::open("/home/xlc/contract/flipper.wasm").unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    data
}

fn system_name() {
    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "system_name",
        "id": 1,
        "params": []
    });
    post(request);
}

fn main() -> Result<(), Box<std::error::Error>> {
    let alice = get_account_id_from_seed("Alice");
    let bob = get_account_id_from_seed("Bob");

    free_balance_of(&alice);
    // free_balance_of(&bob);

    // transfer("Alice", bob.clone(), 200);
    // free_balance_of(&alice);
    // free_balance_of(&bob);

    let wasm = load_wasm();
    println!("wasm: {:?}", wasm);
    put_code("Alice", 100000, wasm);

    Ok(())
}
