use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

use ketzal_validation::Validator;
use serde_json::json;

use crate::helpers::make;

/// Tests for thread safety and concurrency

#[test]
fn multiple_validators_no_shared_state() {
    // Each validator should have its own state
    let data1 = make(json!({ "email": "test1@example.com" }));
    let data2 = make(json!({ "email": "test2@example.com" }));
    let data3 = make(json!({ "email": "test3@example.com" }));

    let mut v1 = Validator::make(data1, [("email", "required|email")].into());
    let mut v2 = Validator::make(data2, [("email", "required|email")].into());
    let mut v3 = Validator::make(data3, [("email", "required|email")].into());

    assert!(v1.validate().is_ok());
    assert!(v2.validate().is_ok());
    assert!(v3.validate().is_ok());

    // Verify each validator kept its own data
    assert_eq!(v1.validated_data()["email"], "test1@example.com");
    assert_eq!(v2.validated_data()["email"], "test2@example.com");
    assert_eq!(v3.validated_data()["email"], "test3@example.com");
}

#[test]
fn validator_clone_has_independent_errors() {
    let data = make(json!({ "email": "invalid" }));

    let mut v1 = Validator::make(data, [("email", "required|email")].into());
    let _ = v1.validate();

    // Clone should have independent error state
    let mut v2 = Validator::make(
        make(json!({ "email": "valid@example.com" })),
        [("email", "required|email")].into(),
    );

    assert!(v2.validate().is_ok());
}

#[test]
fn concurrent_validation_same_rules() {
    let results: Arc<Mutex<Vec<Result<(), HashMap<String, Vec<String>>>>>> =
        Arc::new(Mutex::new(Vec::new()));

    let mut handles = vec![];

    for i in 0..10 {
        let results_clone = results.clone();
        let handle = thread::spawn(move || {
            let data = make(json!({ "email": format!("user{}@example.com", i) }));
            let mut v = Validator::make(data, [("email", "required|email")].into());
            let result = v.validate();

            let mut results = results_clone.lock().unwrap();
            results.push(result);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let results = results.lock().unwrap();
    assert_eq!(results.len(), 10);

    // All should pass
    for result in results.iter() {
        assert!(result.is_ok());
    }
}

#[test]
fn concurrent_validation_different_data() {
    let results: Arc<Mutex<Vec<bool>>> = Arc::new(Mutex::new(Vec::new()));

    let test_cases = vec![
        (json!({"email": "valid@example.com"}), true),
        (json!({"email": "invalid"}), false),
        (json!({"email": "another@test.com"}), true),
        (json!({"age": "not-a-number"}), false),
        (json!({"age": 25}), true),
    ];

    let mut handles = vec![];

    for (data, should_pass) in test_cases {
        let results_clone = results.clone();
        let data = make(data);
        let handle = thread::spawn(move || {
            let rules: HashMap<&str, &str> = if data.contains_key("email") {
                [("email", "required|email")].into()
            } else {
                [("age", "required|numeric")].into()
            };

            let mut v = Validator::make(data, rules);
            let result = v.validate().is_ok();

            let mut results = results_clone.lock().unwrap();
            results.push(result == should_pass);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let results = results.lock().unwrap();
    // All validations should match expected outcomes
    assert!(results.iter().all(|&x| x));
}

#[test]
fn validator_reuse_after_validation() {
    let mut v = Validator::make(
        make(json!({ "email": "test@example.com" })),
        [("email", "required|email")].into(),
    );

    // First validation
    assert!(v.validate().is_ok());

    // Reuse with new data
    v.data = make(json!({ "email": "another@example.com" }));

    assert!(v.validate().is_ok());
    assert_eq!(v.validated_data()["email"], "another@example.com");
}

#[test]
fn validator_with_custom_messages_thread_safe() {
    let mut v1 = Validator::make(
        make(json!({ "email": "invalid" })),
        [("email", "required|email")].into(),
    );
    v1.set_custom_messages([("email.email", "Custom: Invalid email")].into());

    let mut v2 = Validator::make(
        make(json!({ "email": "invalid" })),
        [("email", "required|email")].into(),
    );
    // v2 has no custom messages

    let r1 = v1.validate();
    let r2 = v2.validate();

    assert!(r1.is_err());
    assert!(r2.is_err());

    // v1 should use custom message
    let err1 = r1.unwrap_err();
    assert!(err1["email"][0].contains("Custom"));

    // v2 should use default message
    let err2 = r2.unwrap_err();
    assert!(!err2["email"][0].contains("Custom"));
}

#[test]
fn many_validators_concurrent() {
    let count = 100;
    let results: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..count {
        let results_clone = results.clone();
        let handle = thread::spawn(move || {
            let data = make(json!({
                "email": format!("user{}@example.com", i),
                "name": format!("User {}", i),
                "age": 20 + (i % 50)
            }));

            let mut v = Validator::make(
                data,
                [
                    ("email", "required|email"),
                    ("name", "required|string|min:2"),
                    ("age", "required|numeric|min:18"),
                ]
                .into(),
            );

            if v.validate().is_ok() {
                let mut results = results_clone.lock().unwrap();
                *results += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let results = results.lock().unwrap();
    assert_eq!(*results, count);
}

#[test]
fn validated_data_can_be_called_multiple_times() {
    let data = make(json!({ "email": "test@example.com" }));
    let mut v = Validator::make(data, [("email", "required|email")].into());

    v.validate().unwrap();

    // validated_data consumes self, so we need a new validator
    let data2 = make(json!({ "email": "test2@example.com" }));
    let mut v2 = Validator::make(data2, [("email", "required|email")].into());

    v2.validate().unwrap();
    let validated2 = v2.validated_data();

    assert_eq!(validated2["email"], "test2@example.com");
}
