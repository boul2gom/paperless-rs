mod correspondents;
mod document_types;
pub mod documents;
mod tags;

use crate::PaperlessClient;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u64,
    pub task_id: String,
    pub task_file_name: String,

    pub date_created: String,
    pub date_done: Option<String>,

    #[serde(rename = "type")]
    pub task_type: String,
    pub status: String,
    pub result: Option<String>,
    pub acknowledged: bool,
    pub related_document: Option<String>,
}

impl PaperlessClient {
    pub async fn fetch_logs_producers(
        &mut self,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let url = format!("{}/logs/", self.base_url);

        let request_builder = self.prepare_endpoint(Method::GET, url).await?;
        self.call_endpoint(request_builder).await
    }

    pub async fn fetch_logs(
        &mut self,
        producer: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let url = format!("{}/logs/{}/", self.base_url, producer);

        let request_builder = self.prepare_endpoint(Method::GET, url).await?;
        self.call_endpoint(request_builder).await
    }

    pub async fn fetch_tasks(&mut self) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
        let url = format!("{}/tasks/", self.base_url);

        let request_builder = self.prepare_endpoint(Method::GET, url).await?;
        self.call_endpoint(request_builder).await
    }

    pub async fn fetch_task(&mut self, task_id: &str) -> Result<Task, Box<dyn std::error::Error>> {
        let url = format!("{}/tasks/{}/", self.base_url, task_id);

        let request_builder = self.prepare_endpoint(Method::GET, url).await?;
        self.call_endpoint(request_builder).await
    }
}
