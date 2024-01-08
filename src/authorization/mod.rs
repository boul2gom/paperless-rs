use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use derive_more::Constructor;

/// A struct that represents the type of authorization to use.
pub enum AuthorizationType {
    Basic(Credentials),
    Token(String),
}

/// A struct that represents the credentials to use for basic authorization.
#[derive(Constructor)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub encoded: Option<String>,
}

/// A struct that represents the type of certificate to use.
pub enum CertificateType {
    Pem(String),
    Der(String),
}

impl AuthorizationType {
    /// Converts the authorization type to a header.
    pub fn as_header(&mut self) -> (String, String) {
        match self {
            AuthorizationType::Basic(credentials) => {
                if let Some(encoded) = &credentials.encoded {
                    return ("Authorization".to_string(), format!("Basic {}", encoded));
                }

                let encoded_credentials =
                    STANDARD.encode(format!("{}:{}", credentials.username, credentials.password));
                credentials.encoded = Some(encoded_credentials.clone());

                (
                    "Authorization".to_string(),
                    format!("Basic {}", encoded_credentials),
                )
            }
            AuthorizationType::Token(token) => {
                ("Authorization".to_string(), format!("Token {}", token))
            }
        }
    }
}

impl CertificateType {
    /// Converts the certificate type to a reqwest certificate.
    pub async fn as_certificate(&self) -> Result<reqwest::Certificate, Box<dyn std::error::Error>> {
        match self {
            CertificateType::Pem(path) => {
                let certificate = tokio::fs::read(path).await?;
                Ok(reqwest::Certificate::from_pem(&certificate)?)
            }
            CertificateType::Der(path) => {
                let certificate = tokio::fs::read(path).await?;
                Ok(reqwest::Certificate::from_der(&certificate)?)
            }
        }
    }
}
