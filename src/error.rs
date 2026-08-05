use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum CertError {
    #[error("Failed to read certificate file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Missing certificate file: {0}")]
    MissingCert(String),

    #[cfg(feature = "grpc")]
    #[error("TLS configuration error: {0}")]
    TlsError(#[from] tonic::transport::Error),

    #[error("REST request error: {0}")]
    RestError(String),
}

#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, CertError>;
