use serde_json::Value;

#[derive(Debug, Clone)]
pub struct ItemData {
    pub image_url: String,
    pub base_model: String,
    pub nsfw_level: String,
    pub gen_type: String,
}

impl ItemData {
    pub fn new() -> Self {
        Self {
            image_url: String::from("undefined"),
            base_model: String::from("undefined"),
            nsfw_level: String::from("undefined"),
            gen_type: String::from("undefined"),
        }
    }

    fn fill_value(&mut self, item: &Value, field_name: &str) {
        let field_val = item.get(field_name);
        let mut data_value = String::from("undefined");

        if field_val.is_none() {
            return;
        }

        let data_opt = field_val.unwrap().as_str();

        if data_opt.is_some() {
            data_value = String::from(data_opt.unwrap());
        }

        match field_name {
            "url" => self.image_url = data_value,
            "baseModel" => self.base_model = data_value,
            "nsfwLevel" => self.nsfw_level = data_value,
            "type" => self.gen_type = data_value,
            _ => println!("Undefined image item data variable"),
        }
    }

    pub fn fill(&mut self, item: &Value) {
        self.fill_value(item, "url");
        self.fill_value(item, "baseModel");
        self.fill_value(item, "nsfwLevel");
        self.fill_value(item, "type");
    }
}
