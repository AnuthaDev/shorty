use rand::Rng;

const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const SHORT_CODE_LENGTH: usize = 6;

pub fn generate_short_code() -> String {
    let mut rng = rand::thread_rng();

    (0..SHORT_CODE_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

pub fn validate_url(url_str: &str) -> Result<String, String> {
    match url::Url::parse(url_str) {
        Ok(url) => {
            let scheme = url.scheme();
            if scheme != "http" && scheme != "https" {
                return Err("URL must use http or https scheme".to_string());
            }

            if url.host_str().is_none() {
                return Err("URL must have a valid host".to_string());
            }

            Ok(url.to_string())
        }
        Err(_) => Err("Invalid URL format".to_string()),
    }
}
