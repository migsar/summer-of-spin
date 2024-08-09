use http::{StatusCode};
use serde::{Deserialize, Serialize};
use spin_sdk::http::{IntoResponse, Method, Request, Response};
use spin_sdk::http_component;
use spin_sdk::llm;
use serde_json;

#[derive(Serialize, Deserialize)]
struct EndpointResponse {
    tag: String,
    itinerary: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TripInfo {
    destination: String,
    duration: String,
    num_people: String,
    activities: Vec<String>,
}

fn create_prompt(TripInfo { destination, duration, num_people, activities }: TripInfo) -> String {
    let activities_string = activities.join(", ");
    format!(
        "Create a summer vacation detailed itinerary trip to go to {} for a {}. {} people will be going on this trip planning to do {}.",
        destination,
        duration,
        num_people,
        activities_string,
    )
}

fn create_itinerary(prompt: String) -> String {    
    let result = llm::infer_with_options(
        llm::InferencingModel::Llama2Chat,
        prompt.as_str(),
        llm::InferencingParams {
            max_tokens: 400,
            repeat_penalty: 1.1,
            repeat_penalty_last_n_token_count: 64,
            temperature: 0.8,
            top_k: 40,
            top_p: 0.9,
        },
    );

    match result {
        Ok(x) => x.text,
        Err(_e) => String::from(""),        
    }
}

/// A simple Spin HTTP component.
#[http_component]
fn handle_challenge_two(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    
    let resp = match *req.method() {
        Method::Post => { 
            let body = req.body();
            let trip_info: TripInfo = serde_json::from_slice(body).unwrap();

            let prompt = create_prompt(trip_info);
            let itinerary = create_itinerary(prompt);
            let response_struct = EndpointResponse {
                tag: "Something nice!".to_owned(),
                itinerary: itinerary,
            };
            let response_json = serde_json::to_string(&response_struct)?;
            
            Response::builder()
                .status(StatusCode::CREATED)
                .header("content-type", "application/json")
                .body(response_json)
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
