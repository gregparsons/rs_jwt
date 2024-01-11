//! main.rs
//!
//! generate a JWT (for coinbase)
//! https://docs.cloud.coinbase.com/advanced-trade-api/docs/rest-api-auth
//!

use chrono::{Duration, Utc};
use jsonwebtoken::Algorithm;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing_subscriber::{EnvFilter, fmt};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init_env_and_tracing(dot_env_path:&str){

    // println!("dotenv filename: {}", dot_env_path);
    match dotenvy::from_filename(dot_env_path) {
        Ok(_) => {
            // println!(".env found")
        },
        _ => println!(".env not found"),
    }

    // https://github.com/tokio-rs/tracing
    let log_format_layer = fmt::layer();
    // output log as json; see zero2prod pg 104
    // let log_format_layer = BunyanFormattingLayer::new("actix_trace_2022".into(), std::io::stdout);
    tracing_subscriber::registry()
        // .with(JsonStorageLayer)
        .with(log_format_layer)
        .with(EnvFilter::from_default_env())
        .init();

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payload {

    // #[serde(default = None)]
    pub sub: String,
    pub iss: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub nbf: chrono::DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub exp: chrono::DateTime<Utc>,
    pub aud: Vec<String>,
    pub uri: String,

}

impl Payload {

    pub fn new(jwt_key_name: String, iss: &str, service_name: &str, uri: &str) -> Self {

        let expiration = Utc::now() + Duration::seconds(60);

        Payload{
            sub: jwt_key_name,
            iss: iss.to_string(),
            nbf: Utc::now(),
            exp: expiration,
            aud: vec!(service_name.to_string()), // format!("[\\"{}\\"]", service_name),
            uri: uri.to_string(),
        }

    }

    pub fn as_json(&self)-> String {
        json!(self).to_string()
    }

    /// Encode to JWT with a fresh expiration
    pub fn encode_as_jwt(&self) -> Result<String, jsonwebtoken::errors::Error> {
        tracing::debug!("[as_jwt]");
        // let mut header = jsonwebtoken::Header::new(Algorithm::HS512);
        let mut header = jsonwebtoken::Header::new(Algorithm::ES256);

        // key name
        header.kid = Some(self.sub.clone());

        // nonce??
        // https://github.com/andrewbaxter/fork-jsonwebtoken/blob/acme-and-jws/src/header.rs
        header.nonce = Some("a0db9fa7f11c40032eebfda61e9a6d939c7f31704f132610fe2573604edeaa6f".to_string());

        // let header = jsonwebtoken::Header::new(Algorithm::ES256);
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET environment variable not found");

        // fake key from https://8gwifi.org/jwsgen.jsp
        // or generate with openssl ecparam -genkey -noout -name prime256v1     | openssl pkcs8 -topk8 -nocrypt -out ec-private.pem
        // per https://docs.rs/jsonwebtoken/8.3.0/jsonwebtoken/struct.EncodingKey.html#method.from_ec_pem


        // let jwt_secret =
        //     r#""#.to_string();

            // this format does not work
            // r#"-----BEGIN EC PRIVATE KEY-----...-----END EC PRIVATE KEY-----"#.to_string();

            // fake, working key
            // r#"-----BEGIN PRIVATE KEY-----MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQg22YNdV0SQ59aHPs3Kp7Op5KWWPTqLmsoRzeoip4YyhWhRANCAARwUo2BKMPVLQXuwMNere1ynWh44E2JMQCWWUjZ6WtsS2XKu3VhVE5bm0hQNDRp0Sf+Ne7qnYz4Yg6oXVeyyQ6/-----END PRIVATE KEY-----"#.to_string();



        let jwt_secret = jwt_secret.into_bytes();
        let encode_result = jsonwebtoken::encode(
            &header,
            self,
            // &jsonwebtoken::EncodingKey::from_secret(&jwt_secret),
            &jsonwebtoken::EncodingKey::from_ec_pem(&jwt_secret).unwrap(),
        );
        tracing::debug!("[as_jwt] json_web_token encode result: {:?}", &encode_result);
        encode_result
    }
}

fn main() {

    init_env_and_tracing(concat!(env!("CARGO_MANIFEST_DIR"), "/.env"));

    let jwt_key_name = std::env::var("JWT_KEY_NAME").expect("JWT_KEY_NAME environment variable not found");
    let service_name = "retail_rest_api_proxy";
    let uri = "GET api.coinbase.com/api/v3/brokerage/accounts";

    let p = Payload::new(jwt_key_name, "coinbase-cloud", service_name, uri);
    let jwt = p.encode_as_jwt().unwrap();

    println!("{}", &jwt.to_string());
}
