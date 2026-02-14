use http::Method;
use ketzal_router::{Route, Router};

#[derive(Clone, Debug)]
struct TestRequest;

#[derive(Clone, Debug, PartialEq)]
struct TestResponse {
    status: u16,
}

impl TestResponse {
    fn ok() -> Self {
        Self { status: 200 }
    }
}

#[test]
fn can_register_get_route() {
    let mut router = Router::<TestRequest, TestResponse>::new();

    let route = Route::get("/users", |_req| async { TestResponse::ok() });

    router.register(route);

    let found = router.find(&Method::GET, "/users");
    assert!(found.is_some());
}

#[test]
fn route_name_is_stored() {
    let mut router = Router::<TestRequest, TestResponse>::new();

    router.register(Route::get("/users", |_req| async { TestResponse::ok() }).name("users.index"));

    let url = router.url("users.index");
    assert_eq!(url, Some(&"/users".to_string()));
}

#[test]
fn different_methods_do_not_conflict() {
    let mut router = Router::<TestRequest, TestResponse>::new();

    router.register(Route::get("/users", |_req| async { TestResponse::ok() }));

    let found = router.find(&Method::POST, "/users");
    assert!(found.is_none());
}

#[test]
fn unknown_route_returns_none() {
    let router = Router::<TestRequest, TestResponse>::new();

    let found = router.find(&Method::GET, "/missing");
    assert!(found.is_none());
}

#[test]
fn can_register_post_route() {
    let mut router = Router::<TestRequest, TestResponse>::new();

    let route = Route::post("/users", |_req| async { TestResponse::ok() });

    router.register(route);

    let found = router.find(&Method::POST, "/users");
    assert!(found.is_some());
}

#[test]
fn can_register_put_route() {
    let mut router = Router::<TestRequest, TestResponse>::new();

    let route = Route::put("/users", |_req| async { TestResponse::ok() });

    router.register(route);

    let found = router.find(&Method::PUT, "/users");
    assert!(found.is_some());
}

#[test]
fn can_register_delete_route() {
    let mut router = Router::<TestRequest, TestResponse>::new();

    let route = Route::delete("/users", |_req| async { TestResponse::ok() });

    router.register(route);

    let found = router.find(&Method::DELETE, "/users");
    assert!(found.is_some());
}

#[test]
fn can_register_multiple_routes() {
    let mut router = Router::<TestRequest, TestResponse>::new();

    router.register(Route::get("/users", |_req| async { TestResponse::ok() }));
    router.register(Route::post("/users", |_req| async { TestResponse::ok() }));
    router.register(Route::get("/posts", |_req| async { TestResponse::ok() }));

    assert!(router.find(&Method::GET, "/users").is_some());
    assert!(router.find(&Method::POST, "/users").is_some());
    assert!(router.find(&Method::GET, "/posts").is_some());
    assert!(router.find(&Method::DELETE, "/users").is_none());
}

#[test]
fn empty_router_has_no_routes() {
    let router = Router::<TestRequest, TestResponse>::new();

    assert!(router.find(&Method::GET, "/").is_none());
}
