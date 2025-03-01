mod preprocess;

use kdam::tqdm;
use preprocess::preprocess;
use regex::Regex;
use serde_json::Value;
use std::ops::Index;

fn check_prompt(prompt: &String, word_list: &Vec<String>) -> bool {
    let multi_whitespace_regex = Regex::new(r"\s+").unwrap();

    let processed_prompt = multi_whitespace_regex.replace_all(prompt, " ").to_string();

    for word in word_list {
        let word_regex_str = format!(r"\b({})\b", word.as_str());
        let word_regex = Regex::new(word_regex_str.as_str()).unwrap();

        if word_regex.find(processed_prompt.as_str()).is_some() {
            return true;
        }
    }

    return false;
}

fn is_prompt_passed(
    prompt: &String,
    wanted_prompts: &Vec<String>,
    unwanted_prompts: &Vec<String>,
) -> bool {
    if check_prompt(prompt, unwanted_prompts) == true {
        return false;
    }

    if check_prompt(prompt, wanted_prompts) == true {
        return true;
    }

    return false;
}

#[derive(Debug, Clone)]
pub struct ImageData {
    pub prompt: String,
    pub image_url: String,
    pub base_model: String,
    pub nsfw_level: String,
    pub gen_type: String,
}

#[derive(Copy, Clone)]
pub enum ImageDataVals {
    PROMPT = 0,
    IMAGE_URL,
    BASE_MODEL,
    NSFW_LEVEL,
    GEN_TYPE,
    TOTAL,
}

impl ImageData {
    pub fn new() -> Self {
        Self {
            prompt: String::from("undefined"),
            image_url: String::from("undefined"),
            base_model: String::from("undefined"),
            nsfw_level: String::from("undefined"),
            gen_type: String::from("undefined"),
        }
    }
}

impl Index<ImageDataVals> for ImageData {
    type Output = String;

    fn index(&self, index: ImageDataVals) -> &Self::Output {
        match index {
            ImageDataVals::PROMPT => &self.prompt,
            ImageDataVals::IMAGE_URL => &self.image_url,
            ImageDataVals::BASE_MODEL => &self.base_model,
            ImageDataVals::NSFW_LEVEL => &self.nsfw_level,
            ImageDataVals::GEN_TYPE => &self.gen_type,
            ImageDataVals::TOTAL => panic!("do not use ImageDataVals::TOTAL"),
        }
    }
}

fn fill_value(data: &mut ImageData, item: &Value, field_name: &str) {
    let field_val = item.get(field_name);

    if field_val.is_none() {
        return;
    }

    let data_opt = field_val.unwrap().as_str();

    if data_opt.is_none() {
        return;
    }

    let data_value = String::from(data_opt.unwrap());

    match field_name {
        "url" => data.image_url = data_value,
        "baseModel" => data.base_model = data_value,
        "nsfwLevel" => data.nsfw_level = data_value,
        "type" => data.gen_type = data_value,
        _ => println!("Undefined image data variable"),
    }
}

fn get_image_data(item: &Value) -> ImageData {
    let mut image_data = ImageData::new();

    let meta_val = item.get("meta");

    if meta_val.is_none() {
        return image_data;
    }

    let meta_opt = meta_val.unwrap().as_object();

    if meta_opt.is_none() {
        return image_data;
    }

    let prompt_val = meta_opt.unwrap().get("prompt");

    if prompt_val.is_none() {
        return image_data;
    }

    image_data.prompt = String::from(prompt_val.unwrap().as_str().unwrap());

    fill_value(&mut image_data, item, "url");
    fill_value(&mut image_data, item, "baseModel");
    fill_value(&mut image_data, item, "nsfwLevel");
    fill_value(&mut image_data, item, "type");

    image_data
}

pub fn get_page_image_data(
    items: &Vec<Value>,
    wanted_prompts: &Vec<String>,
    unwanted_prompts: &Vec<String>,
) -> Vec<ImageData> {
    let mut page_image_data = Vec::with_capacity(items.len());

    for item in tqdm!(items.into_iter(), desc = "Processing items", position = 1) {
        let mut image_data = get_image_data(item);

        if image_data.prompt == "undefined" {
            continue;
        }

        if is_prompt_passed(&image_data.prompt, wanted_prompts, unwanted_prompts) == false {
            continue;
        }

        image_data.prompt = preprocess(image_data.prompt);

        page_image_data.push(image_data);
    }

    page_image_data
}

#[test]
fn test_check_prompt() {
    let prompt = String::from("masterpiece, car, woman, cat, dog, ((((bug:1.232))))");
    let wanted_prompts = vec![String::from("dog"), String::from("cat")];
    let unwanted_prompts = vec![String::from("bug")];

    assert_eq!(true, check_prompt(&prompt, &wanted_prompts));
    assert_eq!(true, check_prompt(&prompt, &unwanted_prompts));

    let prompt_2 = String::from("masterpiece, car, woman, cat, dog");

    assert_eq!(true, check_prompt(&prompt_2, &wanted_prompts));
    assert_eq!(false, check_prompt(&prompt_2, &unwanted_prompts));

    let prompt_3 = String::from("masterpiece");

    assert_eq!(false, check_prompt(&prompt_3, &wanted_prompts));
    assert_eq!(false, check_prompt(&prompt_3, &unwanted_prompts));

    let prompt_4 = String::from("masterpiece, car, woman, dogcat, catdog");

    assert_eq!(false, check_prompt(&prompt_4, &wanted_prompts));
    assert_eq!(false, check_prompt(&prompt_4, &unwanted_prompts));
}
