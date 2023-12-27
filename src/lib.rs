pub mod authorization;
pub mod endpoint;
pub mod utils;

use crate::authorization::{AuthorizationType, CertificateType};
use reqwest::{Client, Method, RequestBuilder};
use serde::Deserialize;

pub struct PaperlessClient {
    pub client: Client,

    pub base_url: String,
    pub auth_type: AuthorizationType,
}

impl PaperlessClient {
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

    pub async fn prepare_endpoint(
        &mut self,
        method: Method,
        url: String,
    ) -> Result<RequestBuilder, Box<dyn std::error::Error>> {
        let (header, value) = self.auth_type.as_header();

        let request_builder = self.client.request(method, url);
        Ok(request_builder.header(header, value))
    }

    pub async fn call_endpoint<T>(
        &mut self,
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
