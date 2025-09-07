use reqwest::{StatusCode, blocking::Client};
use serde_json::{Value, json};

pub static APP_HOST: &'static str = "http://localhost:8000";

pub fn create_test_rustacean(client: &Client) -> Value {
    let response = client
        .post(format!("{}/rustaceans", APP_HOST))
        .json(&json!({
            "name": "John Smith",
            "email": "john@smith.com"
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    response.json().unwrap()
}

pub fn delete_test_rustacean(client: &Client, rustacean: Value) {
    let response = client
        .delete(format!("{}/rustaceans/{}", APP_HOST, rustacean["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

pub fn delete_test_crate(client: &Client, crate_: Value) {
    let response = client
        .delete(format!("{}/crates/{}", APP_HOST, crate_["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
