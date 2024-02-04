use crate::utils::pagination::Response;
use crate::PaperlessClient;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub id: u64,
    pub slug: String,
    pub name: String,
    pub colour: u64,
    #[serde(rename = "match")]
    pub match_type: String,
    pub matching_algorithm: u64,
    pub is_insensitive: bool,
    pub is_inbox_tag: bool,
    pub document_count: u64,
    pub owner: u64,
    pub user_can_change: bool,
}

impl PaperlessClient {
    pub async fn fetch_tags(&self) -> Result<Response<Tag>, Box<dyn std::error::Error>> {
        let url = format!("{}/tags/", self.base_url);

        let request_builder = self.prepare_endpoint(Method::GET, url).await?;
        self.call_endpoint(request_builder).await
    }

    pub async fn fetch_tag(&self, tag_id: u64) -> Result<Tag, Box<dyn std::error::Error>> {
        let url = format!("{}/tags/{}/", self.base_url, tag_id);

        let request_builder = self.prepare_endpoint(Method::GET, url).await?;
        self.call_endpoint(request_builder).await
    }
}
