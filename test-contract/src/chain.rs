use super::*;

pub fn block_hash(block_number: BlockNumber) -> Hash {
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

pub fn genesis_hash() -> Hash {
    block_hash(0u64)
}
