use std::path::Path;

use axum_server::tls_rustls::RustlsConfig;
use color_eyre::{eyre::WrapErr, Result};
use tracing::{info, warn};

/// Initialize the TLS configuration
///
/// If the certificate and key files exist in the data directory, they are loaded. Otherwise, a
/// self-signed certificate is generated and saved to the data directory.
pub async fn init(data_path: impl AsRef<Path>) -> Result<RustlsConfig> {
    let data_path = data_path.as_ref();
    let cert_path = data_path.join("cert.pem");
    let key_path = data_path.join("key.pem");
    if !cert_path.exists() || !key_path.exists() {
        fs_err::create_dir_all(data_path).wrap_err("unable to create data directory")?;
        generate(&cert_path, &key_path).await?;
    }
    from_file(&cert_path, &key_path).await
}

/// Load the TLS configuration from the certificate and key files
async fn from_file(cert_path: &Path, key_path: &Path) -> Result<RustlsConfig> {
    info!(
        "Loading TLS configuration from {} and {}",
        cert_path.display(),
        key_path.display()
    );
    RustlsConfig::from_pem_file(cert_path, key_path)
        .await
        .wrap_err("unable to load TLS configuration")
}

/// Generate a self-signed certificate and save it to the certificate and key files
async fn generate(cert_path: &Path, key_path: &Path) -> Result<()> {
    warn!(
        "Generating self-signed certificate and saving to {} and {}",
        cert_path.display(),
        key_path.display()
    );
    let certified_key = rcgen::generate_simple_self_signed(["localhost".to_string()])?;
    let cert = certified_key.cert.pem().into_bytes();
    let key = certified_key.key_pair.serialize_pem().into_bytes();
    fs_err::write(cert_path, &cert).wrap_err("unable to save Certificate file")?;
    fs_err::write(key_path, &key).wrap_err("unable to save Key file")?;
    Ok(())
}
