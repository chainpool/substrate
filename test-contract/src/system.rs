use super::*;

fn system_name() {
    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "system_name",
        "id": 1,
        "params": []
    });
    post(request);
}
