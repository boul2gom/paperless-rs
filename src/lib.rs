pub mod authorization;
pub mod endpoint;
pub mod utils;

use crate::authorization::{AuthorizationType, CertificateType};
use reqwest::{Client, Method, RequestBuilder};
use serde::Deserialize;

/// This is the library entrypoint. From this struct you can interact with the Paperless API.
/// You can create a new instance of this struct with the `new` method.
///
/// # Example
///
/// ```rust
/// use paperless_rs::PaperlessClient;
/// use paperless_rs::authorization::{AuthorizationType, Credentials};
///
/// #[tokio::main]
/// async fn main() {
///     let credentials = Credentials::new(String::from("username"), String::from("password"), None);
///     let auth_type = AuthorizationType::Basic(credentials);
///
///    let mut client = PaperlessClient::new("https://paperless.example.com", auth_type, None).await.unwrap();
/// }
/// ```
pub struct PaperlessClient {
    pub client: Client,

    pub base_url: String,
    pub auth_type: AuthorizationType,
}

impl PaperlessClient {
    /// Creates a new instance of the PaperlessClient struct.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base url of the Paperless instance. This should be the url without the `/api` part.
    /// * `auth_type` - The authorization type to use. This can be either `Basic` or `Token`.
    /// * `certificate_path` - The path to the certificate file. This is only needed if you are using a self-signed certificate.
    pub async fn new(
        base_url: &str,
        auth_type: AuthorizationType,
        certificate_path: Option<CertificateType>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut builder = Client::builder();

        if let Some(certificate_path) = certificate_path {
            let parsed_certificate = certificate_path.as_certificate().await?;

            builder = builder
                .danger_accept_invalid_certs(true)
                .add_root_certificate(parsed_certificate);
        }

        let client = builder.build()?;
        let base_url = format!("{}/api", base_url);

        Ok(PaperlessClient {
            client,
            base_url,
            auth_type,
        })
    }

    /// Prepares an endpoint for a request.
    ///
    /// # Arguments
    ///
    /// * `method` - The HTTP method to use for the request.
    /// * `url` - The url to call.
    pub async fn prepare_endpoint(
        &self,
        method: Method,
        url: String,
    ) -> Result<RequestBuilder, Box<dyn std::error::Error>> {
        let (header, value) = self.auth_type.as_header();

        let request_builder = self.client.request(method, url);
        Ok(request_builder.header(header, value))
    }

    /// Calls an endpoint and returns the response.
    ///
    /// # Arguments
    ///
    /// * `request_builder` - The request builder to use for the request.
    pub async fn call_endpoint<T>(
        &self,
        request_builder: RequestBuilder,
    ) -> Result<T, Box<dyn std::error::Error>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let response = request_builder.send().await?;
        let status = response.status();

        if status.is_success() {
            let value = response.json::<T>().await?;
            return Ok(value);
        }

        let error = response.text().await?;
        Err(error.into())
    }
}
