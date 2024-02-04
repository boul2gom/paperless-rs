use std::fmt::Debug;

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::PaperlessClient;

/// A struct that represents a response from the Paperless API.
/// Its created by endpoints that return a list of items, and
/// should not be instantiated manually.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Response<T> {
    pub count: u64,
    pub next: Option<String>,
    pub previous: Option<String>,

    pub all: Vec<u64>,
    pub results: Vec<T>,
}

/// A trait that represents a page of items from the Paperless API.
#[async_trait::async_trait]
pub trait Page<R> {
    /// Gets the next page of items, if there is one.
    async fn get_next_page(
        &self,
        client: &PaperlessClient,
    ) -> Result<Response<R>, Box<dyn std::error::Error>>;
    /// Gets the previous page of items, if there is one.
    async fn get_previous_page(
        &self,
        client: &PaperlessClient,
    ) -> Result<Response<R>, Box<dyn std::error::Error>>;

    /// Gets a specific page of items.
    async fn get_page(
        &self,
        page: u64,
        client: &PaperlessClient,
    ) -> Result<Response<R>, Box<dyn std::error::Error>>;
    /// Gets all items from all pages.
    async fn get_all(&self, client: &PaperlessClient)
        -> Result<Vec<R>, Box<dyn std::error::Error>>;
}

#[async_trait::async_trait]
impl<R> Page<R> for Response<R>
where
    R: for<'de> Deserialize<'de> + Clone + Debug + Send + Sync,
{
    async fn get_next_page(
        &self,
        client: &PaperlessClient,
    ) -> Result<Response<R>, Box<dyn std::error::Error>> {
        if let Some(next) = &self.next {
            let request_builder = client
                .prepare_endpoint(Method::GET, next.to_string())
                .await?;
            let response = client.call_endpoint(request_builder).await?;
            return Ok(response);
        }

        Err("No next page".into())
    }

    async fn get_previous_page(
        &self,
        client: &PaperlessClient,
    ) -> Result<Response<R>, Box<dyn std::error::Error>> {
        if let Some(previous) = &self.previous {
            let request_builder = client
                .prepare_endpoint(Method::GET, previous.to_string())
                .await?;
            let response = client.call_endpoint(request_builder).await?;
            return Ok(response);
        }

        Err("No previous page".into())
    }

    async fn get_page(
        &self,
        page: u64,
        client: &PaperlessClient,
    ) -> Result<Response<R>, Box<dyn std::error::Error>> {
        let new_url = if let Some(next) = &self.next {
            let page_index = next.find("page=").ok_or("Malformed next page url")?;
            let new_url = next[..page_index].to_string();
            format!("{}page={}", new_url, page)
        } else if let Some(previous) = &self.previous {
            let page_index = previous
                .find("page=")
                .ok_or("Malformed previous page url")?;
            let new_url = previous[..page_index].to_string();
            format!("{}page={}", new_url, page)
        } else {
            return Err("No next or previous page".into());
        };

        let request_builder = client.prepare_endpoint(Method::GET, new_url).await?;
        let response = client.call_endpoint(request_builder).await?;
        Ok(response)
    }

    async fn get_all(
        &self,
        client: &PaperlessClient,
    ) -> Result<Vec<R>, Box<dyn std::error::Error>> {
        let mut all_items = Vec::new();
        let mut current_page = self.get_page(1, client).await?;

        all_items.extend(current_page.clone().results);

        while let Some(_next) = &current_page.next {
            current_page = current_page.get_next_page(client).await?;

            let results = std::mem::take(&mut current_page.results);
            all_items.extend(results);
        }

        Ok(all_items)
    }
}
