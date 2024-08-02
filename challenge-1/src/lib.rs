use std::str;
use http::{StatusCode};
use spin_sdk::http::{IntoResponse, Request, Response, Method};
use spin_sdk::http_component;

#[allow(non_snake_case)]
#[derive(serde::Deserialize)]
struct Decrypt {
    // requestBody: String,
    // actionType: String,
    response: String,
}

/// A simple Spin HTTP component.
#[http_component]
async fn handle_challenge_one(req: Request) -> anyhow::Result<impl IntoResponse> {
    
    println!("Handling request to {:?}", req.header("spin-full-url"));

    let resp = match *req.method() {
        Method::Post => { 
            let secret_play = "Very Secret Play";
            let encryption_module_path = "crypto";
            let s = match str::from_utf8(req.body()) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
            let decrypt_request = Request::post("http://localhost:3000/crypto", s)
                .header("x-action", "decrypt")
                .build();
            let current_playbook: Response = spin_sdk::http::send(decrypt_request).await?;
            let s2 = match str::from_utf8(current_playbook.body()) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };

            let decrypt_parsed: Decrypt = serde_json::from_str(s2).unwrap();
            let mut complete_playbook = decrypt_parsed.response.clone();
            
            complete_playbook.push_str(secret_play);

            let reencrypt_request = Request::post("http://localhost:3000/crypto", complete_playbook)
                .header("x-action", "encrypt")
                .build();
            let complete_playbook: Response = spin_sdk::http::send(reencrypt_request).await?;
            let s3 = match str::from_utf8(complete_playbook.body()) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
            let reencrypt_parsed: Decrypt = serde_json::from_str(s3).unwrap();

            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .header("x-secret-play", secret_play)
                .header("x-encryption-module-path", encryption_module_path)
                .body(format!("{{ \"encryptedMessage\": \"{}\"}}", reencrypt_parsed.response))
                .build()
        }
        // Only POST method is supported
        _ => {
            Response::builder()
                .status(StatusCode::METHOD_NOT_ALLOWED)
                .build()
        }
    };

    Ok(resp)
}
