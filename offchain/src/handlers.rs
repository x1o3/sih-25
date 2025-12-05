use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::Utc;
use serde_json::json;
use tracing::{debug, info};
use validator::Validate;

use crate::{error::AppError, models::*, AppState};

// ======================== STAGE 1: FARMER REGISTRATION ========================

pub async fn register_farmer(
    State(state): State<AppState>,
    Json(payload): Json<FarmerRegistrationRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate input
    payload
        .validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    info!("Registering farmer: {}", payload.farmer_name);

    // Generate farmer DID
    let farmer_did = generate_did("farmer");

    // Create metadata object
    let metadata = FarmerRegistrationMetadata {
        farmer_did: farmer_did.clone(),
        registration_data: payload.clone(),
        registered_at: Utc::now(),
        ipfs_cid: String::new(),     // Will be updated after upload
        crop_id_hash: String::new(), // Will be computed
    };

    // Upload metadata to IPFS
    let ipfs_response = state.ipfs_client.upload_json(&metadata).await?;
    info!("Farmer metadata uploaded to IPFS: {}", ipfs_response.cid);

    // Compute crop ID hash (keccak256 for Solidity compatibility)
    let crop_id_data = format!(
        "{}-{}-{}",
        farmer_did, payload.crop_type, metadata.registered_at
    );
    let crop_id_hash = compute_keccak256(crop_id_data.as_bytes());

    // Pin the content
    state.ipfs_client.pin(&ipfs_response.cid).await?;

    let response = FarmerRegistrationResponse {
        farmer_did,
        crop_id_hash,
        ipfs_cid: ipfs_response.cid,
        registered_at: metadata.registered_at,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

// ======================== STAGE 2: FPO PURCHASE ========================

pub async fn fpo_purchase(
    State(state): State<AppState>,
    Json(payload): Json<FpoPurchaseRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate input
    payload
        .validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    info!("Recording FPO purchase for batch: {}", payload.batch_id);

    // Compute batch hash
    let batch_data = format!(
        "{}-{}-{}-{}",
        payload.farmer_did, payload.batch_id, payload.quantity_kg, payload.fpo_name
    );
    let batch_hash = compute_keccak256(batch_data.as_bytes());

    // Create metadata
    let metadata = FpoPurchaseMetadata {
        batch_hash: batch_hash.clone(),
        purchase_data: payload,
        purchased_at: Utc::now(),
        ipfs_cid: String::new(),
    };

    // Upload to IPFS
    let ipfs_response = state.ipfs_client.upload_json(&metadata).await?;
    info!(
        "FPO purchase metadata uploaded to IPFS: {}",
        ipfs_response.cid
    );

    // Pin the content
    state.ipfs_client.pin(&ipfs_response.cid).await?;

    let response = FpoPurchaseResponse {
        batch_hash,
        ipfs_cid: ipfs_response.cid,
        purchased_at: metadata.purchased_at,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

// ======================== STAGE 3: WAREHOUSE UPDATE ========================

pub async fn warehouse_update(
    State(state): State<AppState>,
    Json(payload): Json<WarehouseUpdateRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate input
    payload
        .validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    info!("Updating warehouse state: {}", payload.warehouse_id);

    // Create metadata
    let metadata = WarehouseStateMetadata {
        warehouse_id: payload.warehouse_id.clone(),
        state_hash: String::new(), // Will be computed
        warehouse_data: payload.clone(),
        updated_at: Utc::now(),
        ipfs_cid: String::new(),
    };

    // Upload to IPFS first
    let ipfs_response = state.ipfs_client.upload_json(&metadata).await?;
    info!("Warehouse state uploaded to IPFS: {}", ipfs_response.cid);

    // Compute state hash from IPFS CID + warehouse data
    let state_data = format!(
        "{}-{}-{:?}-{:?}-{}",
        payload.warehouse_id,
        payload.batch_id,
        payload.temperature_celsius,
        payload.humidity_percentage,
        ipfs_response.cid
    );
    let state_hash = compute_keccak256(state_data.as_bytes());

    // Pin the content
    state.ipfs_client.pin(&ipfs_response.cid).await?;

    let response = WarehouseUpdateResponse {
        warehouse_id: payload.warehouse_id,
        state_hash,
        ipfs_cid: ipfs_response.cid,
        updated_at: metadata.updated_at,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

// ======================== STAGE 4: LOGISTICS MILESTONE ========================

pub async fn logistics_milestone(
    State(state): State<AppState>,
    Json(payload): Json<LogisticsMilestoneRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate input
    payload
        .validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    info!(
        "Recording logistics milestone for shipment: {}",
        payload.shipment_id
    );

    // Compute location hash
    let location_data = format!(
        "{}-{}-{}-{}",
        payload.shipment_id,
        payload.current_location,
        payload.gps_coordinates.latitude,
        payload.gps_coordinates.longitude
    );
    let location_hash = compute_keccak256(location_data.as_bytes());

    // Create metadata
    let metadata = LogisticsMilestoneMetadata {
        shipment_id: payload.shipment_id.clone(),
        location_hash: location_hash.clone(),
        milestone_data: payload.clone(),
        recorded_at: Utc::now(),
        ipfs_cid: String::new(),
    };

    // Upload to IPFS
    let ipfs_response = state.ipfs_client.upload_json(&metadata).await?;
    info!(
        "Logistics milestone uploaded to IPFS: {}",
        ipfs_response.cid
    );

    // Pin the content
    state.ipfs_client.pin(&ipfs_response.cid).await?;

    let response = LogisticsMilestoneResponse {
        shipment_id: payload.shipment_id,
        location_hash,
        ipfs_cid: ipfs_response.cid,
        recorded_at: metadata.recorded_at,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

// ======================== STAGE 5: PROCESS BATCH ========================

pub async fn process_batch(
    State(state): State<AppState>,
    Json(payload): Json<ProcessBatchRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate input
    payload
        .validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    info!("Processing batch: {}", payload.input_batch_id);

    // Compute input batch hash
    let input_data = format!(
        "{}-{}-{}",
        payload.input_batch_id, payload.processor_name, payload.input_quantity_kg
    );
    let input_batch_hash = compute_keccak256(input_data.as_bytes());

    // Compute output batch hashes
    let output_batch_hashes: Vec<String> = payload
        .output_batch_ids
        .iter()
        .map(|id| {
            let output_data = format!("{}-{}", id, payload.output_quantity_kg);
            compute_keccak256(output_data.as_bytes())
        })
        .collect();

    // Compute transform hash (hash of the transformation process)
    let transform_data = format!(
        "{:?}-{}-{}",
        payload.processing_type, payload.yield_percentage, payload.waste_percentage
    );
    let transform_hash = compute_keccak256(transform_data.as_bytes());

    // Create metadata
    let metadata = ProcessBatchMetadata {
        input_batch_hash: input_batch_hash.clone(),
        transform_hash: transform_hash.clone(),
        output_batch_hashes: output_batch_hashes.clone(),
        process_data: payload,
        processed_at: Utc::now(),
        ipfs_cid: String::new(),
    };

    // Upload to IPFS
    let ipfs_response = state.ipfs_client.upload_json(&metadata).await?;
    info!(
        "Process batch metadata uploaded to IPFS: {}",
        ipfs_response.cid
    );

    // Pin the content
    state.ipfs_client.pin(&ipfs_response.cid).await?;

    let response = ProcessBatchResponse {
        input_batch_hash,
        transform_hash,
        output_batch_hashes,
        ipfs_cid: ipfs_response.cid,
        processed_at: metadata.processed_at,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

// ======================== STAGE 6: CREATE SKU ========================

pub async fn create_sku(
    State(state): State<AppState>,
    Json(payload): Json<CreateSkuRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate input
    payload
        .validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    info!("Creating SKU: {}", payload.sku_id);

    // Compute parent batch hash
    let batch_data = format!("{}-{}", payload.parent_batch_id, payload.product_name);
    let parent_batch_hash = compute_keccak256(batch_data.as_bytes());

    // Compute merkle root (if SKU IDs provided in proof, otherwise use SKU ID)
    let merkle_leaves = if let Some(ref proof) = payload.merkle_proof {
        proof.clone()
    } else {
        vec![payload.sku_id.clone()]
    };
    let merkle_root = compute_merkle_root(&merkle_leaves);

    // Create metadata
    let metadata = CreateSkuMetadata {
        sku_id: payload.sku_id.clone(),
        parent_batch_hash: parent_batch_hash.clone(),
        merkle_root: merkle_root.clone(),
        sku_data: payload,
        packaged_at: Utc::now(),
        ipfs_cid: String::new(),
    };

    // Upload to IPFS
    let ipfs_response = state.ipfs_client.upload_json(&metadata).await?;
    info!("SKU metadata uploaded to IPFS: {}", ipfs_response.cid);

    // Pin the content
    state.ipfs_client.pin(&ipfs_response.cid).await?;

    let response = CreateSkuResponse {
        sku_id: metadata.sku_id,
        parent_batch_hash,
        merkle_root,
        ipfs_cid: ipfs_response.cid,
        packaged_at: metadata.packaged_at,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

// ======================== STAGE 7: AI SCORING ========================

pub async fn ai_score(
    State(state): State<AppState>,
    Json(payload): Json<AiScoreRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate input
    payload
        .validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    info!("Recording AI score for batch: {}", payload.batch_id);

    // Compute batch hash
    let batch_data = format!("{}-{}", payload.batch_id, payload.model_name);
    let batch_hash = compute_keccak256(batch_data.as_bytes());

    // Generate nonce for commit-reveal
    let nonce = uuid::Uuid::new_v4().to_string();

    // Compute reveal hash (hash of the score data)
    let reveal_data =
        serde_json::to_string(&payload).map_err(|e| AppError::SerializationError(e))?;
    let reveal_hash = compute_keccak256(reveal_data.as_bytes());

    // Compute commit hash (hash of reveal hash + nonce)
    let commit_data = format!("{}{}", reveal_hash, nonce);
    let commit_hash = compute_keccak256(commit_data.as_bytes());

    // Create metadata
    let metadata = AiScoreMetadata {
        batch_hash: batch_hash.clone(),
        commit_hash: commit_hash.clone(),
        reveal_hash: reveal_hash.clone(),
        nonce: nonce.clone(),
        score_data: payload,
        scored_at: Utc::now(),
        ipfs_cid: String::new(),
    };

    // Upload to IPFS
    let ipfs_response = state.ipfs_client.upload_json(&metadata).await?;
    info!("AI score metadata uploaded to IPFS: {}", ipfs_response.cid);

    // Pin the content
    state.ipfs_client.pin(&ipfs_response.cid).await?;

    let response = AiScoreResponse {
        batch_hash,
        commit_hash,
        reveal_hash,
        ipfs_cid: ipfs_response.cid,
        scored_at: metadata.scored_at,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

// ======================== GENERIC IPFS OPERATIONS ========================

pub async fn upload_to_ipfs(
    State(state): State<AppState>,
    Json(payload): Json<IpfsUploadRequest>,
) -> Result<impl IntoResponse, AppError> {
    debug!("Uploading generic data to IPFS");

    // Upload to IPFS
    let ipfs_response = state.ipfs_client.upload_json(&payload.data).await?;
    info!("Data uploaded to IPFS: {}", ipfs_response.cid);

    // Pin if requested
    let pinned = if payload.pin {
        state.ipfs_client.pin(&ipfs_response.cid).await?;
        true
    } else {
        false
    };

    let response = IpfsUploadResponse {
        cid: ipfs_response.cid,
        size: ipfs_response.size,
        pinned,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_from_ipfs(
    State(state): State<AppState>,
    Path(cid): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    debug!("Fetching data from IPFS: {}", cid);

    // Get from IPFS
    let data: serde_json::Value = state.ipfs_client.get_json(&cid).await?;

    let response = IpfsGetResponse { cid, data };

    Ok(Json(response))
}

pub async fn pin_ipfs(
    State(state): State<AppState>,
    Path(cid): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    debug!("Pinning IPFS content: {}", cid);

    // Pin the content
    let pin_response = state.ipfs_client.pin(&cid).await?;

    let response = IpfsPinResponse {
        cid: pin_response.cid,
        pinned: pin_response.pinned,
    };

    Ok(Json(response))
}
