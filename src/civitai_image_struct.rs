use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CivitaiImageStats {
    pub cryCount: u32,
    pub laughCount: u32,
    pub likeCount: u32,
    pub dislikeCount: u32,
    pub heartCount: u32,
    pub commentCount: u32,
}

#[derive(Serialize, Deserialize)]
pub struct CivitaiImageCivitaiResources {
    pub r#type: Option<String>,
    pub weight: Option<f32>,
    pub modelVersionId: u32,
}

#[derive(Serialize, Deserialize)]
pub struct CivitaiImageResource {
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub weight: Option<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct CivitaiImageMeta {
    pub Size: Option<String>,
    pub seed: Option<u64>,
    pub steps: Option<u8>,
    pub prompt: Option<String>,
    pub sampler: Option<String>,
    pub cfgScale: Option<f32>,
    pub clipSkip: Option<u8>,
    pub resources: Option<Vec<CivitaiImageResource>>,
    pub negativePrompt: Option<String>,
    pub civitaiResources: Option<Vec<CivitaiImageCivitaiResources>>,
}

#[derive(Serialize, Deserialize)]
pub struct CivitaiImageInfo {
    pub id: u64,
    pub url: Option<String>,
    pub hash: Option<String>,
    pub width: u32,
    pub height: u32,
    pub nsfwLevel: Option<String>,
    pub r#type: Option<String>,
    pub nsfw: bool,
    pub browsingLevel: u32,
    pub createdAt: Option<String>,
    pub postId: u64,
    pub stats: CivitaiImageStats,
    pub meta: Option<CivitaiImageMeta>,
    pub username: Option<String>,
    pub baseModel: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CivitaiImagePageMetadata {
    pub nextCursor: Option<String>,
    pub nextPage: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CivitaiImagePage {
    pub items: Vec<CivitaiImageInfo>,
    pub metadata: CivitaiImagePageMetadata,
}
