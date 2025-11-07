use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalAsset {
    pub id: Uuid,
    pub original_contract: String,
    pub chain_id: u64,
    pub token_id: String,
    pub metadata: AssetMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetMetadata {
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub attributes: Vec<AssetAttribute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetAttribute {
    pub trait_type: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameCompatibility {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub game_id: String,
    pub game_name: String,
    pub compatibility_level: CompatibilityLevel,
    pub supported: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompatibilityLevel {
    Full,
    Partial,
    Visual,
    Economic,
}

// Request/Response types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAssetRequest {
    pub original_contract: String,
    pub chain_id: u64,
    pub token_id: String,
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub attributes: Vec<AssetAttribute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAssetResponse {
    pub asset_id: Uuid,
}
EOF