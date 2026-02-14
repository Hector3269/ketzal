use crate::app::http::controllers::user_controller::UserController;
use ketzal::{routes_web, Route};

routes_web! {
    Route::get("/", UserController::login);
}
