use std::collections::HashMap;
use http::{StatusCode};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use spin_sdk::{
    http::{IntoResponse, Params, Request, Response, Router},
    http_component,
    key_value::Store,
};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct GameDescription {
    message: String,
    gameId: String,
    grid: Vec<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
struct EndpointResponse {
    solved: bool,
    correctLetters: Vec<String>,
}

fn parse_query(query: &str) -> HashMap<String, String> {
    query
        .split('&')
        .filter_map(|s| {
            s.split_once('=')
                .and_then(|t| Some((t.0.to_owned(), t.1.to_owned())))
        })
        .collect()
}

/// A simple Spin HTTP component.
#[http_component]
fn handle_challenge_three(req: Request) -> Response {
    let mut router = Router::new();

    router.post("/api/start", api::start_game);
    router.get("/api/guess", api::guess_game);
    router.any("/*", api::catch_all);
    router.handle(req)
}

mod api {
    use super::*;

    pub fn start_game(req: Request, _params: Params) -> Result<impl IntoResponse> {
        let store = Store::open_default()?;
        let body = req.body();
        let game: GameDescription = serde_json::from_slice(body).unwrap();

        store.set(game.gameId.as_str(), body)?;
        Ok(Response::new(StatusCode::OK, "Wow!"))
    }

    pub fn guess_game(req: Request, _params: Params) -> Result<impl IntoResponse> {
        let query_map = parse_query(req.query());
        let game_id = query_map.get("gameId").unwrap().as_str();
        let guess_word = query_map.get("guess").unwrap().chars();

        if guess_word.clone().count() != 5 {
            return Ok(Response::builder().status(StatusCode::BAD_REQUEST).build());
        }
        // println!("Handling request to {:?}", guess_word.clone().count());
        
        let store = Store::open_default()?;
        let stored_game = store.get(game_id).unwrap().unwrap().clone();
        let game: GameDescription = serde_json::from_slice(stored_game.as_slice()).unwrap();
        let guess_compare = guess_word
            .enumerate()
            .map(|(index, letter)| if game.grid[0][index].chars().nth(0).unwrap() == letter {letter} else {'_'});
        let response_struct = EndpointResponse {
            solved: guess_compare.clone().all(|it| it != '_' ),
            correctLetters: guess_compare.clone().map(|it| String::from(it)).collect(),
        };
        let response_json = serde_json::to_string(&response_struct)?;
        
        let response = Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(response_json)
                .build();
        Ok(response)
    }

    pub fn catch_all(req: Request, _params: Params) -> Result<impl IntoResponse> {
        println!("Else");
        println!("Handling request to {:?}", req.query());
        Ok(Response::new(StatusCode::BAD_REQUEST, "Oops!"))
    }
}
