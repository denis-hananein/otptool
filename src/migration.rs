use crate::otp::OtpParameters;

pub fn decode_from_link(link: &str) -> anyhow::Result<Vec<OtpParameters>> {
    const MIGRATION_SCHEMA: &str = "otpauth-migration";
    const MIGRATION_HOST: &str = "offline";
    const MIGRATION_QUERY_KEY: &str = "data";

    let parsed_link = url::Url::parse(link).unwrap();

    if parsed_link.scheme() != MIGRATION_SCHEMA {
        return Err(anyhow::anyhow!("schema is not correct"));
    }

    if !parsed_link.has_host() || parsed_link.host_str().unwrap() != MIGRATION_HOST {
        return Err(anyhow::anyhow!("host is not correct"));
    }

    let value = match parsed_link
        .query_pairs()
        .find(|(k, _)| return k == MIGRATION_QUERY_KEY)
    {
        Some((_, val)) => val,
        None => {
            return Err(anyhow::anyhow!("query data not found"));
        }
    };

    let fixed_value = value.replace(" ", "+");
    return OtpParameters::from_base64(fixed_value.as_bytes());
}

pub fn decode_from_image(image_path: std::path::PathBuf) -> anyhow::Result<Vec<OtpParameters>> {
    let img = image::open(image_path)?;

    let decoder = bardecoder::default_decoder();
    let results = decoder.decode(&img);

    if results.len() > 1 {
        return Err(anyhow::anyhow!("too much data to decode OTP data"));
    }

    let maybe_result = results.first().unwrap();
    let result = maybe_result.as_ref().unwrap();

    return decode_from_link(result);
}

mod tests {
    #[test]
    fn test_decode_link() {
        let link = "otpauth-migration://offline?data=CjEKCkhlbGxvId6tvu8SGEV4YW1wbGU6YWxpY2VAZ29vZ2xlLmNvbRoHRXhhbXBsZTAC";
        let res = super::decode_from_link(link);

        assert!(res.is_ok(), "decode_from_link failed: {}", res.unwrap_err());

        let params = res.unwrap();
        assert_eq!(params.len(), 1);

        let param = params.first().unwrap();
        assert_eq!(param.name, "Example:alice@google.com");
        assert_eq!(param.issuer, "Example");
        assert_eq!(param.secret, "JBSWY3DPEHPK3PXP")
    }
}
