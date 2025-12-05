use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use validator::Validate;

// ======================== STAGE 1: FARMER REGISTRATION ========================

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct FarmerRegistrationRequest {
    #[validate(length(min = 1))]
    pub farmer_name: String,

    #[validate(length(min = 1))]
    pub crop_type: String,

    pub land_area_hectares: f64,

    #[validate(length(min = 1))]
    pub location: String,

    pub gps_coordinates: Option<GpsCoordinates>,

    // Off-chain metadata
    pub kyc_document_url: Option<String>,
    pub land_ownership_docs: Vec<String>,
    pub satellite_imagery_url: Option<String>,
    pub soil_test_report: Option<String>,

    // Additional metadata
    pub phone_number: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GpsCoordinates {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FarmerRegistrationMetadata {
    pub farmer_did: String,
    pub registration_data: FarmerRegistrationRequest,
    pub registered_at: DateTime<Utc>,
    pub ipfs_cid: String,
    pub crop_id_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FarmerRegistrationResponse {
    pub farmer_did: String,
    pub crop_id_hash: String,
    pub ipfs_cid: String,
    pub registered_at: DateTime<Utc>,
}

// ======================== STAGE 2: FPO PURCHASE ========================

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct FpoPurchaseRequest {
    #[validate(length(min = 1))]
    pub farmer_did: String,

    #[validate(length(min = 1))]
    pub fpo_name: String,

    pub batch_id: String,

    pub quantity_kg: f64,

    pub price_per_kg: f64,

    pub quality_grade: String,

    // Off-chain metadata
    pub quality_report_url: Option<String>,
    pub weight_slip_url: Option<String>,
    pub photos: Vec<String>,
    pub moisture_content: Option<f64>,
    pub impurity_percentage: Option<f64>,
    pub payment_reference: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FpoPurchaseMetadata {
    pub batch_hash: String,
    pub purchase_data: FpoPurchaseRequest,
    pub purchased_at: DateTime<Utc>,
    pub ipfs_cid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FpoPurchaseResponse {
    pub batch_hash: String,
    pub ipfs_cid: String,
    pub purchased_at: DateTime<Utc>,
}

// ======================== STAGE 3: WAREHOUSE STORAGE ========================

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WarehouseUpdateRequest {
    #[validate(length(min = 1))]
    pub warehouse_id: String,

    #[validate(length(min = 1))]
    pub batch_id: String,

    pub storage_location: String,

    // IoT sensor data
    pub temperature_celsius: Option<f64>,
    pub humidity_percentage: Option<f64>,
    pub co2_level_ppm: Option<f64>,

    // Continuous monitoring data
    pub iot_logs_url: Option<String>,
    pub inspection_reports: Vec<String>,

    // Pest and quality control
    pub pest_inspection: Option<PestInspection>,
    pub quality_degradation: Option<f64>, // percentage
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PestInspection {
    pub inspected_at: DateTime<Utc>,
    pub pest_found: bool,
    pub pest_type: Option<String>,
    pub treatment_applied: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WarehouseStateMetadata {
    pub warehouse_id: String,
    pub state_hash: String,
    pub warehouse_data: WarehouseUpdateRequest,
    pub updated_at: DateTime<Utc>,
    pub ipfs_cid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WarehouseUpdateResponse {
    pub warehouse_id: String,
    pub state_hash: String,
    pub ipfs_cid: String,
    pub updated_at: DateTime<Utc>,
}

// ======================== STAGE 4: LOGISTICS TRACKING ========================

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LogisticsMilestoneRequest {
    #[validate(length(min = 1))]
    pub shipment_id: String,

    pub current_location: String,
    pub gps_coordinates: GpsCoordinates,

    pub milestone_type: MilestoneType,

    // Full GPS history (off-chain)
    pub gps_history_url: Option<String>,

    pub carrier_name: String,
    pub vehicle_id: String,
    pub driver_name: Option<String>,

    // Environmental conditions during transport
    pub temperature_log: Option<String>,
    pub shock_events: Vec<ShockEvent>,

    pub estimated_arrival: Option<DateTime<Utc>>,
    pub is_delivered: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MilestoneType {
    PickedUp,
    InTransit,
    AtCheckpoint,
    Delivered,
    Delayed,
    Incident,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShockEvent {
    pub timestamp: DateTime<Utc>,
    pub g_force: f64,
    pub location: Option<GpsCoordinates>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogisticsMilestoneMetadata {
    pub shipment_id: String,
    pub location_hash: String,
    pub milestone_data: LogisticsMilestoneRequest,
    pub recorded_at: DateTime<Utc>,
    pub ipfs_cid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogisticsMilestoneResponse {
    pub shipment_id: String,
    pub location_hash: String,
    pub ipfs_cid: String,
    pub recorded_at: DateTime<Utc>,
}

// ======================== STAGE 5: PROCESSING ========================

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ProcessBatchRequest {
    #[validate(length(min = 1))]
    pub input_batch_id: String,

    #[validate(length(min = 1))]
    pub processor_name: String,

    pub processing_type: ProcessingType,

    pub input_quantity_kg: f64,
    pub output_quantity_kg: f64,

    // Yield calculations
    pub yield_percentage: f64,
    pub waste_percentage: f64,

    // Lab results and certifications
    pub lab_results_url: Vec<String>,
    pub certifications: Vec<String>,

    // Output batches (for splitting/transformation)
    pub output_batch_ids: Vec<String>,

    // Process parameters
    pub processing_parameters: Option<ProcessingParameters>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ProcessingType {
    Cleaning,
    Drying,
    Milling,
    Extraction,
    Refining,
    Blending,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessingParameters {
    pub temperature_celsius: Option<f64>,
    pub pressure_bar: Option<f64>,
    pub duration_minutes: Option<u32>,
    pub method: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessBatchMetadata {
    pub input_batch_hash: String,
    pub transform_hash: String,
    pub output_batch_hashes: Vec<String>,
    pub process_data: ProcessBatchRequest,
    pub processed_at: DateTime<Utc>,
    pub ipfs_cid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessBatchResponse {
    pub input_batch_hash: String,
    pub transform_hash: String,
    pub output_batch_hashes: Vec<String>,
    pub ipfs_cid: String,
    pub processed_at: DateTime<Utc>,
}

// ======================== STAGE 6: PACKAGING ========================

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateSkuRequest {
    #[validate(length(min = 1))]
    pub sku_id: String,

    #[validate(length(min = 1))]
    pub parent_batch_id: String,

    #[validate(length(min = 1))]
    pub product_name: String,

    pub brand: String,
    pub unit_weight_grams: f64,
    pub units_packaged: u32,

    // Packaging metadata
    pub package_type: String,
    pub barcode: Option<String>,
    pub qr_code: Option<String>,

    // Nutritional and regulatory info
    pub nutritional_info_url: Option<String>,
    pub regulatory_certifications: Vec<String>,

    // Consumer-facing data
    pub label_images: Vec<String>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub best_before_date: Option<DateTime<Utc>>,

    // Merkle proof for batch verification
    pub merkle_proof: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSkuMetadata {
    pub sku_id: String,
    pub parent_batch_hash: String,
    pub merkle_root: String,
    pub sku_data: CreateSkuRequest,
    pub packaged_at: DateTime<Utc>,
    pub ipfs_cid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSkuResponse {
    pub sku_id: String,
    pub parent_batch_hash: String,
    pub merkle_root: String,
    pub ipfs_cid: String,
    pub packaged_at: DateTime<Utc>,
}

// ======================== STAGE 7: AI SCORING ========================

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct AiScoreRequest {
    #[validate(length(min = 1))]
    pub batch_id: String,

    pub quality_score: f64, // 0.0 to 100.0
    pub sustainability_score: f64,
    pub traceability_score: f64,

    // AI model information
    pub model_name: String,
    pub model_version: String,

    // Input features used
    pub features: serde_json::Value,

    // Model outputs and explanations
    pub predictions: serde_json::Value,
    pub confidence: f64,

    // Links to full model artifacts
    pub model_artifacts_url: Option<String>,
    pub training_data_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiScoreMetadata {
    pub batch_hash: String,
    pub commit_hash: String,
    pub reveal_hash: String,
    pub nonce: String,
    pub score_data: AiScoreRequest,
    pub scored_at: DateTime<Utc>,
    pub ipfs_cid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiScoreResponse {
    pub batch_hash: String,
    pub commit_hash: String,
    pub reveal_hash: String,
    pub ipfs_cid: String,
    pub scored_at: DateTime<Utc>,
}

// ======================== GENERIC IPFS OPERATIONS ========================

#[derive(Debug, Serialize, Deserialize)]
pub struct IpfsUploadRequest {
    pub data: serde_json::Value,
    pub pin: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpfsUploadResponse {
    pub cid: String,
    pub size: u64,
    pub pinned: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpfsGetResponse {
    pub cid: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpfsPinRequest {
    pub cid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpfsPinResponse {
    pub cid: String,
    pub pinned: bool,
}

// ======================== UTILITY FUNCTIONS ========================

/// Compute keccak256 hash (compatible with Solidity)
pub fn compute_keccak256(data: &[u8]) -> String {
    use sha3::Keccak256;
    let mut hasher = Keccak256::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("0x{}", hex::encode(result))
}

/// Compute SHA256 hash
pub fn compute_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("0x{}", hex::encode(result))
}

/// Generate a unique DID (Decentralized Identifier)
pub fn generate_did(prefix: &str) -> String {
    format!("did:{}:{}", prefix, uuid::Uuid::new_v4())
}

/// Compute merkle root from a list of hashes
pub fn compute_merkle_root(hashes: &[String]) -> String {
    if hashes.is_empty() {
        return compute_sha256(b"empty");
    }

    if hashes.len() == 1 {
        return hashes[0].clone();
    }

    let mut current_level = hashes.to_vec();

    while current_level.len() > 1 {
        let mut next_level = Vec::new();

        for chunk in current_level.chunks(2) {
            let combined = if chunk.len() == 2 {
                format!("{}{}", chunk[0], chunk[1])
            } else {
                format!("{}{}", chunk[0], chunk[0])
            };

            next_level.push(compute_sha256(combined.as_bytes()));
        }

        current_level = next_level;
    }

    current_level[0].clone()
}
