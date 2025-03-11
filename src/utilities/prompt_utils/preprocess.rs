use regex::Regex;

fn fix_commas(string: String) -> String {
    let multiwhitespaces_regex = Regex::new(r"\s+").unwrap();
    let nonpromptcommas_regex = Regex::new(r"(,\s){2,}").unwrap();

    let mut modified_string = multiwhitespaces_regex
        .replace_all(string.as_str(), " ")
        .to_string();

    modified_string = nonpromptcommas_regex
        .replace_all(modified_string.as_str(), ", ")
        .to_string();

    modified_string
}

pub fn preprocess(prompt: String) -> String {
    let scalarweights_regex = Regex::new(r",\s*:[0-9]*\.?[0-9]+").unwrap();
    let emptyprompts_regex = Regex::new(r",\s+[()\[\]{}]+\s*,").unwrap();
    let danglingparantheses_regex = Regex::new(r"\B\s+|\s+\B").unwrap();

    let mut modified_prompt = prompt.replace("\n", ", ");

    modified_prompt = fix_commas(modified_prompt);

    modified_prompt = scalarweights_regex
        .replace_all(modified_prompt.as_str(), "")
        .to_string();

    modified_prompt = fix_commas(modified_prompt);

    modified_prompt = emptyprompts_regex
        .replace_all(modified_prompt.as_str(), ",")
        .to_string();

    modified_prompt = danglingparantheses_regex
        .replace_all(modified_prompt.as_str(), "")
        .to_string()
        .replace(",", ", ");

    modified_prompt
}

#[test]
fn test_fix_commas() {
    let prompt = String::from("                              ");

    assert_eq!(" ", fix_commas(prompt));
}
