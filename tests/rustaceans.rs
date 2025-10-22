use reqwest::{StatusCode, blocking::Client};
use serde_json::{Value, json};

pub mod common;

#[test]
fn test_get_rustaceans() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean1 = common::create_test_rustacean(&client);
    let rustacean2 = common::create_test_rustacean(&client);

    let response = client
        .get(format!("{}/rustaceans", common::APP_HOST))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&rustacean1));
    assert!(json.as_array().unwrap().contains(&rustacean2));

    common::delete_test_rustacean(&client, rustacean1);
    common::delete_test_rustacean(&client, rustacean2);
}

#[test]
fn test_create_rustaceans() {
    let client = common::get_client_with_logged_in_admin();
    let response = client
        .post(format!("{}/rustaceans", common::APP_HOST))
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

    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_view_rustaceans() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean = common::create_test_rustacean(&client);

    let response = client
        .get(format!(
            "{}/rustaceans/{}",
            common::APP_HOST,
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

    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_view_rustaceans_not_found() {
    let client = common::get_client_with_logged_in_admin();

    let response = client
        .get(format!("{}/rustaceans/{}", common::APP_HOST, -1))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[test]
fn test_update_rustaceans() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean = common::create_test_rustacean(&client);

    let response = client
        .put(format!(
            "{}/rustaceans/{}",
            common::APP_HOST,
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

    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_delete_rustaceans() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean = common::create_test_rustacean(&client);

    let response = client
        .delete(format!(
            "{}/rustaceans/{}",
            common::APP_HOST,
            rustacean["id"]
        ))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
