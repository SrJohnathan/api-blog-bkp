use dotenvy::dotenv;
use rocket_cors::CorsOptions;
use rocket_okapi::rapidoc::{GeneralConfig, HideShowConfig, make_rapidoc, RapiDocConfig, Theme, UiConfig};
use rocket_okapi::settings::UrlObject;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};






pub mod models;
pub mod repository;
pub mod controller;

pub mod auth;
 pub mod schema;

pub mod s3;

pub mod mid;

#[tokio::main]
async fn main() {

    if cfg!(debug_assertions) {
        dotenv().unwrap();
    }

   //
  //  let mut channel: (Sender<String>, Receiver<String>) = mpsc::channel(100);

    let db =
        models::connection("DATABASE_URL".to_string()).await.unwrap();

    let _ = rocket::build()
        .attach(CorsOptions::default().to_cors().unwrap())
        .manage(db)
        .mount("/", controller::routes())
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../openapi.json")],
                    ..Default::default()
                },
                ui: UiConfig {
                    theme: Theme::Dark,
                    ..Default::default()

                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
        .launch()
        .await;

}
