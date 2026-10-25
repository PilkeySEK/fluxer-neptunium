use reqwest::{IntoUrl, Method};

use crate::{client::HttpClient, endpoints::ExecuteEndpointRequestError};

/// Upload a file using the S3 API.
///
/// # Errors
/// Returns an error if executing the request fails or the response status is not a success response.
pub async fn upload_file_s3(
    url: impl IntoUrl,
    file_bytes: Vec<u8>,
    client: &HttpClient,
) -> Result<(), ExecuteEndpointRequestError> {
    let response = client
        .reqwest_client
        .request(Method::PUT, url)
        .body(file_bytes)
        // TODO: Is this header needed?
        .header("Content-Type", "application/octet-stream")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(ExecuteEndpointRequestError::ResponseNotOk(Box::new(
            response,
        )));
    }
    Ok(())
}
