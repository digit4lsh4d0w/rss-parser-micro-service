use crate::config::{
    error::{ValidationError, ValidationResult},
    feed::{MAX_UPDATE_RETRIES, MIN_UPDATE_INTERVAL, MIN_UPDATE_RETRIES},
};

pub fn validate_url(value: &str) -> ValidationResult<()> {
    if value.trim().is_empty() {
        return Err(ValidationError::EmptyUrl);
    }

    reqwest::Url::parse(value).map_err(|_| ValidationError::InvalidUrlFormat)?;

    Ok(())
}

pub fn validate_feed_name(value: &str) -> ValidationResult<()> {
    if value.trim().is_empty() {
        return Err(ValidationError::FeedEmptyName);
    }

    Ok(())
}

pub fn validate_feed_update_interval(value: usize) -> ValidationResult<()> {
    if value < MIN_UPDATE_INTERVAL {
        return Err(ValidationError::FeedUpdateIntervalTooSmall(value));
    }

    Ok(())
}

pub fn validate_feed_update_retries(value: usize) -> ValidationResult<()> {
    if value < MIN_UPDATE_RETRIES {
        return Err(ValidationError::FeedUpdateRetriesTooSmall(value));
    }

    if value > MAX_UPDATE_RETRIES {
        return Err(ValidationError::FeedUpdateRetriesTooBig(value));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_url() {
        let result = validate_url("");
        assert_eq!(result, Err(ValidationError::EmptyUrl));
    }

    #[test]
    fn test_invalid_url() {
        let result = validate_url("not-a-url");
        assert_eq!(result, Err(ValidationError::InvalidUrlFormat));
    }

    #[test]
    fn test_valid_url() {
        let result = validate_url("http://localhsot");
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_empty_feed_name() {
        let result = validate_feed_name("");
        assert_eq!(result, Err(ValidationError::FeedEmptyName));
    }

    #[test]
    fn test_valid_feed_name() {
        let result = validate_feed_name("feed");
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_feed_too_small_update_interval() {
        const INVALID_UPDATE_INTERVAL: usize = 5;
        let result = validate_feed_update_interval(INVALID_UPDATE_INTERVAL);
        assert_eq!(
            result,
            Err(ValidationError::FeedUpdateIntervalTooSmall(
                INVALID_UPDATE_INTERVAL
            ))
        );
    }

    #[test]
    fn test_feed_valid_update_interval() {
        let result = validate_feed_update_interval(MIN_UPDATE_INTERVAL);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_feed_too_small_update_retries() {
        const INVALID_UPDATE_RETRIES: usize = 0;
        let result = validate_feed_update_retries(INVALID_UPDATE_RETRIES);
        assert_eq!(
            result,
            Err(ValidationError::FeedUpdateRetriesTooSmall(
                INVALID_UPDATE_RETRIES
            ))
        );
    }

    #[test]
    fn test_feed_too_big_update_retries() {
        const INVALID_UPDATE_RETRIES: usize = 20;
        let result = validate_feed_update_retries(INVALID_UPDATE_RETRIES);
        assert_eq!(
            result,
            Err(ValidationError::FeedUpdateRetriesTooBig(
                INVALID_UPDATE_RETRIES
            ))
        );
    }
}
