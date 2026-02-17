use ketzal::validate;
use ketzal::{Request, Response};

pub struct UserController;

impl UserController {
    pub async fn login(_req: Request) -> Response {
        Response::ok("Hello, World!")
    }

    pub async fn store(req: Request) -> Response {
        let validated = validate!(req => {
            "name" => "required|string|max:255",
            "email" => "required|email",
            "password" => "required|min:8|confirmed",
        });

        let safe = validated.except(["password"]).all();

        Response::json(safe)
    }
}
