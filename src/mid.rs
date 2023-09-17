use rocket::{Request, State};
use rocket::request::{FromRequest, Outcome};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use crate::models::PgAsyncConnection;



pub struct ConnectionManager<'r>(pub &'r PgAsyncConnection);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ConnectionManager<'r>{
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {

        let res = request.guard::<&State<PgAsyncConnection>>().await;
       let rtn = res.map(|s| ConnectionManager(s) );
        rtn

    }
}


#[rocket::async_trait]
impl<'r> OpenApiFromRequest<'r> for ConnectionManager<'r>{
    fn from_request_input(_gen: &mut OpenApiGenerator, _name: String, _required: bool) -> rocket_okapi::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}


