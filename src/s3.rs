extern crate rusoto_core;
extern crate rusoto_s3;

use std::str::FromStr;
use std::time::Duration;

use rusoto_core::credential::{AwsCredentials, StaticProvider};
use rusoto_core::request::HttpClient;
use rusoto_core::{Region, RusotoError};
use rusoto_s3::util::{PreSignedRequest, PreSignedRequestOption};
use rusoto_s3::{S3, S3Client, GetObjectRequest, ListObjectsV2Request, GetObjectTaggingRequest, PutObjectRequest, Tag, Tagging, PutObjectTaggingRequest, PutObjectTaggingError, PutObjectTaggingOutput, PutObjectOutput, PutObjectError};
use serde::Serialize;


mod util {
    use std::env::VarError;

    pub fn get_env_var_value(var_name: &str) -> Result<String, VarError> {
        match  std::env::var(var_name) {
            Ok(x) => { Ok(x) }
            Err(e) => { Err(e)  }
        }
    }
    pub fn get_env_var_value_or_default(var_name: &str, default: Option<String>) -> String {
        get_env_var_value(var_name).unwrap_or(
            default.unwrap_or(
                "".to_string()
            )
        )
    }
}


#[derive(Serialize)]
pub struct S3Object {
    file_name: String,
    presigned_url: String,
    tags: String,
    e_tag: String, // AWS generated MD5 checksum hash for object
    is_filtered: bool,
}

impl S3Object {
    pub fn new(
        file_name: String,
        e_tag: String,
        categories_string: String,
        presigned_url: String,
        is_filtered: bool,
    ) -> S3Object {
        S3Object {
            file_name: file_name,
            e_tag: e_tag,
            tags: categories_string,
            presigned_url: presigned_url,
            is_filtered: is_filtered,
        }
    }

    pub fn is_hidden(&self) -> bool {
        self.is_filtered
    }
}

#[derive(Serialize)]
pub struct BucketContents {
    data: Vec<S3Object>,
}

impl BucketContents {
    pub fn new(contents: Vec<S3Object>) -> BucketContents {
        BucketContents {
            data: contents,
        }
    }

    pub fn empty_bucket() -> BucketContents {
        BucketContents {
            data: Vec::new(),
        }
    }
}

pub struct S3FileManager {
    s3_client: S3Client,
    bucket_name: String,
    pub access_key: String,
    secret_key: String,
    region: Region,
}

impl S3FileManager {
    pub fn new(
        region_str: Option<String>,
        access_key: Option<String>,
        secret_key: Option<String>,
        bucket_name: Option<String>,
    ) -> S3FileManager {

        let bucket = bucket_name.unwrap_or(
            util::get_env_var_value_or_default("BUCKET_NAME", None)
        );
        let key = access_key.unwrap_or(
            util::get_env_var_value_or_default("AWS_ACCESS_KEY_ID", None)
        );
        let secret = secret_key.unwrap_or(
            util::get_env_var_value_or_default("AWS_SECRET_ACCESS_KEY", None)
        );
        let credentials = S3FileManager::_create_credentials(
            key.clone(),
            secret.clone(),
        );
        let credentials_provider = StaticProvider::from(credentials);

        let region_name = region_str.unwrap_or(
            util::get_env_var_value_or_default("AWS_DEFAULT_REGION", None)
        );
        let resource_region = Region::from_str(&region_name).unwrap_or(Region::UsEast1);
        let s3_client = S3Client::new_with(
            HttpClient::new().expect("Failed to create HTTP client"),
            credentials_provider,
            resource_region.clone(),
        );

        return S3FileManager {
            access_key: key,
            secret_key: secret,
            bucket_name: bucket,
            s3_client,
            region: resource_region,
        }
    }

    fn _create_credentials(key: String, secret: String) -> AwsCredentials {
        AwsCredentials::new(
            key, secret, None, None
        )
    }

    fn create_credentials(&self) -> AwsCredentials {
        S3FileManager::_create_credentials(
            self.access_key.clone(),
            self.secret_key.clone(),
        )
    }

    pub fn get_presigned_url_for_file(&self, file_name: String) -> String {
        let get_obj_req = GetObjectRequest {
            bucket: self.bucket_name.clone(),
            key: file_name,
            ..Default::default()
        };
        let options = PreSignedRequestOption {
            expires_in: Duration::from_secs(60 * 30),
        };
        get_obj_req.get_presigned_url(
            &self.region,
            &self.create_credentials(),
            &options
        )
    }

    pub async fn get_bucket_contents(&self) -> Option<Vec<rusoto_s3::Object>> {
        let list_objs_req = ListObjectsV2Request {
            bucket: self.bucket_name.clone(),
            ..Default::default()
        };
        self.s3_client
            .list_objects_v2(list_objs_req)
            .await
            .expect("failed to list objects v2")
            .contents
    }

    pub async fn get_tags_on_file(&self, file_name: String) -> Vec<Tag> {
        let get_obj_tag_req = GetObjectTaggingRequest {
            bucket: self.bucket_name.clone(),
            key: file_name,
            ..Default::default()
        };
        self.s3_client
            .get_object_tagging(get_obj_tag_req)
            .await
            .expect("failed to get tag list")
            .tag_set
    }

    pub async fn put_file_in_bucket(&self, file_name: String, file_data: Vec::<u8>) -> Result<String, String> {

        let put_request = PutObjectRequest {
            bucket: self.bucket_name.clone(),
            key: file_name.clone(),
            body: Some(file_data.into()),
            ..Default::default()
        };

      match  self.s3_client
            .put_object(put_request)
          .await {
              Ok(x) => {

              Ok(self.get_presigned_url_for_file(file_name))

              }
              Err(e) => Err(e.to_string())
      }
    }

    pub async fn put_tags_on_file(&self, file_name: String, tag_names_and_vals: Vec<(String, String)>) -> Result<String, String> {

        let mut vec_of_tags = Vec::new();
        for tag in tag_names_and_vals {
            vec_of_tags.push(
                Tag {
                    key: tag.0,
                    value: tag.1,
                }
            )
        }

        let tag_set = Tagging {
            tag_set: vec_of_tags
        };

        let put_tagging_request = PutObjectTaggingRequest {
            bucket: self.bucket_name.clone(),
            key: file_name.clone(),
            tagging: tag_set,
            ..Default::default()
        };

      match    self.s3_client
            .put_object_tagging(put_tagging_request)
            .await {
          Ok(x) => {

              Ok(self.get_presigned_url_for_file(file_name))

          }
          Err(e) => Err(e.to_string())
      }

    }
}

pub fn list_bucket_contents() {

}