
use rocket::{Data, Request, State};
use rocket::data::{FromData, ToByteUnit};

use rocket::form::{DataField, FromFormField};



use rocket::request::{FromParam, FromRequest, Outcome};
use rocket_multipart_form_data::{ MultipartFormData, MultipartFormDataField, MultipartFormDataOptions};

use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::{okapi,};
use rocket_okapi::okapi::openapi3::{MediaType, RequestBody};

use rocket_okapi::request::{OpenApiFromData, OpenApiFromRequest, RequestHeaderInput};
use schemars::gen::SchemaGenerator;
use schemars::JsonSchema;
use schemars::schema::{InstanceType,Schema, SchemaObject,};



use tokio::io::AsyncReadExt;
use crate::models::new_models::{DataFile, FormNewPost, Language, TipoPost};
use crate::models::PgAsyncConnection;



pub struct ConnectionManager<'r>(pub &'r PgAsyncConnection);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ConnectionManager<'r> {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let res = request.guard::<&State<PgAsyncConnection>>().await;
        let rtn = res.map(|s| ConnectionManager(s));
        rtn
    }
}
#[rocket::async_trait]
impl<'r> OpenApiFromRequest<'r> for ConnectionManager<'r> {
    fn from_request_input(_gen: &mut OpenApiGenerator, _name: String, _required: bool) -> rocket_okapi::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}
#[rocket::async_trait]
impl<'r> FromFormField<'r> for TipoPost {
    async fn from_data(field: DataField<'r, '_>) -> rocket::form::Result<'r, Self> {
        let mut value = String::new();

        if let Err(e) = field.data.open(2.megabytes()).read_to_string(&mut value).await {
            return Err(rocket::form::Errors::from(rocket::form::Error::validation(e.to_string())));
        }
        let tipo = match value.as_str() {
            "Video" => TipoPost::Video,
            "Texto" => TipoPost::Texto,
            "Audio" => TipoPost::Audio,
            "Html" => TipoPost::Html,
            _ => return Err(rocket::form::Errors::from(rocket::form::Error::validation("Valor de TipoPost inválido"))),
        };
        Ok(tipo)
    }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for FormNewPost {
    type Error = ();

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> rocket::data::Outcome<'r, Self> {

        let  options = MultipartFormDataOptions::with_multipart_form_data_fields(
            vec![
                MultipartFormDataField::raw("photo").size_limit(10_000_000),
                MultipartFormDataField::text("titulo"),
                MultipartFormDataField::text("categoria_id"),
                MultipartFormDataField::text("tipo"),
                MultipartFormDataField::text("conteudo"),
                MultipartFormDataField::text("language"),
            ],
        );


        let mut form = FormNewPost {
            titulo: "".to_string(),
            categoria_id: 0,
            tipo: TipoPost::Texto,
            language : Language::Pt,
            conteudo: None,
            photo: DataFile(vec![]),
        } ;

        let mut multipart_form = match    MultipartFormData::parse(req.content_type().unwrap(), data, options).await {
            Ok(x) => x,
            Err(x) => {
                println!("{}",x);
            todo!()
            }
        };

        let photo = multipart_form.raw.remove("photo");
        let title = multipart_form.texts.remove("titulo");
        let categoria_id = multipart_form.texts.remove("categoria_id");
        let  conteudo = multipart_form.texts.remove("conteudo");
        let  tipo = multipart_form.texts.remove("tipo");
        let  language = multipart_form.texts.remove("language");



        if let Some( mut   file) = photo {
            let file_data = file.remove(0);
            form.photo = DataFile(file_data.raw);
        }
        if let Some( mut value) = title {
            let v = value.remove(0);
            form.titulo = v.text
        }
        if let Some( mut value) = categoria_id {
            let v = value.remove(0);
            form.categoria_id = v.text.parse().unwrap()
        }

        if let Some( mut value) = conteudo {
            let v = value.remove(0);
            form.conteudo = Some( v.text )
        }

        if let Some( mut value) = tipo {
            let v = value.remove(0);

            match v.text.as_str() {
                "Video" => { form.tipo = TipoPost::Video }
                "Texto" => { form.tipo = TipoPost::Texto }
                "Audio" => {form.tipo = TipoPost::Audio }
                "Html" => { form.tipo = TipoPost::Html }
                _ => { form.tipo = TipoPost::Texto  }
            }


        }

        if let Some( mut value) = language {
            let v = value.remove(0);

              match  v.text.as_str() {
                 "Pt" => {  form.language = Language::Pt }
                  "En" => {  form.language = Language::En }
                  "Es" => {  form.language = Language::Es }
                  "Fr" => {  form.language = Language::Fr }
                  _ => { form.language = Language::Pt  }
              }


        }

        rocket::data::Outcome::Success(form)
    }
}

#[rocket::async_trait]
impl<'v> FromFormField<'v> for DataFile {
    async fn from_data(field: DataField<'v, '_>) -> rocket::form::Result<'v, Self> {
        let stream = field.data.open(u32::MAX.bytes());
        let bytes = stream.into_bytes().await?;
      //  Ok(DataFile {
           // file_name: Some(field.file_name.unwrap().as_str().unwrap().to_string()),
            // content_type: field.content_type,
       // })
        Ok(DataFile(bytes.value))
    }
}

#[rocket::async_trait]
impl<'r> OpenApiFromData<'r> for FormNewPost {
    fn request_body(gen: &mut OpenApiGenerator) -> rocket_okapi::Result<RequestBody> {
        let mut ty = MediaType::default();
        ty.schema = Some( gen.json_schema::<FormNewPost>());
        let mut m = okapi::Map::<String, MediaType>::new();
        m.insert("multipart/form-data".to_string(),ty);
        Ok(RequestBody {
            description: None,
            required: false,
            content: m,
            ..Default::default()
        })
    }
}


impl JsonSchema for DataFile {
    fn schema_name() -> String {
        "DataFile".to_string()
    }

    fn json_schema(_gen: &mut SchemaGenerator) -> Schema {
        let file_schema = SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            format: Some("binary".to_string()),
            ..Default::default()
        };
      //  schema.definitions.insert("photo".to_string(),file_schema.into());

        Schema::Object(file_schema)


    }
}


impl<'r> FromParam<'r> for Language {
    type Error = String;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        match param {
            "Pt" => Ok(Language::Pt),
            "En" => Ok(Language::En),
            "Es" => Ok(Language::Es),
            "Fr" => Ok(Language::Fr),
            _ => Err(format!("Invalid language: {}", param)),
        }
    }
}