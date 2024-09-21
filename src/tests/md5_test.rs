#[cfg(test)]
mod md5_test {
    #[test]
    fn test_md5_text() {
        let result = crate::md5::md5_text::md5_text("Wutong".to_string());
        assert_eq!(result, "1944e3ec8ad0d14d3d7e6506477b1478");
        assert_eq!(result.len(), 32);
    }
}
