use bytes::Bytes;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::utils::pagination::Response;
use crate::utils::Field;
use crate::{ternary, PaperlessClient};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Document {
    pub id: u64,
    pub title: String,
    pub content: String,

    pub tags: Vec<u64>,
    pub document_type: Option<String>,
    pub correspondent: Option<String>,

    pub created: String,
    pub created_date: Option<String>,
    pub modified: Option<String>,
    pub added: String,

    pub archive_serial_number: Option<String>,
    pub original_file_name: String,
    pub archived_file_name: Option<String>,

    pub notes: Vec<String>,
    //TODO: Add permissions
    pub custom_fields: Vec<Field>,

    #[serde(rename = "__search_hit__")]
    pub search_hit: Option<SearchHit>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Metadata {
    pub original_checksum: String,
    pub original_size: String,
    pub original_mime_type: String,
    pub media_filename: String,
    pub has_archive_version: bool,
    pub original_metadata: Vec<Field>,

    pub archive_checksum: Option<String>,
    pub archive_size: Option<String>,
    pub archive_metadata: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SearchHit {
    pub rank: u64,
    pub score: f64,
}

impl PaperlessClient {
    pub async fn search_documents(
        &self,
        query: &str,
    ) -> Result<Response<Document>, Box<dyn std::error::Error>> {
        let formatted_query = query.replace(' ', "%20");
        let url = format!("{}/documents/?query={}", self.base_url, formatted_query);

        let request_builder = self.prepare_endpoint(Method::GET, url).await?;
        self.call_endpoint(request_builder).await
    }

    pub async fn fetch_documents(&self) -> Result<Response<Document>, Box<dyn std::error::Error>> {
        let url = format!("{}/documents/", self.base_url);

        let request_builder = self.prepare_endpoint(Method::GET, url).await?;
        self.call_endpoint(request_builder).await
    }

    pub async fn fetch_document(
        &self,
        document_id: u64,
    ) -> Result<Document, Box<dyn std::error::Error>> {
        let url = format!("{}/documents/{}/", self.base_url, document_id);

        let request_builder = self.prepare_endpoint(Method::GET, url).await?;
        self.call_endpoint(request_builder).await
    }

    pub async fn download_document(
        &self,
        document_id: u64,
        original: bool,
        download_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/documents/{}/download/", self.base_url, document_id);
        let query_url = format!("{}{}", url, ternary!(original, "?original=true", ""));

        let request_builder = self.prepare_endpoint(Method::GET, query_url).await?;
        self.call_downloadable_endpoint(request_builder, download_path)
            .await
    }

    pub async fn preview_document(
        &self,
        document_id: u64,
        original: bool,
    ) -> Result<Bytes, Box<dyn std::error::Error>> {
        let url = format!("{}/documents/{}/preview/", self.base_url, document_id);
        let query_url = format!("{}{}", url, ternary!(original, "?original=true", ""));

        let request_builder = self.prepare_endpoint(Method::GET, query_url).await?;
        self.call_binary_endpoint(request_builder).await
    }

    pub async fn fetch_document_thumbnail(
        &self,
        document_id: u64,
    ) -> Result<Bytes, Box<dyn std::error::Error>> {
        let url = format!("{}/documents/{}/thumb/", self.base_url, document_id);

        let request_builder = self.prepare_endpoint(Method::GET, url).await?;
        self.call_binary_endpoint(request_builder).await
    }

    pub async fn fetch_document_metadata(
        &self,
        document_id: u64,
    ) -> Result<Metadata, Box<dyn std::error::Error>> {
        let url = format!("{}/documents/{}/metadata/", self.base_url, document_id);

        let request_builder = self.prepare_endpoint(Method::GET, url).await?;
        self.call_endpoint(request_builder).await
    }

    pub async fn fetch_document_notes(
        &self,
        document_id: u64,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let url = format!("{}/documents/{}/notes/", self.base_url, document_id);

        let request_builder = self.prepare_endpoint(Method::GET, url).await?;
        self.call_endpoint(request_builder).await
    }

    pub async fn fetch_document_share_links(
        &self,
        document_id: u64,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let url = format!("{}/documents/{}/share-links/", self.base_url, document_id);

        let request_builder = self.prepare_endpoint(Method::GET, url).await?;
        self.call_endpoint(request_builder).await
    }
}
