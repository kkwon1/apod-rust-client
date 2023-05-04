use regex::Regex;

pub fn is_valid(api_key: &str) -> bool {
    // The valid regex for a NASA API key consists of lower, upper case alphabet, and digits.
    // It must be exactly 40 characters long.
    let regex: Regex = Regex::new(r"^[a-zA-Z0-9]{40}$").unwrap();

    return regex.is_match(api_key);
}

#[cfg(test)]
mod tests {
    use crate::validators::api_key_validator::is_valid;

    #[test]
    fn valid_api_key() {
        let is_valid = is_valid("hrDwl56I9DKfPstNy9cqaTn0S68dTYpo4kB96dku");
        assert_eq!(is_valid, true);
    }

    #[test]
    fn invalid_api_key_too_short() {
        let is_valid = is_valid("hrDwl56I9DKfPstNy9cq");
        assert_eq!(is_valid, false);
    }

    #[test]
    fn invalid_api_key_too_long() {
        let is_valid = is_valid("hrDwl56I9DKfPstNy9cqaTn0S68dTYpo4kB96dkuHfo389sJWE");
        assert_eq!(is_valid, false);
    }

    #[test]
    fn invalid_api_key_special_characters() {
        let is_valid = is_valid("hrDwl56I9DKfPstNy9cqaTn%S68dTYpo4kB96dku");
        assert_eq!(is_valid, false);
    }
}
