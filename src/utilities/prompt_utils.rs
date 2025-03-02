pub mod image_data;
mod preprocess;

use image_data::ImageData;
use preprocess::preprocess;
use regex::Regex;
use serde_json::Value;
use tqdm::tqdm;

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

fn get_parent_from_item(
    item: &Value,
    parent_name: String,
) -> Option<&serde_json::Map<String, Value>> {
    let parent_val = item.get(parent_name);

    if parent_val.is_none() {
        return None;
    }

    let parent_obj = parent_val.unwrap().as_object();

    if parent_obj.is_none() {
        return None;
    }

    let parent = parent_obj.unwrap();

    return Some(parent);
}

fn get_image_data(item: &Value) -> ImageData {
    let mut image_data = ImageData::new();

    let meta_opt = get_parent_from_item(item, String::from("meta"));
    let stats_opt = get_parent_from_item(item, String::from("stats"));

    if meta_opt.is_none() || stats_opt.is_none() {
        return image_data;
    }

    let meta = meta_opt.unwrap();
    let stats = stats_opt.unwrap();

    image_data.meta.fill(meta);
    image_data.items.fill(item);
    image_data.stats.fill(stats);

    image_data
}

pub fn get_page_image_data(
    items: &Vec<Value>,
    wanted_prompts: &Vec<String>,
    unwanted_prompts: &Vec<String>,
) -> Vec<ImageData> {
    let mut page_image_data = Vec::with_capacity(items.len());

    for item in tqdm(items.into_iter()).desc(Some("Processing Items")) {
        let mut image_data = get_image_data(item);

        if image_data.meta.prompt == "undefined" {
            continue;
        }

        if is_prompt_passed(&image_data.meta.prompt, wanted_prompts, unwanted_prompts) == false {
            continue;
        }

        image_data.meta.prompt = preprocess(image_data.meta.prompt);

        page_image_data.push(image_data);
    }

    page_image_data
}

#[test]
fn test_check_prompt() {
    let mut prompt = String::from("masterpiece, car, woman, cat, dog, ((((bug:1.232))))");
    let wanted_prompts = vec![String::from("dog"), String::from("cat")];
    let unwanted_prompts = vec![String::from("bug")];

    assert_eq!(true, check_prompt(&prompt, &wanted_prompts));
    assert_eq!(true, check_prompt(&prompt, &unwanted_prompts));

    prompt = String::from("masterpiece, car, woman, cat, dog");

    assert_eq!(true, check_prompt(&prompt, &wanted_prompts));
    assert_eq!(false, check_prompt(&prompt, &unwanted_prompts));

    prompt = String::from("masterpiece");

    assert_eq!(false, check_prompt(&prompt, &wanted_prompts));
    assert_eq!(false, check_prompt(&prompt, &unwanted_prompts));

    prompt = String::from("masterpiece, car, woman, dogcat, catdog");

    assert_eq!(false, check_prompt(&prompt, &wanted_prompts));
    assert_eq!(false, check_prompt(&prompt, &unwanted_prompts));
}
