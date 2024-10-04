#[cfg(test)]
#[test]
fn binary() {
    let result = crate::base_conversion::math::binary("1001101001101100011100010");
    assert_eq!(
        result,
        [
            "115154342".to_string(),
            "20240610".to_string(),
            "134d8e2".to_string()
        ]
    );
}

#[test]
fn octal() {
    let result = crate::base_conversion::math::octal("115154342");
    assert_eq!(
        result,
        [
            "1001101001101100011100010".to_string(),
            20240610.to_string(),
            "134d8e2".to_string()
        ]
    );
}

#[test]
fn decimal() {
    let result = crate::base_conversion::math::decimal("20240610");
    assert_eq!(
        result,
        [
            "1001101001101100011100010".to_string(),
            "115154342".to_string(),
            "134d8e2".to_string()
        ]
    )
}

#[test]
fn hexadecimal() {
    let result = crate::base_conversion::math::hexadecimal("134d8e2");
    assert_eq!(
        result,
        [
            "1001101001101100011100010".to_string(),
            "115154342".to_string(),
            "20240610".to_string()
        ]
    )
}
