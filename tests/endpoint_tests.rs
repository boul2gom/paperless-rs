mod setup;

use std::path::Path;

use tokio::fs;

use crate::setup::PAPERLESS_CLIENT;

#[tokio::test]
pub async fn test_search_documents() {
    let documents = PAPERLESS_CLIENT.search_documents("Sample document n°1").await.unwrap();
    
    assert_eq!(documents.count, 1);
    assert_eq!(documents.results.len(), 1);
    assert_eq!(documents.results[0].id, 1);
    assert_eq!(documents.results[0].title, "Sample document n°1");
}

#[tokio::test]
pub async fn test_fetch_documents() {
    let documents = PAPERLESS_CLIENT.fetch_documents().await.unwrap();
    
    assert_eq!(documents.count, 10);
    assert_eq!(documents.results.len(), 10);

    for (id, document) in documents.results.iter().enumerate() {
        assert_eq!(document.id, id as u64 + 1);
        assert_eq!(document.title, format!("Sample document n°{}", id + 1));
    }
}

#[tokio::test]
pub async fn test_fetch_document() {
    for id in 1..=10 {
        let document = PAPERLESS_CLIENT.fetch_document(id).await.unwrap();
        assert_eq!(document.id, id);
        assert_eq!(document.title, format!("Sample document n°{}", id));
    }
}

#[tokio::test]
pub async fn test_download_document() {
    for id in 1..=10 {
        let download_path = format!("/tmp/sample-document-{}.pdf", id);
        let path = Path::new(&download_path);

        PAPERLESS_CLIENT.download_document(id, false, &download_path).await.unwrap();
        assert!(path.exists());

        fs::remove_file(&download_path).await.unwrap();
        assert!(!path.exists());
    }
}