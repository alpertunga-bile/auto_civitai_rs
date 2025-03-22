use serde_json::Value;

#[derive(Debug, Clone)]
pub struct MetaData {
    pub prompt: String,
    pub negative_prompt: String,
    pub seed: u64,
    pub steps: u64,
    pub sampler: String,
    pub cfg: f64,
    pub clip_skip: u64,
    pub resources: String,
    pub civitai_resources: String,
}

impl MetaData {
    pub fn new() -> Self {
        Self {
            prompt: String::from("undefined"),
            negative_prompt: String::from("undefined"),
            seed: 0,
            steps: 0,
            sampler: String::from("undefined"),
            cfg: 0.0,
            clip_skip: 0,
            resources: String::from("undefined"),
            civitai_resources: String::from("undefined"),
        }
    }

    fn fill_string_values(&mut self, meta: &serde_json::Map<String, Value>, field_name: &str) {
        let field_val = meta.get(field_name);
        let mut data_value = String::from("undefined");

        if field_val.is_none() {
            return;
        }

        let data_opt = field_val.unwrap().as_str();

        if data_opt.is_some() {
            data_value = String::from(data_opt.unwrap());
        }

        match field_name {
            "prompt" => self.prompt = data_value,
            "negativePrompt" => self.negative_prompt = data_value,
            "sampler" => self.sampler = data_value,
            _ => println!("Wrong image meta string fieldname"),
        }
    }

    fn fill_uint_values(&mut self, meta: &serde_json::Map<String, Value>, field_name: &str) {
        let field_val = meta.get(field_name);
        let mut data_value: u64 = 0;

        if field_val.is_none() {
            return;
        }

        let data_opt = field_val.unwrap().as_u64();

        if data_opt.is_some() {
            data_value = data_opt.unwrap();
        }

        match field_name {
            "seed" => self.seed = data_value,
            "steps" => self.steps = data_value,
            "clipSkip" => self.clip_skip = data_value,
            _ => println!("Wrong image meta u64 fieldname"),
        }
    }

    fn fill_float_values(&mut self, meta: &serde_json::Map<String, Value>, field_name: &str) {
        let field_val = meta.get(field_name);
        let mut data_value: f64 = 0.0;

        if field_val.is_none() {
            return;
        }

        let data_opt = field_val.unwrap().as_f64();

        if data_opt.is_some() {
            data_value = data_opt.unwrap();
        }

        match field_name {
            "cfgScale" => self.cfg = data_value,
            _ => println!("Wrong image meta f64 fieldname"),
        }
    }

    fn fill_vector_values(&mut self, meta: &serde_json::Map<String, Value>, field_name: &str) {
        let field_val = meta.get(field_name);

        if field_val.is_none() {
            return;
        }

        let data_value = field_val.unwrap().to_string();

        match field_name {
            "resources" => self.resources = data_value,
            "civitaiResources" => self.civitai_resources = data_value,
            _ => println!("Wrong image meta vector fieldname"),
        }
    }

    pub fn fill(&mut self, meta: &serde_json::Map<String, Value>) {
        self.fill_string_values(meta, "prompt");
        self.fill_string_values(meta, "negativePrompt");
        self.fill_string_values(meta, "sampler");

        self.fill_uint_values(meta, "seed");
        self.fill_uint_values(meta, "steps");
        self.fill_uint_values(meta, "clipSkip");

        self.fill_float_values(meta, "cfgScale");

        self.fill_vector_values(meta, "resources");
        self.fill_vector_values(meta, "civitaiResources");
    }
}
