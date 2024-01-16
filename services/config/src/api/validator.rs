use validator::ValidationError;

pub fn validate_next_page(next_page: &str) -> Result<(), ValidationError> {
    base64_url::decode(next_page)
        .map_err(|_err| ValidationError::new("invalid token"))?;

    Ok(())
}
