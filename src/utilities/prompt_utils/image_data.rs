mod item_data;
mod meta_data;
mod stats_data;

use item_data::ItemData;
use meta_data::MetaData;
use stats_data::StatsData;

use polars::prelude::{Column, DataFrame};

#[derive(Debug, Clone)]
pub struct ImageData {
    pub meta: MetaData,
    pub items: ItemData,
    pub stats: StatsData,
}

pub enum ImageDataValues {
    Prompt = 0,
    NegativePrompt,
    Seed,
    Steps,
    Sampler,
    Cfg,
    ClipSkip,
    Resources,
    CivitaiResources,
    ImageUrl,
    BaseModel,
    NsfwLevel,
    GenType,
    CryCount,
    LaughCount,
    LikeCount,
    DislikeCount,
    HeartCount,
    CommentCount,
    TotalValues,
}

impl ImageData {
    pub fn new() -> Self {
        Self {
            meta: MetaData::new(),
            items: ItemData::new(),
            stats: StatsData::new(),
        }
    }
}

pub struct ImageDataVectors {
    pub prompts: Vec<String>,
    pub negative_prompts: Vec<String>,
    pub seeds: Vec<u64>,
    pub steps: Vec<u64>,
    pub samplers: Vec<String>,
    pub cfgs: Vec<f64>,
    pub clip_skips: Vec<u64>,
    pub resources: Vec<String>,
    pub civitai_resources: Vec<String>,
    pub image_urls: Vec<String>,
    pub base_models: Vec<String>,
    pub nsfw_levels: Vec<String>,
    pub gen_types: Vec<String>,
    pub cry_counts: Vec<u64>,
    pub laugh_counts: Vec<u64>,
    pub like_counts: Vec<u64>,
    pub dislike_counts: Vec<u64>,
    pub heart_counts: Vec<u64>,
    pub comment_counts: Vec<u64>,
}

impl ImageDataVectors {
    pub fn new(total_size: usize) -> Self {
        Self {
            prompts: Vec::with_capacity(total_size),
            negative_prompts: Vec::with_capacity(total_size),
            seeds: Vec::with_capacity(total_size),
            steps: Vec::with_capacity(total_size),
            samplers: Vec::with_capacity(total_size),
            cfgs: Vec::with_capacity(total_size),
            clip_skips: Vec::with_capacity(total_size),
            resources: Vec::with_capacity(total_size),
            civitai_resources: Vec::with_capacity(total_size),
            image_urls: Vec::with_capacity(total_size),
            base_models: Vec::with_capacity(total_size),
            nsfw_levels: Vec::with_capacity(total_size),
            gen_types: Vec::with_capacity(total_size),
            cry_counts: Vec::with_capacity(total_size),
            laugh_counts: Vec::with_capacity(total_size),
            like_counts: Vec::with_capacity(total_size),
            dislike_counts: Vec::with_capacity(total_size),
            heart_counts: Vec::with_capacity(total_size),
            comment_counts: Vec::with_capacity(total_size),
        }
    }

    pub fn append(&mut self, data: &ImageData) {
        self.prompts.push(data.meta.prompt.clone());
        self.negative_prompts
            .push(data.meta.negative_prompt.clone());
        self.seeds.push(data.meta.seed);
        self.steps.push(data.meta.steps);
        self.samplers.push(data.meta.sampler.clone());
        self.cfgs.push(data.meta.cfg);
        self.clip_skips.push(data.meta.clip_skip);
        self.resources.push(data.meta.resources.clone());
        self.civitai_resources
            .push(data.meta.civitai_resources.clone());

        self.image_urls.push(data.items.image_url.clone());
        self.base_models.push(data.items.base_model.clone());
        self.nsfw_levels.push(data.items.nsfw_level.clone());
        self.gen_types.push(data.items.gen_type.clone());

        self.cry_counts.push(data.stats.cry_count);
        self.laugh_counts.push(data.stats.laugh_count);
        self.like_counts.push(data.stats.like_count);
        self.dislike_counts.push(data.stats.dislike_count);
        self.heart_counts.push(data.stats.heart_count);
        self.comment_counts.push(data.stats.comment_count);
    }

    pub fn create_dataframe(&self, total_size: usize) -> DataFrame {
        let mut columns = Vec::with_capacity(total_size);

        columns.push(Column::new("prompt".into(), self.prompts.clone()));
        columns.push(Column::new(
            "negative_prompt".into(),
            self.negative_prompts.clone(),
        ));
        columns.push(Column::new("seed".into(), self.seeds.clone()));
        columns.push(Column::new("steps".into(), self.steps.clone()));
        columns.push(Column::new("sampler".into(), self.samplers.clone()));
        columns.push(Column::new("cfg".into(), self.cfgs.clone()));
        columns.push(Column::new("clip_skip".into(), self.clip_skips.clone()));
        columns.push(Column::new("resources".into(), self.resources.clone()));
        columns.push(Column::new(
            "civitai_resources".into(),
            self.civitai_resources.clone(),
        ));
        columns.push(Column::new("url".into(), self.image_urls.clone()));
        columns.push(Column::new("base_model".into(), self.base_models.clone()));
        columns.push(Column::new("nsfw_level".into(), self.nsfw_levels.clone()));
        columns.push(Column::new("type".into(), self.gen_types.clone()));
        columns.push(Column::new("cry_count".into(), self.cry_counts.clone()));
        columns.push(Column::new("laugh_count".into(), self.laugh_counts.clone()));
        columns.push(Column::new("like_count".into(), self.like_counts.clone()));
        columns.push(Column::new(
            "dislike_count".into(),
            self.dislike_counts.clone(),
        ));
        columns.push(Column::new("heart_count".into(), self.heart_counts.clone()));
        columns.push(Column::new(
            "comment_count".into(),
            self.comment_counts.clone(),
        ));

        let df = DataFrame::new(columns);

        if df.is_err() {
            return DataFrame::default();
        }

        df.ok().unwrap()
    }
}
