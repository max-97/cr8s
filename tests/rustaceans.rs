use reqwest::{StatusCode, blocking::Client};
use rocket::serde::json::{Value, json};

fn create_test_rustacean(client: &Client) -> Value {
    let response = client
        .post("http://localhost:8000/rustaceans")
        .json(&json!({
            "name": "John Smith",
            "email": "john@smith.com"
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    response.json().unwrap()
}

fn delete_test_rustacean(client: &Client, rustacean: Value) {
    let response = client
        .delete(format!(
            "http://localhost:8000/rustaceans/{}",
            rustacean["id"]
        ))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[test]
fn test_get_rustaceans() {
    let client = Client::new();
    let rustacean1 = create_test_rustacean(&client);
    let rustacean2 = create_test_rustacean(&client);

    let response = client
        .get("http://localhost:8000/rustaceans")
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&rustacean1));
    assert!(json.as_array().unwrap().contains(&rustacean2));

    delete_test_rustacean(&client, rustacean1);
    delete_test_rustacean(&client, rustacean2);
}

#[test]
fn test_create_rustaceans() {
    let client = Client::new();
    let response = client
        .post("http://localhost:8000/rustaceans")
        .json(&json!({
            "name": "John Smith",
            "email": "john@smith.com"
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let rustacean: Value = response.json().unwrap();
    assert_eq!(
        rustacean,
        json!({
            "id": rustacean["id"],
            "name": "John Smith",
            "email": "john@smith.com",
            "created_at": rustacean["created_at"]
        })
    );

    delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_view_rustaceans() {
    let client = Client::new();
    let rustacean = create_test_rustacean(&client);

    let response = client
        .get(format!(
            "http://localhost:8000/rustaceans/{}",
            rustacean["id"]
        ))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let rustacean: Value = response.json().unwrap();

    assert_eq!(
        rustacean,
        json!({
            "id": rustacean["id"],
            "name": "John Smith",
            "email": "john@smith.com",
            "created_at": rustacean["created_at"]
        })
    );

    delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_update_rustaceans() {
    let client = Client::new();
    let rustacean = create_test_rustacean(&client);

    let response = client
        .put(format!(
            "http://localhost:8000/rustaceans/{}",
            rustacean["id"]
        ))
        .json(&json!({
            "name": "Jane Smith",
            "email": "jane@smith.com"
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let rustacean: Value = response.json().unwrap();

    assert_eq!(
        rustacean,
        json!({
            "id": rustacean["id"],
            "name": "Jane Smith",
            "email": "jane@smith.com",
            "created_at": rustacean["created_at"]
        })
    );

    delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_delete_rustaceans() {
    let client = Client::new();
    let rustacean = create_test_rustacean(&client);

    let response = client
        .delete(format!(
            "http://localhost:8000/rustaceans/{}",
            rustacean["id"]
        ))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
