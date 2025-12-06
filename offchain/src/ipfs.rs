// read ipfs api
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct IpfsClient {
    http_client: String,
    base_url: String,
    auth: String,
}

impl IpfsClient {
    pub async fn upload_json(http_client: String, base_url: String, auth: String) -> Self {
        Self {
            http_client,
            base_url,
            auth,
        }
    }
}
