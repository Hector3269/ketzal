use ketzal::{Request, Response};

pub struct UserController;

impl UserController {
    pub async fn login(_req: Request) -> Response {
        Response::ok("Hello, World!")
    }
}
