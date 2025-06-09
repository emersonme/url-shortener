use std::time::{SystemTime, UNIX_EPOCH};


const BASE62_CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

pub fn generate_short_code() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    
    to_base62(timestamp as u64)
}

fn to_base62(mut num: u64) -> String {
    if num == 0 {
        return "0".to_string();
    }

    let mut result = Vec::new();
    while num > 0 {
        result.push(BASE62_CHARS[(num % 62) as usize] as char);
        num /= 62;
    }

    result.reverse();
    let code: String = result.into_iter().collect();
    
    if code.len() < 6 {
        format!("{:0>6}", code)
    } else {
        code[..6].to_string()
    }
}

pub fn is_valid_url(url: &str) -> bool {
    url::Url::parse(url).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_short_code() {
        let code = generate_short_code();
        assert_eq!(code.len(), 6);
        
        for c in code.chars() {
            assert!(BASE62_CHARS.contains(&(c as u8)));
        }
    }

    #[test]
    fn test_is_valid_url() {
        assert!(is_valid_url("https://www.google.com"));
        assert!(is_valid_url("http://localhost:3000"));
        assert!(!is_valid_url("not-a-url"));
        assert!(!is_valid_url(""));
    }
} 