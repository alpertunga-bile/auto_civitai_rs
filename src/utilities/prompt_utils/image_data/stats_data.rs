use serde_json::Value;

#[derive(Debug, Copy, Clone)]
pub struct StatsData {
    pub cry_count: u64,
    pub laugh_count: u64,
    pub like_count: u64,
    pub dislike_count: u64,
    pub heart_count: u64,
    pub comment_count: u64,
}

impl StatsData {
    pub fn new() -> Self {
        Self {
            cry_count: 0,
            laugh_count: 0,
            like_count: 0,
            dislike_count: 0,
            heart_count: 0,
            comment_count: 0,
        }
    }

    fn fill_value(&mut self, stats: &serde_json::Map<String, Value>, field_name: &str) {
        let field_val = stats.get(field_name);
        let mut data_value: u64 = 0;

        if field_val.is_none() {
            return;
        }

        let data_opt = field_val.unwrap().as_u64();

        if data_opt.is_some() {
            data_value = data_opt.unwrap();
        }

        match field_name {
            "cryCount" => self.cry_count = data_value,
            "laughCount" => self.laugh_count = data_value,
            "likeCount" => self.like_count = data_value,
            "dislikeCount" => self.dislike_count = data_value,
            "heartCount" => self.heart_count = data_value,
            "commentCount" => self.comment_count = data_value,
            _ => println!("Undefined image stats data variable"),
        }
    }

    pub fn fill(&mut self, stats: &serde_json::Map<String, Value>) {
        self.fill_value(stats, "cryCount");
        self.fill_value(stats, "laughCount");
        self.fill_value(stats, "likeCount");
        self.fill_value(stats, "dislikeCount");
        self.fill_value(stats, "heartCount");
        self.fill_value(stats, "commentCount");
    }
}
