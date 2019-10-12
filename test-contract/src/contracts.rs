use super::*;

pub fn load_wasm() -> Vec<u8> {
    let mut file = File::open("/home/xlc/contract/flipper.wasm").unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    data
}

pub fn put_code(raw_seed: &str, gas_limit: Gas, code: Vec<u8>) {
    let function = Call::Contracts(ContractsCall::put_code(gas_limit, code));
    let utx = build_tx(raw_seed, function);
    let h = submit_tx(utx);
    println!("put_code hash: {:?}", h);
}

pub fn instantiate(
    raw_seed: &str,
    endowment: Balance,
    gas_limit: Gas,
    code_hash: Hash,
    data: Vec<u8>,
) {
    let function = Call::Contracts(ContractsCall::instantiate(
        endowment, gas_limit, code_hash, data,
    ));
    let utx = build_tx(raw_seed, function);
    let h = submit_tx(utx);
    println!("instantiate hash: {:?}", h);
}

pub fn call(raw_seed: &str, dest: AccountId, value: Balance, gas_limit: Gas, data: Vec<u8>) {
    let dest: srml_indices::address::Address<AccountId, AccountIndex> = dest.into();
    let function = Call::Contracts(ContractsCall::call(dest, value, gas_limit, data));
    let utx = build_tx(raw_seed, function);
    let h = submit_tx(utx);
    println!("call hash: {:?}", h);
}
