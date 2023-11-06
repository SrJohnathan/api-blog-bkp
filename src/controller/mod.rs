use rocket::Route;
use rocket_okapi::openapi_get_routes;


pub mod category;
pub mod post;
pub mod ads;
pub mod files;

pub mod settings;
pub fn routes() -> Vec<Route> {
    openapi_get_routes![
    crate::auth::http_auth,
        category::all,
        category::fisrt,
        category::fisrt_name,
        category::insert,
        category::delete,

        post::all,
        post::all_lang,
        post::all_limit,
        post::all_lang_views,
        post::first,
        post::insert_no_file,
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

        settings::all,
        settings::fisrt,
        settings::fisrt_name,
        settings::insert,
        settings::delete,

]
}





