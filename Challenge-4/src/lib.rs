use http::{StatusCode};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use spin_sdk::{
    http::{IntoResponse, Params, Request, Response, Router},
    http_component,
    key_value::Store,
};

#[derive(Serialize, Deserialize, Debug)]
struct RegisterPayload {
    player_name: String,
    player_no: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct RecordPayload {
    player_no: u32,
    calories: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct PlayerInfo {
    player_name: String,
    #[serde(default = "zero_calories")]
    calories: u32,
}

// #[derive(Serialize, Deserialize, Debug)]
// struct PlayerInfo {
//     player_name: String,
//     player_no: String,
//     #[serde(default = "zero_calories")]
//     calories: u32,
// }

fn zero_calories () -> u32 {
    0
}

/// A simple Spin HTTP component.
#[http_component]
fn handle_challenge_four(req: Request) -> Response {
    let mut router = Router::new();

    router.post("/register", api::register);
    router.post("/record", api::record);
    router.any("/*", api::catch_all);
    router.handle(req)
}

mod api {
    use super::*;

    pub fn register(req: Request, _params: Params) -> Result<impl IntoResponse> {
        let store = Store::open_default()?;
        let body = req.body();
        match serde_json::from_slice(body) {
            Ok(x) => {
                let player: RegisterPayload = x;
                let player_key = player.player_no.to_string().clone();

                match store.get(player_key.as_str())? {
                    Some(_) => {
                        Ok(Response::new(StatusCode::CONFLICT, ""))
                    },
                    None => {
                        let player_info = PlayerInfo {
                            player_name: player.player_name,
                            calories: 0,
                        };
                        let stringified_player_info = serde_json::to_string(&player_info)?;

                        store.set(player_key.as_str(), stringified_player_info.as_bytes())?;
                        Ok(Response::new(StatusCode::CREATED, body.to_vec()))
                    },
                }
            },
            Err(_) => Ok(Response::new(StatusCode::BAD_REQUEST, "")),
        }
    }

    pub fn record(req: Request, _params: Params) -> Result<impl IntoResponse> {
        let store = Store::open_default()?;
        let body = req.body();
        match serde_json::from_slice(body) {
            Ok(x) => {
                let record: RecordPayload = x;
                let player_key = record.player_no.to_string().clone();

                match store.get(player_key.as_str())? {
                    Some(row) => {
                        let mut player_info: PlayerInfo = serde_json::from_slice(row.as_slice()).unwrap();

                        player_info.calories += record.calories;
                
                        let stringified_player_info = serde_json::to_string(&player_info)?;

                        store.set(player_key.as_str(), stringified_player_info.as_bytes())?;
                        Ok(Response::new(StatusCode::OK, stringified_player_info))
                    },
                    None => {
                        Ok(Response::new(StatusCode::NOT_FOUND, ""))
                    },
                }
            },
            Err(_) => Ok(Response::new(StatusCode::BAD_REQUEST, "")),
        }
    }

    pub fn catch_all(req: Request, _params: Params) -> Result<impl IntoResponse> {
        println!("Else");
        println!("Handling request to {:?}", req.query());
        Ok(Response::new(StatusCode::BAD_REQUEST, "Oops!"))
    }
}
