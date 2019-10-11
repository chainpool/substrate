use super::*;

pub fn free_balance_of(who: &AccountId) -> Balance {
    let key = <srml_balances::FreeBalance<Runtime>>::key_for(who);
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

pub fn account_nonce(who: &AccountId) -> Index {
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
