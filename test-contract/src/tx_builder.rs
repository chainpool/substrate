use super::*;

pub fn build_tx(raw_seed: &str, function: Call) -> String {
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
    println!("signature: {:?}", signature);
    let xt = node_runtime::UncheckedExtrinsic::new_signed(
        index,
        raw_payload.1,
        from.into(),
        signature.into(),
        era,
    )
    .encode();
    println!("encoded.xt.len(): {}", xt.len());

    let utx = format!("0x{}", hex::encode(&xt));

    // Problematic!!!!
    // let utx = format!("0x{:}", HexDisplay::from(&xt));

    println!("utx.len() : {}", utx.len());
    utx
}

pub fn submit_tx(utx: String) -> Option<Hash> {
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
    println!("Hash: {:?}", h);
    h
}
