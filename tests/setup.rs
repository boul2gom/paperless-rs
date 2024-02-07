use paperless_rs::PaperlessClient;

#[cfg(test)]
lazy_static::lazy_static! {
    pub static ref PAPERLESS_CLIENT: PaperlessClient = {
        let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
        runtime.block_on(setup_tests())
    };
}

#[cfg(test)]
pub async fn setup_tests() -> PaperlessClient {
    use paperless_rs::authorization::{AuthorizationType, Credentials};
    
    println!("Setting up tests...");
    let credentials = Credentials::new("boul2gom".to_string(), "ci-env-password".to_string());
    let auth_type = AuthorizationType::Basic(credentials);

    let client = PaperlessClient::new("http://127.0.0.1:8080", auth_type, None).await;
    let client = client.expect("Failed to create Paperless client, please check your running environment...");

    for id in 1..=10 {
        let content = format!("Sample document n°{}", id);
        let path = format!("sample-document-{}.pdf", id);
        self::create_pdf(&content, &content, &path).expect("Failed to create PDF");
    }

    client
}

#[cfg(test)]
pub fn create_pdf(title: &str, content: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::{fs::File, io::BufWriter};

    use printpdf::{Mm, PdfDocument};

    let file_name = format!("/usr/src/paperless/consume/{}", path);
    let (doc, _, _) = PdfDocument::new(title, Mm(297.0), Mm(210.0), content);

    let file = File::create(file_name)?;
    doc.save(&mut BufWriter::new(file))?;

    //Wait for the documents to be indexed
    std::thread::sleep(std::time::Duration::from_secs(1));

    Ok(())
}