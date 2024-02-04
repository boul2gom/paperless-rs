use crate::utils::pagination::Response;
use crate::PaperlessClient;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentType {
    pub id: u64,
    pub slug: String,
    pub name: String,
    #[serde(rename = "match")]
    pub match_type: String,
    pub matching_algorithm: u64,
    pub is_insensitive: bool,
    pub document_count: u64,
    pub owner: u64,
    pub user_can_change: bool,
}

impl PaperlessClient {
    pub async fn fetch_document_types(
        &self,
    ) -> Result<Response<DocumentType>, Box<dyn std::error::Error>> {
        let url = format!("{}/document_types/", self.base_url);

        let request_builder = self.prepare_endpoint(Method::GET, url).await?;
        self.call_endpoint(request_builder).await
    }

    pub async fn fetch_document_type(
        &self,
        document_type_id: u64,
    ) -> Result<DocumentType, Box<dyn std::error::Error>> {
        let url = format!("{}/document_types/{}/", self.base_url, document_type_id);

        let request_builder = self.prepare_endpoint(Method::GET, url).await?;
        self.call_endpoint(request_builder).await
    }
}
