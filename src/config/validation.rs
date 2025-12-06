use crate::config::{
    error::{ValidationError, ValidationResult},
    feed::{MAX_UPDATE_RETRIES, MIN_UPDATE_INTERVAL, MIN_UPDATE_RETRIES},
};

pub fn validate_feed_name(value: &str) -> ValidationResult<()> {
    if value.trim().is_empty() {
        return Err(ValidationError::EmptyName);
    }

    Ok(())
}

pub fn validate_url(value: &str) -> ValidationResult<()> {
    if value.trim().is_empty() {
        return Err(ValidationError::EmptyUrl);
    }

    reqwest::Url::parse(value).map_err(|_| ValidationError::InvalidUrlFormat)?;

    Ok(())
}

pub fn validate_update_interval(value: usize) -> ValidationResult<()> {
    if value < MIN_UPDATE_INTERVAL {
        return Err(ValidationError::UpdateIntervalTooSmall(value));
    }

    Ok(())
}

pub fn validate_update_retries(value: usize) -> ValidationResult<()> {
    if value < MIN_UPDATE_RETRIES {
        return Err(ValidationError::UpdateRetriesTooSmall(value));
    }

    if value > MAX_UPDATE_RETRIES {
        return Err(ValidationError::UpdateRetriesTooBig(value));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feed_empty_url() {
        let error = validate_url("").unwrap_err();
        assert_eq!(error, ValidationError::EmptyUrl);
    }

    #[test]
    fn test_feed_invalid_url() {
        let error = validate_url("not-a-url").unwrap_err();
        assert_eq!(error, ValidationError::InvalidUrlFormat);
    }

    #[test]
    fn test_feed_too_small_update_interval() {
        const INVALID_UPDATE_INTERVAL: usize = 5;
        let error = validate_update_interval(INVALID_UPDATE_INTERVAL).unwrap_err();
        assert_eq!(
            error,
            ValidationError::UpdateIntervalTooSmall(INVALID_UPDATE_INTERVAL)
        );
    }

    #[test]
    fn test_feed_too_small_update_retries() {
        const INVALID_UPDATE_RETRIES: usize = 0;
        let error = validate_update_retries(INVALID_UPDATE_RETRIES).unwrap_err();
        assert_eq!(
            error,
            ValidationError::UpdateRetriesTooSmall(INVALID_UPDATE_RETRIES)
        );
    }

    #[test]
    fn test_feed_too_big_update_retries() {
        const INVALID_UPDATE_RETRIES: usize = 20;
        let error = validate_update_retries(INVALID_UPDATE_RETRIES).unwrap_err();
        assert_eq!(
            error,
            ValidationError::UpdateRetriesTooBig(INVALID_UPDATE_RETRIES)
        );
    }
}
