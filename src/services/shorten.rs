use crate::{
    models::ShortenResponse, repositories::url::UrlRepository, utils::generate_short_code,
};

pub async fn build_shorten_url(
    url: &String,
) -> Result<ShortenResponse, Box<dyn std::error::Error>> {
    let mut short_code = generate_short_code();
    let mut attempts = 0;

    let url_repository = UrlRepository::new();

    while attempts < 10 {
        match url_repository.short_code_exists(&short_code).await {
            Ok(exists) => {
                if !exists {
                    break;
                }
                short_code = generate_short_code();
                attempts += 1;
            }
            Err(_) => return Err("Failed to check if short code exists".into()),
        }
    }

    if attempts >= 10 {
        return Err("Failed to generate short code".into());
    }

    match url_repository.save_url(&short_code, &url).await {
        Ok(_) => Ok(ShortenResponse {
            short_code: short_code.clone(),
            short_url: format!("http://localhost:3000/{}", short_code),
            original_url: url.clone(),
        }),
        Err(_) => return Err("Failed to save URL".into()),
    }
}
