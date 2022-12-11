use serde_json::json;

pub fn heartbeat_payload(sequence: Option<u32>) -> String {
    if let Some(s) = sequence {
        json!({"op": 1, "d": s})
    } else {
        json!({"op": 1, "d": null})
    }
    .to_string()
}

pub fn identify_payload(token: &str) -> String {
    json!({
        "op": 2,
        "d": {
            "token": token,
            "properties": {
                "capabilities": 1021,
                "os": "Windows",
                "browser": "Chrome",
                "device": "strife"
            }
        }
    })
    .to_string()
}

pub fn resume_payload(token: &str, session_id: &str, sequence: u32) -> String {
    json!({
        "op": 6,
        "d": {
            "token": token,
            "session_id": session_id,
            "seq": sequence
        }
    })
    .to_string()
}
