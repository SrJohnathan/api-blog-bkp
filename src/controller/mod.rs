use rocket::Route;
use rocket_okapi::openapi_get_routes;

mod controller_lead;
mod category;


pub fn routes() -> Vec<Route> {
    openapi_get_routes![
    crate::auth::http_auth,
        category::all,
        category::fisrt,
        category::insert,
        category::delete


]
}




