#[cfg(test)]
#[test]
fn base64_encode_text() {
    assert_eq!(
        crate::base::base_text::base64_encode_text("Wutong".to_string()),
        Ok("V3V0b25n".to_string())
    );
    assert_eq!(
        crate::base::base_text::base64_encode_text("".to_string()),
        Err("Invalid input".to_string())
    );
}
