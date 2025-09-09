use reqwest::{StatusCode, blocking::Client};
use serde_json::{Value, json};

pub mod common;

#[test]
fn test_get_crates() {
    let client = Client::new();
    let rustacean1 = common::create_test_rustacean(&client);
    let crate1 = common::create_test_crate(&client, &rustacean1);
    let crate2 = common::create_test_crate(&client, &rustacean1);

    let response = client
        .get(format!("{}/crates", common::APP_HOST))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&crate1));
    assert!(json.as_array().unwrap().contains(&crate2));

    common::delete_test_crate(&client, crate1);
    common::delete_test_crate(&client, crate2);
    common::delete_test_rustacean(&client, rustacean1);
}

#[test]
fn test_create_crates() {
    let client = Client::new();

    let rustacean = common::create_test_rustacean(&client);

    let response = client
        .post(format!("{}/crates", common::APP_HOST))
        .json(&json!({
            "rustacean_id": rustacean["id"],
            "code": "foo",
            "name": "Foo crate",
            "version": "0.1",
            "description": "Foo crate description"
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let crate_: Value = response.json().unwrap();
    assert_eq!(
        crate_,
        json!({
            "id": crate_["id"],
            "rustacean_id": rustacean["id"],
            "code": "foo",
            "name": "Foo crate",
            "version": "0.1",
            "description": "Foo crate description",
            "created_at": crate_["created_at"],
        })
    );

    common::delete_test_crate(&client, crate_);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_view_crate() {
    let client = Client::new();

    let rustacean = common::create_test_rustacean(&client);
    let crate_ = common::create_test_crate(&client, &rustacean);

    let response = client
        .get(format!("{}/crates/{}", common::APP_HOST, crate_["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let crate_: Value = response.json().unwrap();
    assert_eq!(
        crate_,
        json!({
            "id": crate_["id"],
            "rustacean_id": rustacean["id"],
            "code": "foo",
            "name": "Foo crate",
            "version": "0.1",
            "description": "Foo crate description",
            "created_at": crate_["created_at"],
        })
    );

    common::delete_test_crate(&client, crate_);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_update_crate() {
    let client = Client::new();

    let rustacean = common::create_test_rustacean(&client);
    let crate_ = common::create_test_crate(&client, &rustacean);

    let response = client
        .put(format!("{}/crates/{}", common::APP_HOST, crate_["id"]))
        .json(&json!({
            "code": "fooz",
            "name": "Fooz crate",
            "version": "0.2",
            "description": "Fooz crate description",
            "rustacean_id": rustacean["id"],
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let crate_: Value = response.json().unwrap();
    assert_eq!(
        crate_,
        json!({
            "id": crate_["id"],
            "rustacean_id": rustacean["id"],
            "code": "fooz",
            "name": "Fooz crate",
            "version": "0.2",
            "description": "Fooz crate description",
            "created_at": crate_["created_at"],
        })
    );

    common::delete_test_crate(&client, crate_);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_update_to_valid_author_of_crate() {
    let client = Client::new();

    let rustacean = common::create_test_rustacean(&client);
    let crate_ = common::create_test_crate(&client, &rustacean);

    let rustacean2 = common::create_test_rustacean(&client);
    let response = client
        .put(format!("{}/crates/{}", common::APP_HOST, crate_["id"]))
        .json(&json!({
            "code": "fooz",
            "name": "Fooz crate",
            "version": "0.2",
            "description": "Fooz crate description",
            "rustacean_id": rustacean2["id"],
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let crate_: Value = response.json().unwrap();
    assert_eq!(
        crate_,
        json!({
            "id": crate_["id"],
            "rustacean_id": rustacean2["id"],
            "code": "fooz",
            "name": "Fooz crate",
            "version": "0.2",
            "description": "Fooz crate description",
            "created_at": crate_["created_at"],
        })
    );

    common::delete_test_crate(&client, crate_);
    common::delete_test_rustacean(&client, rustacean);
    common::delete_test_rustacean(&client, rustacean2);
}

#[test]
fn test_update_to_invalid_author_of_crate() {
    let client = Client::new();

    let rustacean = common::create_test_rustacean(&client);
    let crate_ = common::create_test_crate(&client, &rustacean);

    let response = client
        .put(format!("{}/crates/{}", common::APP_HOST, crate_["id"]))
        .json(&json!({
            "code": "fooz",
            "name": "Fooz crate",
            "version": "0.2",
            "description": "Fooz crate description",
            "rustacean_id": 99999,
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    common::delete_test_crate(&client, crate_);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_delete_crate() {
    let client = Client::new();

    let rustacean = common::create_test_rustacean(&client);
    let crate_ = common::create_test_crate(&client, &rustacean);

    let response = client
        .delete(format!("{}/crates/{}", common::APP_HOST, crate_["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    common::delete_test_rustacean(&client, rustacean);
}
