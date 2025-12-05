use anyhow::Result;
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient as HyperIpfsClient, TryFromUri};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use tracing::{debug, info, warn};

use crate::error::AppError;

#[derive(Clone)]
pub struct IpfsClient {
    client: HyperIpfsClient,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpfsUploadResponse {
    pub cid: String,
    pub size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpfsPinResponse {
    pub cid: String,
    pub pinned: bool,
}

impl IpfsClient {
    /// Create a new IPFS client
    pub fn new(url: &str) -> Result<Self, AppError> {
        let client = HyperIpfsClient::from_str(url)
            .map_err(|e| AppError::IpfsError(format!("Failed to create IPFS client: {}", e)))?;

        info!("IPFS client initialized with URL: {}", url);

        Ok(Self { client })
    }

    /// Upload raw bytes to IPFS
    pub async fn upload_bytes(&self, data: Vec<u8>) -> Result<IpfsUploadResponse, AppError> {
        debug!("Uploading {} bytes to IPFS", data.len());

        let cursor = Cursor::new(data.clone());
        let response = self
            .client
            .add(cursor)
            .await
            .map_err(|e| AppError::IpfsError(format!("Failed to upload to IPFS: {}", e)))?;

        info!("Successfully uploaded to IPFS with CID: {}", response.hash);

        Ok(IpfsUploadResponse {
            cid: response.hash,
            size: response.size,
        })
    }

    /// Upload JSON data to IPFS
    pub async fn upload_json<T: Serialize>(
        &self,
        data: &T,
    ) -> Result<IpfsUploadResponse, AppError> {
        let json_bytes = serde_json::to_vec(data).map_err(|e| AppError::SerializationError(e))?;

        self.upload_bytes(json_bytes).await
    }

    /// Get data from IPFS by CID
    pub async fn get(&self, cid: &str) -> Result<Vec<u8>, AppError> {
        debug!("Fetching CID from IPFS: {}", cid);

        let response = self
            .client
            .cat(cid)
            .map_ok(|chunk| chunk.to_vec())
            .try_concat()
            .await
            .map_err(|e| AppError::IpfsError(format!("Failed to fetch from IPFS: {}", e)))?;

        info!("Successfully fetched {} bytes from IPFS", response.len());

        Ok(response)
    }

    /// Get JSON data from IPFS by CID
    pub async fn get_json<T: for<'de> Deserialize<'de>>(&self, cid: &str) -> Result<T, AppError> {
        let bytes = self.get(cid).await?;
        let data = serde_json::from_slice(&bytes).map_err(|e| AppError::SerializationError(e))?;

        Ok(data)
    }

    /// Pin content by CID
    pub async fn pin(&self, cid: &str) -> Result<IpfsPinResponse, AppError> {
        debug!("Pinning CID: {}", cid);

        self.client
            .pin_add(cid, false)
            .await
            .map_err(|e| AppError::IpfsError(format!("Failed to pin CID: {}", e)))?;

        info!("Successfully pinned CID: {}", cid);

        Ok(IpfsPinResponse {
            cid: cid.to_string(),
            pinned: true,
        })
    }

    /// Unpin content by CID
    pub async fn unpin(&self, cid: &str) -> Result<(), AppError> {
        debug!("Unpinning CID: {}", cid);

        self.client
            .pin_rm(cid, false)
            .await
            .map_err(|e| AppError::IpfsError(format!("Failed to unpin CID: {}", e)))?;

        info!("Successfully unpinned CID: {}", cid);

        Ok(())
    }

    /// Check if content is pinned
    pub async fn is_pinned(&self, cid: &str) -> Result<bool, AppError> {
        debug!("Checking pin status for CID: {}", cid);

        let pins = self
            .client
            .pin_ls(Some(cid), None)
            .await
            .map_err(|e| AppError::IpfsError(format!("Failed to check pin status: {}", e)))?;

        let pinned = !pins.keys.is_empty();
        debug!("CID {} pin status: {}", cid, pinned);

        Ok(pinned)
    }

    /// Get IPFS node info
    pub async fn node_info(&self) -> Result<String, AppError> {
        let version = self
            .client
            .version()
            .await
            .map_err(|e| AppError::IpfsError(format!("Failed to get node info: {}", e)))?;

        Ok(format!("IPFS version: {}", version.version))
    }

    /// Get repository statistics
    pub async fn repo_stats(&self) -> Result<serde_json::Value, AppError> {
        let stats = self
            .client
            .stats_repo()
            .await
            .map_err(|e| AppError::IpfsError(format!("Failed to get repo stats: {}", e)))?;

        Ok(serde_json::json!({
            "num_objects": stats.num_objects,
            "repo_size": stats.repo_size,
            "storage_max": stats.storage_max,
        }))
    }
}

// Import the missing traits
use futures::TryStreamExt;
