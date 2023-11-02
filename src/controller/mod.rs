use rocket::Route;
use rocket_okapi::openapi_get_routes;


mod category;
mod post;
mod ads;
mod files;
pub fn routes() -> Vec<Route> {
    openapi_get_routes![
    crate::auth::http_auth,
        category::all,
        category::fisrt,
        category::insert,
        category::delete,

        post::all,
        post::all_lang,
        post::all_limit,
        post::all_lang_views,
        post::first,
        post::insert,
        post::delete,
        post::update,
        post::category,
        post::view,

        files::upload,
        files::all,
        files::delete,

         ads::all,
        ads::fisrt,
        ads::insert,
        ads::delete,

]
}





