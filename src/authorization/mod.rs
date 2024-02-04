use base64::engine::general_purpose::STANDARD;
use base64::Engine;

/// A struct that represents the type of authorization to use.
pub enum AuthorizationType {
    Basic(Credentials),
    Token(String),
}

/// A struct that represents the credentials to use for basic authorization.
pub struct Credentials(String);

/// A struct that represents the type of certificate to use.
pub enum CertificateType {
    Pem(String),
    Der(String),
}

impl AuthorizationType {
    /// Converts the authorization type to a header.
    pub fn as_header(&self) -> (String, String) {
        match self {
            AuthorizationType::Basic(credentials) => {
                ("Authorization".to_string(), format!("Basic {}", credentials.0))
            }
            AuthorizationType::Token(token) => {
                ("Authorization".to_string(), format!("Token {}", token))
            }
        }
    }
}

impl Credentials {
    /// Creates a new instance of the Credentials struct.
    ///
    /// # Arguments
    ///
    /// * `username` - The username to use for the credentials.
    /// * `password` - The password to use for the credentials.
    pub fn new(username: String, password: String) -> Self {
        let encoded = STANDARD.encode(format!("{}:{}", username, password));

        Self(encoded)
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
