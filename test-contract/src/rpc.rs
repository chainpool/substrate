use super::*;

pub fn post(req: serde_json::Value) -> Option<serde_json::Value> {
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
