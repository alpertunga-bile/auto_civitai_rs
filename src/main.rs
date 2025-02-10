mod civitai_image_struct;

use civitai_image_struct::CivitaiImagePage;
use reqwest::get;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = get("https://civitai.com/api/v1/images")
        .await?
        .text()
        .await?;

    let result: CivitaiImagePage = serde_json::from_str(&body)?;

    println!("{}", result.metadata.nextCursor.clone().unwrap());

    Ok(())
}
