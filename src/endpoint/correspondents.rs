use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::utils::pagination::Response;
use crate::PaperlessClient;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Correspondent {
    pub id: u64,
    pub slug: String,
    pub name: String,
    #[serde(rename = "match")]
    pub match_type: String,
    pub matching_algorithm: u64,
    pub is_insensitive: bool,
    pub document_count: u64,
    pub last_correspondence: Option<String>,
    pub owner: u64,
    pub user_can_change: bool,
}

impl PaperlessClient {
    pub async fn fetch_correspondents(
        &self,
    ) -> Result<Response<Correspondent>, Box<dyn std::error::Error>> {
        let url = format!("{}/correspondents/", self.base_url);

        let request_builder = self.prepare_endpoint(Method::GET, url).await?;
        self.call_endpoint(request_builder).await
    }

    pub async fn fetch_correspondent(
        &self,
        correspondent_id: u64,
    ) -> Result<Correspondent, Box<dyn std::error::Error>> {
        let url = format!("{}/correspondents/{}/", self.base_url, correspondent_id);

        let request_builder = self.prepare_endpoint(Method::GET, url).await?;
        self.call_endpoint(request_builder).await
    }
}
